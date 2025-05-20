use std::{iter::ArrayChunks, str::from_utf8};
#[allow(unused_imports)]
use std::{default, sync::{Arc, Mutex}, thread::sleep, time::Duration};

use anyhow::Ok;


use embedded_dht_rs::dht11::Dht11;
use embedded_svc::timer;
#[allow(unused_imports)]

use embedded_svc::{http::{self, Method}};
use esp_idf_svc::{http::Method::Post, wifi::{AuthMethod, ClientConfiguration, Configuration, PmfConfiguration, ScanMethod}};
use esp_idf_hal::{gpio::PinDriver, io::{Read, Write}, ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution}, peripheral::Peripheral, prelude::Peripherals};

use esp_idf_svc::{eventloop::EspSystemEventLoop, http::server::EspHttpServer, nvs::{EspDefaultNvsPartition, EspNvsPartition, NvsDefault}, ping::EspPing, timer::{EspTaskTimerService, EspTimerService, Task}, wifi::{AsyncWifi, EspWifi}};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use heapless::String;
use esp_idf_sys::esp_get_free_heap_size; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::units::*;



// const SSID: &str = env!("RUST_ESP32_STD_DEMO_WIFI_SSID");
// const PASS: &str = env!("RUST_ESP32_STD_DEMO_WIFI_PASS");

const SSID: &str = "NONO";
const PASS: &str = "NONO@";
fn main() {
 
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // info!("Hello, world!");

    // log::info!("Free heap: {} bytes", unsafe { esp_get_free_heap_size() });
    let mut attempts = 0;

    let peripherals = Peripherals::take().unwrap();
  
    let sysloop = EspSystemEventLoop::take().unwrap();
    let timer_service = EspTaskTimerService::new().unwrap();
    let mut async_wifi = wifi(
      peripherals.modem,
      sysloop,
      Some(EspDefaultNvsPartition::take().unwrap()),
      timer_service
  ).unwrap();
  
  connect_wifi_with_retry(&mut async_wifi, None, Duration::from_secs(15)); // retry forever every 15 secs
  
    // let mut server: EspHttpServer<'_> = EspHttpServer::new(&Default::default()).unwrap();

    // server.fn_handler("/", esp_idf_svc::http::Method::Get, move |req| {
    //     let mut response = req.into_ok_response().unwrap();
    //     response.write("Hello from Esp32-c3".as_bytes()).unwrap();
    //     Ok::<(), anyhow::Error>(())
    // }).unwrap();
// Use the correct configuration type for HTTP server
let config = esp_idf_svc::http::server::Configuration {
  stack_size: 8192,  // For newer versions
  ..Default::default()
};


let mut server = EspHttpServer::new(&config).unwrap();
  let  led_pin = Arc::new(Mutex::new( PinDriver::output(peripherals.pins.gpio2).unwrap()));

  let  other_led = Arc::new(Mutex::new( PinDriver::output(peripherals.pins.gpio17).unwrap()));
  let pin_1 =  Arc::new(Mutex::new( PinDriver::output(peripherals.pins.gpio16).unwrap()));
  let  pin_1_high = Arc::clone(&pin_1);
  let  pin_1_low = Arc::clone(&pin_1);
let pin_2 =  Arc::new(Mutex::new( PinDriver::output(peripherals.pins.gpio18).unwrap()));
let pin_2_on = Arc::clone(&pin_2);
let pin_2_off = Arc::clone(&pin_2);
let pin_2_stop = Arc::clone(&pin_2);


let servo_timer = peripherals.ledc.timer1;
let servo_driver = LedcTimerDriver::new(servo_timer, &TimerConfig::default().frequency(50.Hz()).resolution(esp_idf_hal::ledc::Resolution::Bits14)).unwrap();

let servo = Arc::new(Mutex::new(LedcDriver::new(peripherals.ledc.channel3, servo_driver, peripherals.pins.gpio4).unwrap()));


let max_duty = servo.lock().unwrap().get_max_duty();

let min = (max_duty / 40) as i32;
let max = (max_duty / 8) as i32;



fn interpolate(angle: i32, min: i32, max: i32) -> u32 {
  // Map -180 to +180 range to 0 to 180 range
  let mut mapped_angle = (angle + 180) % 360;
  if mapped_angle < 0 {
      mapped_angle += 360;
  }
  let adjusted_angle = mapped_angle / 2;
  
  // Then calculate duty cycle
  (adjusted_angle * (max - min) / 180 + min) as u32
}

let  sensor_out = Arc::new(Mutex::new(PinDriver::input_output(peripherals.pins.gpio25).unwrap()));
let led_timer: esp_idf_hal::ledc::TIMER0 = peripherals.ledc.timer0;
let led_timer_driver = LedcTimerDriver::new(
  led_timer, 
  &TimerConfig::default()
      .frequency(3000.Hz())
      .resolution(Resolution::Bits8),// Add a resolution
).unwrap();

let red_channel = Arc::new(Mutex::new(
  LedcDriver::new(peripherals.ledc.channel0, &led_timer_driver, peripherals.pins.gpio27).unwrap()
));

let green_channel = Arc::new(Mutex::new(
  LedcDriver::new(peripherals.ledc.channel1, &led_timer_driver, peripherals.pins.gpio14).unwrap()
));

let blue_channel = Arc::new(Mutex::new(
  LedcDriver::new(peripherals.ledc.channel2, &led_timer_driver, peripherals.pins.gpio26).unwrap()
));

// let rgbshit: Arc<Mutex<PinDriver<'_, esp_idf_hal::gpio::Gpio17, esp_idf_hal::gpio::Output>>>  = Arc::new(Mutex::new( PinDriver::output(peripherals.ledc.channel0,led_timer,gpio17).unwrap()));

#[derive(Debug)]
struct Color{
  red:u8,
  blue:u8,
  green:u8
}

impl TryFrom<&str>for Color{
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
      Ok(Color{
        red : u8::from_str_radix(&input[0..2], 16)?,
        blue: u8::from_str_radix(&input[2..4], 16)?,
        green : u8::from_str_radix(&input[4..6], 16)?
      })
    }
}


