use core::any;

use defmt::info;
use embassy_executor::Spawner;
use esp_hal::mcpwm::{McPwm, PeripheralClockConfig};
use esp_hal::peripherals::LEDC;
use esp_hal::time::Rate;
use esp_hal::{clock::CpuClock, peripheral};
use esp_hal::rng::Rng;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
use esp_hal::system::software_reset;
use esp_hal::gpio::{self, Input, InputConfig, Level, Output, OutputConfig, Pull};

use esp_hal::ledc::channel::ChannelIFace;
use esp_hal::ledc::timer::TimerIFace;
use esp_hal::ledc::{channel, timer, HighSpeed, Ledc};
use embedded_hal::pwm::SetDutyCycle;


use embassy_time::{self, Duration, Timer}; 
extern crate alloc;
use esp_storage::FlashStorage;
use embedded_storage::{ReadStorage, Storage};
use esp_wifi::EspWifiController;
use core::sync::atomic::{AtomicI32, Ordering};


pub static SERVO_ANGLE: AtomicI32 = AtomicI32::new(90);

 #[embassy_executor::task]
 pub async fn servo_test(
    mut pwm_pin: esp_hal::mcpwm::operator::PwmPin<'static, esp_hal::peripherals::MCPWM0, 0, true>) {
    


        // loop {
        //     // 0 degree (2.5% of 20_000 => 500)
        //     pwm_pin.set_timestamp(500);
        //     Timer::after(Duration::from_millis(1500)).await;
        //     // 90 degree (7.5% of 20_000 => 1500)
        //     pwm_pin.set_timestamp(1500);
        //     Timer::after(Duration::from_millis(1500)).await;
    
        //     // 180 degree (12.5% of 20_000 => 2500)
        //     pwm_pin.set_timestamp(2500);
        //     Timer::after(Duration::from_millis(1500)).await;
        // }
loop{
        let angle = SERVO_ANGLE.load(Ordering::Relaxed);
        let clamped_angle = angle.clamp(0, 180) as u16;
        let pulse_width = angle_to_pulse(clamped_angle);
        pwm_pin.set_timestamp(pulse_width);
        Timer::after(Duration::from_millis(20)).await;
}
}

pub fn angle_to_pulse(angle: u16) -> u16 {
    let min = 500;
    let max = 2500;
    min + ((angle as u32 * (max - min) as u32) / 180) as u16
}
