#![no_std]
#![no_main]

mod constants;
use constants::*;

use defmt::*;
use embassy_executor::{Executor};
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{self, Async, Config};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::{I2C0};
use embassy_rp::gpio::{Input, Pull};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Timer, Instant};
use ssd1306::mode::DisplayConfig;
use ssd1306::prelude::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};
use static_cell::StaticCell;

use rotary_encoder_hal::{Direction, Rotary};

mod keymaps;
use keymaps::KEYMAPS;

mod usb_keyboard;
use crate::usb_keyboard::{UsbKeyboard, UsbKeyboardRequestHandler};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver};
use embassy_usb::UsbDevice;
use embassy_usb::class::hid::{HidReader, HidWriter};
use usbd_hid::descriptor::{KeyboardReport};
use usbd_hid::descriptor::KeyboardUsage as KU;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
};

use u8g2_fonts::{
    fonts,
    types::{FontColor, VerticalPosition},
    FontRenderer,
};

mod boards;
use boards::*;


use {defmt_rtt as _, panic_probe as _};

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
static CHANNEL: Channel<CriticalSectionRawMutex, DisplayMessage, 20> = Channel::new();
static USB_KEYBOARD_CONFIG: StaticCell<usb_keyboard::Config> = StaticCell::new();


enum DisplayMessage {
    ShowChars([char; 3]),
}

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
});


#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());
    let r = split_resources!(p);

    // Set up I2C0 for the SSD1306 OLED Display
    let i2c0 = i2c::I2c::new_async(r.screen.peri, r.screen.scl, r.screen.sda, Irqs, Config::default());
    // display task
    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| unwrap!(spawner.spawn(core1_task(i2c0))));
        },
    );

    debug!("Create USB keyboard device");
    let usb_driver = Driver::new(p.USB, Irqs);
    let usb_keyboard_config = USB_KEYBOARD_CONFIG.init(usb_keyboard::Config::new());
    let usb_keyboard = UsbKeyboard::new(usb_keyboard_config, usb_driver);

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| {
        spawner.spawn(usb_run(usb_keyboard.usb)).unwrap();
        spawner.spawn(hid_read(usb_keyboard.hid_reader, usb_keyboard.request_handler)).unwrap();
        spawner.spawn(core0_task(r.rotary_enc, r.buttons, usb_keyboard.hid_writer)).unwrap();
    });
}


#[embassy_executor::task]
async fn usb_run(mut usb: UsbDevice<'static, Driver<'static, USB>>) {
    debug!("Start 'USB Run' task");
    usb.run().await;
}

#[embassy_executor::task]
async fn hid_read(
    hid_reader: HidReader<'static, Driver<'static, USB>, 1>,
    request_handler: &'static mut UsbKeyboardRequestHandler,
) {
    debug!("Start 'HID Read' task");
    hid_reader.run(false, request_handler).await;
}


// Keyboard task

