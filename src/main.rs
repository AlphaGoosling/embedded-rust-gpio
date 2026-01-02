#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Output, Level};
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut led = Output::new(peripherals.PIN_2, Level::Low);

    loop {
    led.set_high();
    Timer::after_millis(20).await;

    led.set_low();
    Timer::after_millis(20).await;
}

}

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
