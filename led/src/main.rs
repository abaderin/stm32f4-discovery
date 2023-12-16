#![no_std]
#![no_main]
#[warn(unused_imports)]

use stm32f4::stm32f429 as stm32;
use cortex_m_rt::entry;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {}
}

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let rcc = dp.RCC;

    rcc.ahb1enr.write(|w| w.gpiogen().set_bit().gpioaen().set_bit());


    let ltdc = dp.LTDC;



    let gpiog = dp.GPIOG;

    gpiog.moder.write(|w| w.moder13().bits(0b01).moder14().bits(0b01));
    gpiog.odr.write(|w| w.odr13().set_bit().odr14().clear_bit());

    let gpioa = dp.GPIOA;

    gpioa.moder.write(|w| w.moder0().bits(0b00));
    gpioa.pupdr.write(|w| unsafe { w.pupdr0().bits(0b10) });


    loop {
        if gpioa.idr.read().idr0().is_high() {
            gpiog.odr.write(|w| w.odr13().set_bit().odr14().clear_bit());
        } else {
            gpiog.odr.write(|w| w.odr13().clear_bit().odr14().set_bit());
        }
    }
}


