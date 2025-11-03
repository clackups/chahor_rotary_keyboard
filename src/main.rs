// This example shows how to use a SSD1306 OLED display via I2C to display text
// GPIO4/5 used for this (I2C0 default pins)
//
// Core 0 does measurements and communicates via a CHANNEL to Core 1
// Core 1 does display/LED I/O

#![no_std]
#![no_main]


use defmt::*;
use embassy_executor::Executor;
use embassy_rp::bind_interrupts;
use embassy_rp::i2c::{self, Async, Config, InterruptHandler};
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

mod symbols;

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


enum DisplayMessage {
    ShowChar(char)
}

bind_interrupts!(struct Irqs {
    I2C0_IRQ => InterruptHandler<I2C0>;
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

    let pins: Pins = Pins {
        rotary_pin_a: Input::new(p.PIN_21, Pull::Up),
        rotary_pin_b: Input::new(p.PIN_20, Pull::Up),
        lower_case:   Input::new(p.PIN_2, Pull::Up),
        upper_case:   Input::new(p.PIN_3, Pull::Up),
        space:        Input::new(p.PIN_4, Pull::Up),
    };

    let executor0 = EXECUTOR0.init(Executor::new());

    executor0.run(|spawner| unwrap!(spawner.spawn(core0_task(pins))));
}

// Keyboard task

#[embassy_executor::task]
async fn core0_task(pins: Pins) {
    // info!("Hello from core 0");

    let mut enc = Rotary::new(pins.rotary_pin_a, pins.rotary_pin_b);
    let keymap_n: usize = 1;
    let mut pos: usize = 0;
    let mut updated = true;

    loop {
        match enc.update().unwrap() {
            Direction::Clockwise => {
                pos += 1;
                if symbols::KEYMAPS[keymap_n][pos].c == 0 || pos >= 40 {
                    pos = 0;
                }
                updated = true;
            }
            Direction::CounterClockwise => {
                if pos == 0 {
                    pos = 39;
                    while symbols::KEYMAPS[keymap_n][pos].c == 0 {
                        pos -= 1;
                    }
                }
                else {
                    pos -= 1;
                }
                updated = true;
            }
            Direction::None => {}
        }

        if updated {
            let ks: &symbols::KS = &symbols::KEYMAPS[keymap_n][pos];
            CHANNEL.send(DisplayMessage::ShowChar(ks.s)).await;
            updated = false;
        }

        if pins.lower_case.is_low() {
        }
        else if pins.upper_case.is_low() {
        }
        else if pins.space.is_low() {
        }

        Timer::after_millis(1).await;
    }
}

// Displaying task

#[embassy_executor::task]
async fn core1_task(i2c0: embassy_rp::i2c::I2c<'static, I2C0, Async>) {
    // info!("Hello from core 1");

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
