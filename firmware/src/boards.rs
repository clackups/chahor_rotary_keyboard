//#[cfg(all(feature = "chahor_v1"))]
//compile_error!("You must not enable more than one board type in Cargo features.");
#[cfg(not(any(feature = "chahor_v1")))]
compile_error!("You must enable one board type in Cargo features.");

use assign_resources::assign_resources;
use embassy_rp::Peri;
use embassy_rp::peripherals;


#[cfg(feature = "chahor_v1")]
pub const USB_PRODUCT: &str = "chahor_v1";
pub const USB_SERIAL: &str = "0001";
// applied for a PID at https://pid.codes
pub const USB_VID: u16 = 0x1209;
pub const USB_PID: u16 = 0xBBD0;

assign_resources! {
    screen: ScreenResources {
        peri: I2C0,
        scl: PIN_9,
        sda: PIN_8,
    }
    rotary_enc: RotaryEncResources {
        pin_a: PIN_13,
        pin_b: PIN_12,
    }
    buttons: ButtonResources {
        lower_case:   PIN_2,
        upper_case:   PIN_3,
        space:        PIN_4,
        bcksp:        PIN_5,
        ctrl:         PIN_6,
        switch:       PIN_7,
        navi_up:      PIN_10,
        navi_down:    PIN_11,
    }
}


