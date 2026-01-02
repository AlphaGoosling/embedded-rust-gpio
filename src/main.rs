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

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"GPIO on off program"),
    embassy_rp::binary_info::rp_program_description!(c"This program toggles the state of pin two of a rp pico high and low"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];