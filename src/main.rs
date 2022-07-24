#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::Peripherals;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.led_rx.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(500);
    }
}
