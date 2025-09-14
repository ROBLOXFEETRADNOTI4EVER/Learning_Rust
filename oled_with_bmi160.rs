#![no_std]
#![no_main]

use defmt::{info, println};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::mono_font::ascii::{FONT_6X12, FONT_7X13};
use embedded_graphics::mono_font::ascii::*;

use embedded_graphics::mono_font::iso_8859_16::FONT_6X10;
use embedded_graphics::mono_font::{ MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::text::{Baseline, Text};
use esp_hal::gpio::{Input, InputConfig, Level};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{clock::CpuClock, time::Rate};
use esp_println as _;
use ssd1306::mode::DisplayConfigAsync;
use bmi160_esp32_minimal::{Bmi160, Bmi160Error};
use ssd1306::{prelude::DisplayRotation, size::DisplaySize128x64, I2CDisplayInterface, Ssd1306Async};
use itoa::Buffer;
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Starting OLED demo...");

    // I2C setup for OLED
    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO18)  
    .with_sda(peripherals.GPIO23)
    .into_async();

    let interface = I2CDisplayInterface::new(i2c_bus);


    let imu_i2c_config = Config::default()
    .with_frequency(Rate::from_khz(100));

    let imu_i2c = I2c::new(peripherals.I2C1,imu_i2c_config)
    .unwrap()
    .with_sda(peripherals.GPIO27)
    .with_scl(peripherals.GPIO26);
let mut bmi160: Bmi160 = Bmi160::new(imu_i2c, 0x69);
info!("BMI160 created, initializing...");

    let mut display: Ssd1306Async<ssd1306::prelude::I2CInterface<esp_hal::i2c::master::I2c<'_, esp_hal::Async>>, DisplaySize128x64, ssd1306::mode::BufferedGraphicsModeAsync<DisplaySize128x64>> = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    
    info!("Initializing display...");
    display.init().await.unwrap();
    display.clear(BinaryColor::Off).unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    let mut buffer = Buffer::new();
    let mut bob: i32 = 1;   

    
    // Draw text
    Text::with_baseline("Hello , Rust! ", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

     // Text::with_baseline(s, Point::new(16, 22), text_style, Baseline::Top)
        // .draw(&mut display)
        // .unwrap();
 let btn = Input::new(peripherals.GPIO13,  InputConfig::default().with_pull(esp_hal::gpio::Pull::Up));
    display.flush().await.unwrap();
    info!("Display updated!");
    _spawner.must_spawn(draw_shit(display,0,1,btn,bmi160));
 

//     loop {


  

//         // Timer::after(Duration::from_secs(1)).await;
//     //     display.clear(BinaryColor::On).unwrap();
//     //     Timer::after(Duration::from_millis(10)).await;
//     //     display.clear(BinaryColor::Off).unwrap();
//     //     Text::with_baseline("Hello , Rust! ", Point::new(0, 16), text_style, Baseline::Top)
//     //     .draw(&mut display)
//     //     .unwrap();
//     // let s: &str = buffer.format(bob);
//     //     Text::with_baseline(s, Point::new(18, 24), text_style, Baseline::Top)
//     //     .draw(&mut display)
//     //     .unwrap();
// //     Line::new(Point::new(50, bob), Point::new(bob, 35))
// //     .translate(Point::new(-30, 10))
// //     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
// //     .draw(&mut display)
// //     .unwrap();   
// // Line::new(Point::new(bob, 20), Point::new(60, bob))
// // .translate(Point::new(-30, 10))
// // .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
// // .draw(&mut display)
// // .unwrap();   
// // for mut i in 1..90{
// //     Line::new(Point::new(bob, 20 + (i * 2) - 5), Point::new(60 - (i * 2) , bob))
// //     .translate(Point::new(10, 20))
// //     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
// //     .draw(&mut display)
// //     .unwrap(); 
// // i += 1;
// }

        // if bob < 140 {
        //     bob += 1;
        // }else{
        //     bob = 0;
        // }
        // display.flush().await.unwrap();
       
        // println!("1 secs passed")
    }


