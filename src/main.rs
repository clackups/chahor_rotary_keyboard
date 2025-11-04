// This example shows how to use a SSD1306 OLED display via I2C to display text
// GPIO4/5 used for this (I2C0 default pins)
//
// Core 0 does measurements and communicates via a CHANNEL to Core 1
// Core 1 does display/LED I/O

#![no_std]
#![no_main]


use defmt::*;
use embassy_executor::{Executor};
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{self, Async, Config};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::{I2C0};
use embassy_rp::gpio::{Input, Pull};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Timer};
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


use {defmt_rtt as _, panic_probe as _};

static mut CORE1_STACK: Stack<4096> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
static CHANNEL: Channel<CriticalSectionRawMutex, DisplayMessage, 20> = Channel::new();
static USB_KEYBOARD_CONFIG: StaticCell<usb_keyboard::Config> = StaticCell::new();


enum DisplayMessage {
    ShowChar(char)
}

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
});

struct Pins {
    rotary_pin_a: Input<'static>,
    rotary_pin_b: Input<'static>,
    lower_case: Input<'static>,
    upper_case: Input<'static>,
    space: Input<'static>,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());

    // Set up I2C0 for the SSD1306 OLED Display
    let i2c0 = i2c::I2c::new_async(p.I2C0, p.PIN_9, p.PIN_8, Irqs, Config::default());
    // display task
    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| unwrap!(spawner.spawn(core1_task(i2c0))));
        },
    );

    info!("Create USB keyboard device");
    let usb_driver = Driver::new(p.USB, Irqs);
    let usb_keyboard_config = USB_KEYBOARD_CONFIG.init(usb_keyboard::Config::new());
    let usb_keyboard = UsbKeyboard::new(usb_keyboard_config, usb_driver);

    let pins: Pins = Pins {
        rotary_pin_a: Input::new(p.PIN_21, Pull::Up),
        rotary_pin_b: Input::new(p.PIN_20, Pull::Up),
        lower_case:   Input::new(p.PIN_2, Pull::Up),
        upper_case:   Input::new(p.PIN_3, Pull::Up),
        space:        Input::new(p.PIN_4, Pull::Up),
    };

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| {
        spawner.spawn(usb_run(usb_keyboard.usb)).unwrap();
        spawner.spawn(hid_read(usb_keyboard.hid_reader, usb_keyboard.request_handler)).unwrap();
        spawner.spawn(core0_task(pins, usb_keyboard.hid_writer)).unwrap();
    });
}


#[embassy_executor::task]
async fn usb_run(mut usb: UsbDevice<'static, Driver<'static, USB>>) {
    info!("Start 'USB Run' task");
    usb.run().await;
}

#[embassy_executor::task]
async fn hid_read(
    hid_reader: HidReader<'static, Driver<'static, USB>, 1>,
    request_handler: &'static mut UsbKeyboardRequestHandler,
) {
    info!("Start 'HID Read' task");
    hid_reader.run(false, request_handler).await;
}


// Keyboard task

#[embassy_executor::task]
async fn core0_task(pins: Pins,
                    mut hid_writer: HidWriter<'static, Driver<'static, USB>, 8>) {
    info!("Hello from core 0");

    let mut enc = Rotary::new(pins.rotary_pin_a, pins.rotary_pin_b);
    let keymap_n: usize = 1;
    let mut pos: usize = 0;
    let mut ks: &keymaps::KS = &KEYMAPS[keymap_n][pos];
    let mut updated = true;
    let mut button_released = true;

    loop {
        match enc.update().unwrap() {
            Direction::Clockwise => {
                info!("Clockwise");
                pos += 1;
                if KEYMAPS[keymap_n][pos].c == KU::KeyboardErrorUndefined || pos >= 40 {
                    pos = 0;
                }
                updated = true;
            }
            Direction::CounterClockwise => {
                info!("CounterClockwise");
                if pos == 0 {
                    pos = 39;
                    while KEYMAPS[keymap_n][pos].c == KU::KeyboardErrorUndefined {
                        pos -= 1;
                    }
                }
                else {
                    pos -= 1;
                }
                updated = true;
            }
            Direction::None => {
            }
        }

        if updated {
            ks = &KEYMAPS[keymap_n][pos];
            CHANNEL.send(DisplayMessage::ShowChar(ks.s)).await;
            updated = false;
        }

        let mut report = KeyboardReport {
            leds: 0,
            modifier: 0,
            reserved: 0,
            keycodes: [0, 0, 0, 0, 0, 0],
        };

        let mut button_down = false;

        if pins.lower_case.is_low() {
            report.keycodes[0] = ks.c as u8;
            button_down = true;
        }
        else if pins.upper_case.is_low() {
            report.keycodes[0] = ks.c as u8;
            report.modifier = 0b0000_0010;
            button_down = true;
        }
        else if pins.space.is_low() {
            report.keycodes[0] = KU::KeyboardSpacebar as u8;
            button_down = true;
        }

        let mut send_report = false;

        if button_released {
            if button_down {
                send_report = true;
                button_released = false;
            }
        }
        else {
            if !button_down {
                button_released = true;
            }
        }

        if send_report {
            match hid_writer.write_serialize(&report).await {
                Ok(()) => {}
                Err(e) => warn!("Failed to send report: {:?}", e),
            }

            // send Key Release report
            report = KeyboardReport {
                leds: 0,
                modifier: 0,
                reserved: 0,
                keycodes: [0, 0, 0, 0, 0, 0],
            };
            match hid_writer.write_serialize(&report).await {
                Ok(()) => {}
                Err(e) => warn!("Failed to send report: {:?}", e),
            }
        }

        Timer::after_millis(1).await;
    }
}



// Displaying task

#[embassy_executor::task]
async fn core1_task(i2c0: embassy_rp::i2c::I2c<'static, I2C0, Async>) {
    info!("Hello from core 1");

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
            DisplayMessage::ShowChar(c) => {
                // info!("c={}", c);
                display.clear(BinaryColor::Off).unwrap();
                font.render(
                    c,
                    Point::new(64-33/2, 0),
                    VerticalPosition::Top,
                    FontColor::Transparent(BinaryColor::On),
                    &mut display,
                ).unwrap();

                display.flush().unwrap();
            }
        }
    }
}
