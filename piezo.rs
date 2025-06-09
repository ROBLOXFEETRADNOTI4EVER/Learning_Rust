use core::sync::atomic::{AtomicBool, Ordering};

use embassy_time::{Duration, Timer};
use esp_hal::gpio::Output;
pub static PIEZO_STATE: AtomicBool = AtomicBool::new(false);
#[embassy_executor::task]
pub async fn piezo_task(mut piezo: Output<'static>) {
    loop {
        if PIEZO_STATE.load(Ordering::Relaxed) {
            piezo.set_high();
            Timer::after(Duration::from_millis(500)).await;
            piezo.set_low();
        } else {
            piezo.set_low();
        }
        Timer::after(Duration::from_millis(50)).await;
    }
}
