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
pub static SERVO_ANGLE_B: AtomicI32 = AtomicI32::new(90);

//  #[embassy_executor::task]
//  pub async fn servo_test(
//     mut pwm_pin: esp_hal::mcpwm::operator::PwmPin<'static, esp_hal::peripherals::MCPWM0, 0, true>,  mut pwm_pin_b: esp_hal::mcpwm::operator::PwmPin<'static, esp_hal::peripherals::MCPWM0, 1, true>, ) {
    


//         // loop {
//         //     // 0 degree (2.5% of 20_000 => 500)
//         //     pwm_pin.set_timestamp(500);
//         //     Timer::after(Duration::from_millis(1500)).await;
//         //     // 90 degree (7.5% of 20_000 => 1500)
//         //     pwm_pin.set_timestamp(1500);
//         //     Timer::after(Duration::from_millis(1500)).await;
    
//         //     // 180 degree (12.5% of 20_000 => 2500)
//         //     pwm_pin.set_timestamp(2500);
//         //     Timer::after(Duration::from_millis(1500)).await;
//         // }
// loop{
//         let angle = SERVO_ANGLE.load(Ordering::Relaxed);
//         let angle_b: i32 = SERVO_ANGLE_B.load(Ordering::Relaxed);

//         // let clamped_angle_b: u16 = angle.clamp(0, 180) as u16;
//         // let clamped_angle: u16 = angle_b.clamp(0, 180) as u16;


//         let clamped_angle: u16 = angle.clamp(0, 180) as u16; 
//         let clamped_angle_b: u16 = angle_b.clamp(0, 180) as u16;


//         let pulse_width: u16 = angle_to_pulse(clamped_angle);
//         let pulse_width_b: u16 = angle_to_pulse(clamped_angle_b);
//         pwm_pin.set_timestamp(pulse_width);
//         pwm_pin_b.set_timestamp(pulse_width_b);

//         Timer::after(Duration::from_millis(20)).await;
// }
// }
#[embassy_executor::task]
pub async fn servo_test(
    mut pwm_pin: esp_hal::mcpwm::operator::PwmPin<'static, esp_hal::peripherals::MCPWM0, 0, true>,
    mut pwm_pin_b: esp_hal::mcpwm::operator::PwmPin<'static, esp_hal::peripherals::MCPWM0, 1, true>
) {
    info!("Dual servo tracking system started");
    
    loop {
        let angle = SERVO_ANGLE.load(Ordering::Relaxed);
        let angle_b = SERVO_ANGLE_B.load(Ordering::Relaxed);

        // FIXED - correct variable assignments
        let clamped_angle: u16 = angle_b.clamp(0, 180) as u16;      // Servo A (elevation)
        let clamped_angle_b: u16 = angle.clamp(0, 180) as u16; // Servo B (azmiuth) 
        
        let pulse_width: u16 = angle_to_pulse(clamped_angle);
        let pulse_width_b: u16 = angle_to_pulse(clamped_angle_b);
        
        pwm_pin.set_timestamp(pulse_width);    // Azimuth servo
        pwm_pin_b.set_timestamp(pulse_width_b); // Elevation servo

        Timer::after(Duration::from_millis(20)).await;
    }
}

pub fn angle_to_pulse(angle: u16) -> u16 {
    let min = 500;
    let max = 2500;
    min + ((angle as u32 * (max - min) as u32) / 180) as u16
}

pub fn gps_to_azimuth_elevation(
    observer: &GpsCoord,  
    target: &GpsCoord    
) -> AzEl {
    let lat1_rad = observer.latitude * core::f64::consts::PI / 180.0;
    let lat2_rad = target.latitude * core::f64::consts::PI / 180.0;
    let lon1_rad = observer.longitude * core::f64::consts::PI / 180.0;
    let lon2_rad = target.longitude * core::f64::consts::PI / 180.0;
    let dlon = lon2_rad - lon1_rad;
    let y = libm::sin(dlon) * libm::cos(lat2_rad);
    let x = libm::cos(lat1_rad) * libm::sin(lat2_rad) - 
            libm::sin(lat1_rad) * libm::cos(lat2_rad) * libm::cos(dlon);
    let mut azimuth = libm::atan2(y, x) * 180.0 / core::f64::consts::PI;


    if azimuth < 0.0 {
        azimuth += 360.0; 
    }
    
    const EARTH_RADIUS: f64 = 6371000.0; 
    let dlat = lat2_rad - lat1_rad;
    let a = libm::sin(dlat/2.0) * libm::sin(dlat/2.0) + 
            libm::cos(lat1_rad) * libm::cos(lat2_rad) * 
            libm::sin(dlon/2.0) * libm::sin(dlon/2.0);
    let c = 2.0 * libm::atan2(libm::sqrt(a), libm::sqrt(1.0-a));
    let horizontal_distance = EARTH_RADIUS * c;
    let altitude_diff = target.altitude - observer.altitude;
    let elevation = libm::atan2(altitude_diff, horizontal_distance) * 180.0 / core::f64::consts::PI;
    
    AzEl { azimuth, elevation }
}

pub fn azimuth_elevation_to_servo_angles(azel: &AzEl) -> (i32, i32) {
    let servo_a_angle = (azel.azimuth / 2.0).clamp(0.0, 180.0) as i32;
    let servo_b_angle = (azel.elevation + 90.0).clamp(0.0, 180.0) as i32;
    
    (servo_a_angle, servo_b_angle)
}
// pub fn angle_to_pulse(angle: u16) -> u16 {
//     let min = 500;
//     let max = 2500;
//     min + ((angle as u32 * (max - min) as u32) / 180) as u16
// }


#[derive(Clone, Copy)]
pub struct GpsCoord {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

pub struct AzEl {
    pub azimuth: f64, 
    pub elevation: f64,
}