server.fn_handler("/sex", esp_idf_svc::http::Method::Get, move |req| {
    let mut response = req.into_ok_response().unwrap();
    response.write("Hello from Esp32 Balazs".as_bytes()).unwrap();
    led_pin.lock().unwrap().toggle().unwrap();
    Ok(())
  }).unwrap();
  server.fn_handler("/", esp_idf_svc::http::Method::Get, move |req| {
    let mut response = req.into_ok_response().unwrap();
    response.write("Hello from Esp32 Balazs".as_bytes()).unwrap();
    other_led.lock().unwrap().toggle().unwrap();
    Ok(())
  }).unwrap();
  server.fn_handler("/pin1_high", esp_idf_svc::http::Method::Get, move |req| {
    let mut response = req.into_ok_response().unwrap();
    response.write("Hello from Esp32 Balazs".as_bytes()).unwrap();
    pin_1_high.lock().unwrap().set_high().unwrap();
    Ok(())
  }).unwrap();
  server.fn_handler("/pin1_low", esp_idf_svc::http::Method::Get, move |req| {
    let mut response = req.into_ok_response().unwrap();
    response.write("Hello from Esp32 Balazs".as_bytes()).unwrap();
    pin_1_low.lock().unwrap().set_low().unwrap();
    Ok(())
  }).unwrap();





  server.fn_handler("/servo", Post, move |mut req| {
    // let mut response = req.into_ok_response().unwrap();
    // response.write("Hello from Esp32 Balazs".as_bytes()).unwrap();
    let mut buffer = [0_u8; 6];
  let byte_read =     req.read(&mut buffer).unwrap();

  let angle_string = from_utf8(&buffer[0..byte_read]).unwrap();
    let angle: i32 = angle_string.parse().unwrap();
  
    servo.lock().unwrap().set_duty(interpolate(angle,min,max)as u32).unwrap();
    Ok(())
  }).unwrap();



  server.fn_handler("/pin2_start", esp_idf_svc::http::Method::Get, move |req| {
    let mut response = req.into_ok_response().unwrap();
    response.write("Hello from Esp32 Balazs".as_bytes()).unwrap();
    pin_2_on.lock().unwrap().set_high().unwrap();
    let  mut i: i32 = 0;
    while i < 5 {
        pin_2_stop.lock().unwrap().set_low().unwrap();
        sleep(Duration::from_millis(500));
        pin_2_on.lock().unwrap().set_high().unwrap();
        sleep(Duration::from_millis(500));
        i += 1;
    }

    Ok(())
  }).unwrap();
  server.fn_handler("/pin2_stop", esp_idf_svc::http::Method::Get, move |req| {
    let mut response = req.into_ok_response().unwrap();
    response.write("Hello from Esp32 Balazs".as_bytes()).unwrap();


    pin_2_off.lock().unwrap().set_low().unwrap();
  

    Ok(())
  }).unwrap();

  server.fn_handler("/hosting", esp_idf_svc::http::Method::Get, move |req| {
    let mut response = req.into_ok_response().unwrap();


    response.write_all(b"<html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Document</title><link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.6/dist/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-4Q6Gf2aSP4eDXB8Miphtr37CMZZQ5oXLH2yaXMJ2w8e2ZtHTl7GptT4jmndRuHDT\" crossorigin=\"anonymous\"></head><body><h1 class=\"text-center bg-danger\">ERROR WHILE RETRIVING DATA</h1><script src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.6/dist/js/bootstrap.bundle.min.js\" integrity=\"sha384-j1CDi7MgGQ12Z7Qab0qlWQ/Qqz24Gc6BM0thvEMVjHnfYGF0rmFCozFSxQBxwHKO\" crossorigin=\"anonymous\"></script></body></html>").unwrap();
  

    Ok(())
  }).unwrap();

  server.fn_handler::<anyhow::Error, _>("/math", Post, move | mut req| {
    let mut buffer = [0_u8; 6];
    let byte_read =     req.read(&mut buffer).unwrap();

    info!("{byte_read}");
  //  let b = req.read(&mut [0_u8;1000]).unwrap();
   let angle_string = from_utf8(&buffer[0..byte_read]).unwrap();

    println!("{angle_string}");
    let mut left : Vec<char> = Vec::new();
    let mut right : Vec<char> = Vec::new();
    let mut operator : Vec<&str> = Vec::new();

let _firstpart = true;
for char in angle_string.chars(){
 if _firstpart {

  if   char.is_ascii_digit() == true{
    left.push(char);
  }
  else{
    operator.push(char.to_string().as_str()); // finish it tomorow
  }

 }

}

    Ok(())
  }).unwrap();




  server.fn_handler("/rs", esp_idf_svc::http::Method::Get, move |req| {
    sleep(Duration::from_millis(1500));

    let mut response = req.into_ok_response().unwrap();
    
    let result: Option<(u8, i8)> = {
      let mut pin = sensor_out.lock().unwrap();
      
      pin.set_low().unwrap();
      sleep(Duration::from_millis(18));
      pin.set_high().unwrap();
      sleep(Duration::from_micros(40));
      
      let mut data = [0u8; 5];
      
      let mut timeout = 0;
      while pin.is_high() && timeout < 100 {
        timeout += 1;
        sleep(Duration::from_micros(1));
      }
      
      timeout = 0;
      while pin.is_low() && timeout < 100 {
        timeout += 1;
        sleep(Duration::from_micros(1));
      }
      
      for i in 0..40 {
        timeout = 0;
        while pin.is_high() && timeout < 100 {
          timeout += 1;
          sleep(Duration::from_micros(1));
        }
        
        timeout = 0;
        while pin.is_low() && timeout < 100 {
          timeout += 1;
          sleep(Duration::from_micros(1));
        }
        
        sleep(Duration::from_micros(30));
        if pin.is_high(){
          data[i / 8] |= 1 << (7 - (i % 8));
        }
      }
      
      let sum = (data[0] as u16 + data[1] as u16 + data[2] as u16 + data[3] as u16) & 0xFF;
      
      if data[4] == sum as u8 {
        let humidity = data[0];
        let temperature = data[2] as i8;
        Some((humidity, temperature))
      } else {
        None
      }
    };
    
    match result {
      Some((humidity, temperature)) => {
        let message = format!("Temperature: {} °C, Humidity: {}%", temperature, humidity);
        // info!("{}", message);
        response.write(message.as_bytes()).unwrap();
      },
      None => {
        response.write_all(b"<html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><title>Document</title><link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.6/dist/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-4Q6Gf2aSP4eDXB8Miphtr37CMZZQ5oXLH2yaXMJ2w8e2ZtHTl7GptT4jmndRuHDT\" crossorigin=\"anonymous\"></head><body><h1 class=\"text-center bg-danger\">ERROR WHILE RETRIVING DATA</h1><script src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.6/dist/js/bootstrap.bundle.min.js\" integrity=\"sha384-j1CDi7MgGQ12Z7Qab0qlWQ/Qqz24Gc6BM0thvEMVjHnfYGF0rmFCozFSxQBxwHKO\" crossorigin=\"anonymous\"></script></body></html>").unwrap();

            }
    }
    
    Ok(())
  }).unwrap();
  let ir_sensor = PinDriver::input(peripherals.pins.gpio19).unwrap();