#[embassy_executor::task]
async fn core0_task(rotary_enc: RotaryEncResources, buttons: ButtonResources,
                    mut hid_writer: HidWriter<'static, Driver<'static, USB>, 8>) {
    debug!("Hello from core 0");

    let mut enc = Rotary::new(Input::new(rotary_enc.pin_a, Pull::Up), Input::new(rotary_enc.pin_b, Pull::Up));
    let navi_up = Input::new(buttons.navi_up, Pull::Up);
    let navi_down = Input::new(buttons.navi_down, Pull::Up);
    let lower_case = Input::new(buttons.lower_case, Pull::Up);
    let upper_case = Input::new(buttons.upper_case, Pull::Up);
    let space = Input::new(buttons.space, Pull::Up);
    let bcksp = Input::new(buttons.bcksp, Pull::Up);
    let ctrl = Input::new(buttons.ctrl, Pull::Up);
    let switch = Input::new(buttons.switch, Pull::Up);

    let mut enc_pos: i32 = 0;
    let mut navi_updown: i32 = 0;
    let mut navi_button_pressed: i32 = 0;
    let mut navi_repeat_timer = Instant::now();
    let mut keymap_n: usize = 0;
    let mut keymap_pos: usize = 0;
    let mut ks: &keymaps::KS = &KEYMAPS[keymap_n][keymap_pos];
    let mut updated = true;
    let mut button_released = true;
    let mut maybe_long_press = false;
    let mut long_press_start = Instant::now();
    let mut special_ks = KU::KeyboardErrorUndefined;
    let mut with_ctrl = false;
    let mut with_alt = false;

    loop {
        let mut keymap_delta: i32 = 0;

        match enc.update().unwrap() {
            Direction::Clockwise => {
                enc_pos += 1;
                if enc_pos >= ROTARY_SCALE_FACTOR {
                    enc_pos = 0;
                    keymap_delta = 1;
                    updated = true;
                }
            }
            Direction::CounterClockwise => {
                enc_pos -= 1;
                if enc_pos <= ROTARY_SCALE_FACTOR * -1 {
                    enc_pos = 0;
                    keymap_delta = -1;
                    updated = true;
                }
            }
            Direction::None => {
            }
        }

        if navi_updown != 0 {
            keymap_delta = navi_updown;
            navi_updown = 0;
            updated = true;
        }

        if updated {
            if keymap_delta > 0 {
                keymap_pos += keymap_delta as usize;
                if keymap_pos >= 40 || KEYMAPS[keymap_n][keymap_pos].c == KU::KeyboardErrorUndefined {
                    keymap_pos = 0;
                }
            }
            else if keymap_delta < 0 {
                if keymap_pos == 0 {
                    keymap_pos = 39;
                    while KEYMAPS[keymap_n][keymap_pos].c == KU::KeyboardErrorUndefined {
                        keymap_pos -= 1;
                    }
                }
                else {
                    keymap_pos -= (keymap_delta * -1) as usize;
                }
            }

            ks = &KEYMAPS[keymap_n][keymap_pos];
            if ks.c == KU::KeyboardErrorRollOver {
                let ksc = &keymaps::COMPLEX_KEYMAPS[ks.s[0] as usize];
                CHANNEL.send(DisplayMessage::ShowChars(ksc.display_str)).await;
            }
            else {
                CHANNEL.send(DisplayMessage::ShowChars(ks.s)).await;
            }
            updated = false;
        }

        let mut button_down = false;
        let mut long_press_button_down = false;
        let mut send_letter = false;
        let mut with_shift = false;
        let mut special_ks_button_down = false;
        let mut send_special_ks = false;
        let mut switch_pressed = false;
        let mut ctrl_button_down = false;

        if navi_up.is_low() {
            long_press_button_down = true;
            navi_button_pressed = 1;
            button_down = true;
        }
        if navi_down.is_low() {
            long_press_button_down = true;
            navi_button_pressed = -1;
            button_down = true;
        }
        if lower_case.is_low() {
            send_letter = true;
            button_down = true;
        }
        else if upper_case.is_low() {
            send_letter = true;
            with_shift = true;
            button_down = true;
        }
        else if space.is_low() {
            special_ks = KU::KeyboardSpacebar;
            button_down = true;
            long_press_button_down = true;
        }
        else if bcksp.is_low() {
            special_ks = KU::KeyboardBackspace;
            special_ks_button_down = true;
            button_down = true;
        }
        else if ctrl.is_low() {
            long_press_button_down = true;
            ctrl_button_down = true;
            button_down = true;
        }
        else if switch.is_low() {
            switch_pressed = true;
            button_down = true;
        }

        let mut send_report = false;

        if button_released {
            if button_down {
                button_released = false;
                if long_press_button_down {
                    maybe_long_press = true;
                    long_press_start = Instant::now();
                    if ctrl_button_down {
                        debug!("ctrl_button_down");
                        with_ctrl = true;
                    }
                }
                else if switch_pressed {
                    keymap_n += 1;
                    if keymap_n >= KEYMAPS.len() {
                        keymap_n = 0;
                    }

                    // if switching between keymaps landed on the null key, move back to the last symbol
                    if KEYMAPS[keymap_n][keymap_pos].c == KU::KeyboardErrorUndefined {
                        while KEYMAPS[keymap_n][keymap_pos].c == KU::KeyboardErrorUndefined {
                            keymap_pos -= 1;
                        }
                    }

                    if keymaps::KEYMAP_PRELUDES[keymap_n].0 {
                        let mut report = KeyboardReport {
                            leds: 0,
                            modifier: keymaps::KEYMAP_PRELUDES[keymap_n].1,
                            reserved: 0,
                            keycodes: [keymaps::KEYMAP_PRELUDES[keymap_n].2 as u8, 0, 0, 0, 0, 0],
                        };
                        send_report_to_writer(&mut hid_writer, &report).await;
                        report = KeyboardReport {
                            leds: 0,
                            modifier: 0,
                            reserved: 0,
                            keycodes: [0, 0, 0, 0, 0, 0],
                        };
                        send_report_to_writer(&mut hid_writer, &report).await;
                    }

                    CHANNEL.send(DisplayMessage::ShowChars(keymaps::KEYMAP_TITLES[keymap_n])).await;
                    Timer::after_millis(SWITCH_LAYER_TITLE_DISPLAY_MS).await;
                    updated = true;
                }
                else if special_ks_button_down {
                    send_report = true;
                    send_special_ks = true;
                }
                else {
                    send_report = true;
                }
            }
        }
        else {
            if button_down {
                if maybe_long_press {
                    if long_press_start.elapsed().as_millis() >= LONG_PRESS_INTERVAL_MS {
                        if special_ks == KU::KeyboardSpacebar { // long press on spacebar produces enter
                            send_report = true;
                            send_special_ks = true;
                            special_ks = KU::KeyboardEnter;
                            maybe_long_press = false;
                        }
                        else if with_ctrl {
                            with_ctrl = false;
                            with_alt = true;
                            send_letter = true;
                            send_report = true;
                        }
                        else if navi_button_pressed != 0 {
                            if navi_repeat_timer.elapsed().as_millis() >= NAVI_REPEAT_PERIOD_MS {
                                navi_repeat_timer = Instant::now();
                                navi_updown = navi_button_pressed;
                            }
                        }
                    }
                }
            }
            else {
                button_released = true;
                if maybe_long_press {
                    if special_ks == KU::KeyboardSpacebar {
                        send_special_ks = true;
                        send_report = true;
                    }
                    else if with_ctrl {
                        send_letter = true;
                        send_report = true;
                    }
                    else if navi_button_pressed != 0 {
                        navi_updown = navi_button_pressed;
                        navi_button_pressed = 0;
                    }
                    maybe_long_press = false;
                }
            }
        }

        if send_report {
            debug!("send_report");
            if send_letter {
                if ks.c == KU::KeyboardErrorRollOver {
                    let ksc = &keymaps::COMPLEX_KEYMAPS[ks.s[0] as usize];
                    for key in ksc.keycodes {
                        if key.1[0] != KU::KeyboardErrorUndefined {
                            let mut report = KeyboardReport {
                                leds: 0,
                                modifier: key.0,
                                reserved: 0,
                                keycodes: [0, 0, 0, 0, 0, 0],
                            };
                            if with_shift {
                                report.modifier |= 0b0000_0010;
                                debug!("shift");
                            }
                            if with_ctrl {
                                report.modifier |= 0b0000_0001;
                                with_ctrl = false;
                                debug!("ctrl");
                            }
                            if with_alt {
                                report.modifier |= 0b0000_0100;
                                with_alt = false;
                                debug!("alt");
                            }
                            let mut pos = 0;
                            for keycode in key.1 {
                                if keycode != KU::KeyboardErrorUndefined {
                                    report.keycodes[pos] = keycode as u8;
                                }
                                pos += 1;
                            }
                            send_report_to_writer(&mut hid_writer, &report).await;
                        }
                    }
                }
                else {
                    let mut report = KeyboardReport {
                        leds: 0,
                        modifier: 0,
                        reserved: 0,
                        keycodes: [0, 0, 0, 0, 0, 0],
                    };
                    report.keycodes[0] = ks.c as u8;
                    if with_shift {
                        report.modifier = 0b0000_0010;
                        debug!("shift");
                    }
                    if with_ctrl {
                        report.modifier |= 0b0000_0001;
                        with_ctrl = false;
                        debug!("ctrl");
                    }
                    if with_alt {
                        report.modifier |= 0b0000_0100;
                        with_alt = false;
                        debug!("alt");
                    }
                    send_report_to_writer(&mut hid_writer, &report).await;
                }
            }
            else if send_special_ks {
                let report = KeyboardReport {
                    leds: 0,
                    modifier: 0,
                    reserved: 0,
                    keycodes: [special_ks as u8, 0, 0, 0, 0, 0],
                };
                send_report_to_writer(&mut hid_writer, &report).await;
                special_ks = KU::KeyboardErrorUndefined;
            }

            // send Key Release report
            let report = KeyboardReport {
                leds: 0,
                modifier: 0,
                reserved: 0,
                keycodes: [0, 0, 0, 0, 0, 0],
            };
            send_report_to_writer(&mut hid_writer, &report).await;
        }

        Timer::after_millis(1).await;
    }
}


