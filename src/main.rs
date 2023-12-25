
#![no_std]
#![no_main]

extern crate panic_semihosting;
use hal::pac;
use xmc4_hal as hal;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {

    let _p = pac::Peripherals::take().unwrap();
    loop {}
}