//   let mut previous_state = false;
// let mut state_counter = 0;
// const STATE_THRESHOLD: i32 = 3;
// const SAMPLES: usize = 10; 


loop {
  // let mut detections = 0;
  
  // for _ in 0..SAMPLES {
  //     if ir_sensor.is_low() {
  //         detections += 1;
  //     }
  //     sleep(Duration::from_millis(10));
  // }
  
  // let detection_ratio = detections as f32 / SAMPLES as f32;

  // let current_reading = if previous_state {
  //     detection_ratio > 0.3 
  // } else {
  //     detection_ratio > 0.6 
  // };
  
  // // Count consecutive consistent readings
  // if current_reading == previous_state {
  //     state_counter = 0;
  // } else {
  //     state_counter += 1;
      
  //     if state_counter >= STATE_THRESHOLD {
  //         previous_state = current_reading;
  //         state_counter = 0;
          
  //         if previous_state {
  //             info!("IR Sensor: Object detected!");
  //         } else {
  //             info!("IR Sensor: No object");
  //         }
  //     }
  // }
  
  sleep(Duration::from_millis(50));
}
}
pub fn connect_wifi_with_retry(
  wifi: &mut AsyncWifi<EspWifi<'static>>,
  max_attempts: Option<usize>,
  retry_delay: Duration,
) {
  use futures::executor::block_on;
  use std::thread::sleep;

  let mut attempts = 0;

  loop {
      let result = block_on(connect_wifi(wifi));

      if result.is_ok() {
          // log::info!(
          //     "Connected to Wi-Fi successfully after {} attempt(s).",
          // );
          attempts + 1;

          break;
      } else {
          attempts += 1;
          sleep(Duration::from_secs(60));

          // log::error!(
          //     "Wi-Fi attempt {} failed: {:?}",

          //     attempts,
          //     result.unwrap_err()
          // );

          if let Some(max) = max_attempts {
              if attempts >= max {
                  // log::error!("Wi-Fi failed after {} attempts. Giving up.", max);
                  break;
              }
          }

          
          // log::warn!(
          //     "Retrying Wi-Fi connection in {} seconds...",
          // );
          // retry_delay.as_secs();

          sleep(retry_delay);
      }
  }
}