#[embassy_executor::task]
async  fn draw_shit(mut display: Ssd1306Async<ssd1306::prelude::I2CInterface<esp_hal::i2c::master::I2c<'static, esp_hal::Async>>, DisplaySize128x64, ssd1306::mode::BufferedGraphicsModeAsync<DisplaySize128x64>> , x:i32,y:i32, mut button : Input<'static> , mut bmi160: Bmi160){
   let mut bananine = 2;
   let mut going_up: bool = true;
   let mut chimpanzine: i32 =  2;

   match bmi160.init().await {
    Ok(()) => {
        info!("BMI160 initialized successfully!");
    },
    Err(Bmi160Error::InvalidChipId(id)) => {
        info!("Wrong chip ID: 0x{:02X} (expected 0xD1)", id);
        info!("Check wiring and I2C address (try 0x68 instead of 0x69)");
        return;
    },
    Err(e) => {
        info!("BMI160 initialization failed: ", );
        info!("Check wiring: SDA=GPIO27, SCL=GPIO26, VCC=3.3V, GND=GND");
        return;
    }
}


    for _ in 1..135{

        if going_up {
            bananine += 2;
            if bananine >= 50 {
                going_up = false;
            }
        } else {
            bananine -= 2;
            if bananine <= 0 {
                going_up = true;
            }
        }
        if going_up {
            chimpanzine += 2;
            if chimpanzine >= 200 {
                going_up = false;
            }
        } else {
            chimpanzine -= 2;
            if chimpanzine <= 0 {
                going_up = true;
            }
        }
        Line::new(Point::new(30, bananine), Point::new(30, bananine + 10))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
            .translate(Point::new(x, y))
            .draw(&mut display)
            .unwrap();
        
            Line::new(Point::new(30, bananine), Point::new(30, bananine + 10))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
            .translate(Point::new(x - 30 , y))
            .draw(&mut display)
            .unwrap();

    let text_style = MonoTextStyleBuilder::new()
    .font(&FONT_7X13)
    .text_color(BinaryColor::Off)
    .build();
        Text::with_baseline(" Welcome to Cansat", Point::new(1, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

        Line::new(Point::new(30, bananine), Point::new(30, bananine + 10))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
        .translate(Point::new(x + 30 , y))
        .draw(&mut display)
        .unwrap();
    Line::new(Point::new(30, bananine), Point::new(30, bananine + 10))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
    .translate(Point::new(x + 30 * 2 , y))
    .draw(&mut display)
    .unwrap();  Line::new(Point::new(30, bananine), Point::new(30, bananine + 10))
.into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
.translate(Point::new(x + 30 * 3 , y))
.draw(&mut display)
.unwrap();
Line::new(Point::new(chimpanzine, 30), Point::new(chimpanzine + 10, 20 + 10))
.into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
.translate(Point::new(chimpanzine  , chimpanzine + 50  ))
.draw(&mut display)
.unwrap();
Line::new(Point::new(chimpanzine, 30), Point::new(chimpanzine + 10, 20 + 10))
.into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
.translate(Point::new(x  , y - 30))
.draw(&mut display)
.unwrap();
Line::new(Point::new(chimpanzine, 30), Point::new(chimpanzine + 10, 20 + 10))
.into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
.translate(Point::new(x  , y + 15 * 2))
.draw(&mut display)
.unwrap();
Line::new(Point::new(chimpanzine, 30), Point::new(chimpanzine + 10, 20 + 10))
.into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 1))
.translate(Point::new(x  , y + 15 * 3))
.draw(&mut display)
.unwrap();

        display.flush().await.unwrap();
        Timer::after(Duration::from_nanos(2)).await;
        display.clear(BinaryColor::On).unwrap();

    }
    Timer::after(Duration::from_nanos(200)).await;

    display.clear(BinaryColor::On).unwrap();
    // display.clear(BinaryColor::Off).unwrap();
    display.clear(BinaryColor::Off).unwrap();
    display.flush().await.unwrap();
    let mut buffer = Buffer::new();
    let mut imu_buffer_x = Buffer::new();
    let mut imu_buffer_y = Buffer::new();
    let mut imu_buffer_z = Buffer::new();

    let mut button_pressed_count: i16 = 0;
    let mut last_state = button.is_high();
    Timer::after(Duration::from_nanos(20000)).await;

    for _ in 1..{
        let current_state = button.is_high();

    if current_state && !last_state {
//    if button.is_low() {
       button_pressed_count += 1;
       Timer::after(Duration::from_millis(10)).await;

       println!("PRessed");
   let text_style = MonoTextStyleBuilder::new()
   .font(&FONT_9X18_BOLD)
   .text_color(BinaryColor::Off)
   .build();
display.clear(BinaryColor::On).unwrap();
   let s: &str = buffer.format(button_pressed_count);
        Text::with_baseline(s, Point::new(18, 24), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    Timer::after(Duration::from_nanos(20000)).await;

} 
display.flush().await.unwrap();
last_state = current_state;

println!("run");


match bmi160.read_all() {
    Ok(data) => {
        
        info!("Raw - Accel: X={}, Y={}, Z={}", 
            data.accel.x, data.accel.y, data.accel.z);
        info!("Raw - Gyro: X={}, Y={}, Z={}", 
            data.gyro.x, data.gyro.y, data.gyro.z);
            let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_7X13)
            .text_color(BinaryColor::Off)
            .build();
        // let (ax, ay, az) = data.accel.to_g();
        // let (gx, gy, gz) = data.gyro.to_dps();
        let imu_data_x: &str = imu_buffer_x.format(data.accel.x ); //  + data.gyro.y + data.gyro.z)
        Text::with_baseline(imu_data_x, Point::new(0, 0), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    let imu_data_y: &str = imu_buffer_y.format(data.accel.y ); //  + data.gyro.y + data.gyro.z)
    Text::with_baseline(imu_data_y, Point::new(8, 4), text_style, Baseline::Top)
    .draw(&mut display)
    .unwrap();
let imu_data_z: &str = imu_buffer_z.format(data.accel.z ); //  + data.gyro.y + data.gyro.z)
Text::with_baseline(imu_data_z, Point::new(16, 8), text_style, Baseline::Top)
.draw(&mut display)
.unwrap();
    
    Timer::after(Duration::from_nanos(20000)).await;

        // info!("Physical - Accel: {}g, {}g, {}g", ax, ay, az);
        // info!("Physical - Gyro: {}°/s, {}°/s, {}°/s", gx, gy, gz);
    }
    Err(e) => {
        info!("Read error: ");
    }
}


}
}



