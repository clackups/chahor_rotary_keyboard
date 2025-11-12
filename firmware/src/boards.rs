//#[cfg(all(feature = "chahor_v1"))]
//compile_error!("You must not enable more than one board type in Cargo features.");
#[cfg(not(any(feature = "chahor_v1")))]
compile_error!("You must enable one board type in Cargo features.");

use assign_resources::assign_resources;
use embassy_rp::Peri;
use embassy_rp::peripherals;

#[cfg(feature = "chahor_v1")]
assign_resources! {
    screen: ScreenResources {
        peri: I2C0,
        scl: PIN_9,
        sda: PIN_8,
    }
    rotary_enc: RotaryEncResources {
        pin_a: PIN_21,
        pin_b: PIN_20,
    }
    buttons: ButtonResources {
        navi_up:      PIN_18,
        navi_down:    PIN_19,
        lower_case:   PIN_2,
        upper_case:   PIN_3,
        space:        PIN_4,
        bcksp:        PIN_5,
        ctrl:         PIN_6,
        switch:       PIN_7,
    }
}
