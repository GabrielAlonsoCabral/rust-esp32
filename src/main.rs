use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    esp_idf_sys::link_patches();
    let peripherals: Peripherals = Peripherals::take().unwrap();
    let mut led: PinDriver<Gpio2, Output> = PinDriver::output(peripherals.pins.gpio2).unwrap();

    loop {
        led.toggle().unwrap();
        FreeRtos::delay_ms(1000);
    }
}
