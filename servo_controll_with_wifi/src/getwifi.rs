use core::sync::atomic::{AtomicBool, Ordering};
use embassy_time::{Duration, Timer};
use heapless::{String, Vec};

pub static SUCCESS : AtomicBool = AtomicBool::new(false);

#[embassy_executor::task]
pub async fn get_data(ssid: &'static str, password: &'static str) {
    
}