pub fn wifi(
    modem: impl Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
    nvs: Option<EspNvsPartition<NvsDefault>>,
    timer_service: EspTimerService<Task>,
) -> anyhow::Result<AsyncWifi<EspWifi<'static>>> {
    use futures::executor::block_on;
    let mut wifi = AsyncWifi::wrap(
      
        EspWifi::new(modem, sysloop.clone(), nvs)?,
        sysloop,
        timer_service.clone(),
    )?;

    let mut attempts = 0;
    loop {
        if let Err(e) = block_on(connect_wifi(&mut wifi)) {
            // log::error!("Wi-Fi attempt {} failed: {:?}", attempts + 1, e);
            attempts += 1;
            sleep(Duration::from_secs(6));

            if attempts >= 5 {
                // log::error!("Wi-Fi failed after 5 attempts. Giving up.");
                break;
            }
            sleep(Duration::from_secs(5));
        } else {
            break; // Success
        }
    }
    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    // println!("Wifi DHCP info: {:?}", ip_info);
    
    EspPing::default().ping(ip_info.subnet.gateway, &esp_idf_svc::ping::Configuration::default())?;
    Ok(wifi)
}

async fn connect_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    // Convert string literals to heapless strings
    let mut ssid = String::<32>::new();
    ssid.push_str(SSID).unwrap();
    
    let mut password = String::<64>::new();
    password.push_str(PASS).unwrap();
    
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: ssid,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: password,
        channel: None,
        scan_method: ScanMethod::FastScan,
        pmf_cfg: PmfConfiguration::Capable { required: false },
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start().await?;
    // info!("Wifi started");

    wifi.connect().await?;
    // info!("Wifi connected");

    wifi.wait_netif_up().await?;
    // info!("Wifi netif up");

    Ok(())
}