async fn send_report_to_writer(hid_writer: &mut HidWriter<'static, Driver<'static, USB>, 8>,
               report: &KeyboardReport) {
    match hid_writer.write_serialize(report).await {
        Ok(()) => {}
        Err(e) => warn!("Failed to send report: {:?}", e),
    }
}



// Displaying task
const CHARWIDTH: i32 = 33;

#[embassy_executor::task]
async fn core1_task(i2c0: embassy_rp::i2c::I2c<'static, I2C0, Async>) {
    debug!("Hello from core 1");

    let interface = I2CDisplayInterface::new(i2c0);
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_buffered_graphics_mode();
    display.init().unwrap();
    display.clear(BinaryColor::Off).unwrap();
    display.flush().unwrap();

    // Width: 33, Height: 64
    let font = FontRenderer::new::<fonts::u8g2_font_inr42_t_cyrillic>();

    loop {
        let _ = CHANNEL.ready_to_receive().await;
        while CHANNEL.len() > 1 {
            let _ = CHANNEL.receive().await;
        }

        match CHANNEL.receive().await {
            DisplayMessage::ShowChars(cc) => {
                let mut stringlen = 0;
                for c in cc {
                    if c as u16 != 0 {
                        stringlen += 1;
                    }
                }
                display.clear(BinaryColor::Off).unwrap();
                let mut pos = 0;
                for c in cc {
                    if c as u16 != 0 {
                        font.render(
                            c,
                            Point::new(64-CHARWIDTH*stringlen/2 + pos*CHARWIDTH, 0),
                            VerticalPosition::Top,
                            FontColor::Transparent(BinaryColor::On),
                            &mut display,
                        ).unwrap();
                        pos += 1;
                    }
                }
                display.flush().unwrap();
            }
        }
    }
}
