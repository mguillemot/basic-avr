#![no_std]
#![no_main]

use avr_device::atmega328p::Peripherals;
use avr_hal_generic::delay::Delay;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;

use panic_halt as _;

type Clock = atmega_hal::clock::MHz8;

#[avr_device::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);
    // let mut delay = Delay::<Clock>::new();

    let buttons = [
        pins.pc0.into_pull_up_input().downgrade(),
        pins.pc1.into_pull_up_input().downgrade(),
        pins.pc2.into_pull_up_input().downgrade(),
        pins.pc3.into_pull_up_input().downgrade(),
        pins.pc4.into_pull_up_input().downgrade(),
        pins.pc5.into_pull_up_input().downgrade(),
        pins.pc6.into_pull_up_input().downgrade(),
        pins.pd0.into_pull_up_input().downgrade(),
    ];
    let mut leds = [
        pins.pb0.into_output().downgrade(),
        pins.pb1.into_output().downgrade(),
        pins.pb2.into_output().downgrade(),
        pins.pb3.into_output().downgrade(),
        pins.pb4.into_output().downgrade(),
        pins.pb5.into_output().downgrade(),
        pins.pb6.into_output().downgrade(),
        pins.pb7.into_output().downgrade(),
    ];
    let mut current_button_pressed = [false; 8];
    let mut current_output = [false; 8];

    // loop {
    //     for idx in 0..8 {
    //         if idx % 2 == 0 {
    //             leds[idx].set_low();
    //         } else {
    //             leds[idx].set_high();
    //         }
    //     }
    //     delay.delay_ms(500u16);
    //     for idx in 0..8 {
    //         if idx % 2 == 1 {
    //             leds[idx].set_low();
    //         } else {
    //             leds[idx].set_high();
    //         }
    //     }
    //     delay.delay_ms(500u16);
    // }

    loop {
        for idx in 0..8 {
            let button_pressed = buttons[idx].is_low();
            if button_pressed != current_button_pressed[idx] {
                if button_pressed {
                    // just pressed
                    current_output[idx] = !current_output[idx];
                }
                current_button_pressed[idx] = button_pressed;
            }
            if current_output[idx] {
                leds[idx].set_low();
            } else {
                leds[idx].set_high();
            }
        }
    }
}
