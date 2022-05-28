#![no_std]
#![no_main]

use avr_device::atmega328p::Peripherals;
use avr_hal_generic::delay::Delay;

use panic_halt as _;

type Clock = atmega_hal::clock::MHz8;

#[avr_device::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);
    let mut _delay = Delay::<Clock>::new();

    let buttons = [
        pins.pc0.into_pull_up_input().downgrade(),
        pins.pc1.into_pull_up_input().downgrade(),
        pins.pc2.into_pull_up_input().downgrade(),
        pins.pc3.into_pull_up_input().downgrade(),
    ];
    let mut leds = [
        pins.pb0.into_output().downgrade(),
        pins.pb1.into_output().downgrade(),
        pins.pb2.into_output().downgrade(),
        pins.pb3.into_output().downgrade(),
    ];
    let mut mosfets = [
        pins.pd0.into_output().downgrade(),
        pins.pd1.into_output().downgrade(),
        pins.pd2.into_output().downgrade(),
    ];
    let mut buttons_pressed = [false; 4];
    let mut selected_output = 0;

    loop {
        // Button presses can change the selected output
        for idx in 0..4 {
            let button_pressed = buttons[idx].is_low();
            if button_pressed && !buttons_pressed[idx] {
                selected_output = idx;
            }
            buttons_pressed[idx] = button_pressed;
        }

        // Reflect selected output on LEDs
        for idx in 0..4 {
            if selected_output == idx {
                leds[idx].set_low();
            } else {
                leds[idx].set_high();
            }
        }

        // Reflect selected output on MOSFETs
        match selected_output {
            0 => {
                mosfets[0].set_low();
                mosfets[1].set_low();
                mosfets[2].set_low();
            }
            1 => {
                mosfets[0].set_high();
                mosfets[1].set_low();
                mosfets[2].set_low();
            }
            2 => {
                mosfets[0].set_low();
                mosfets[1].set_high();
                mosfets[2].set_low();
            }
            3 => {
                mosfets[0].set_low();
                mosfets[1].set_high();
                mosfets[2].set_high();
            }
            _other => unreachable!(),
        }
    }
}
