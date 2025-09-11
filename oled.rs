#![no_std]
#![no_main]

use defmt::{info, println};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_graphics::text::{Baseline, Text};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{clock::CpuClock, time::Rate};
use esp_println as _;
use ssd1306::mode::DisplayConfigAsync;
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

    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
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
 
    display.flush().await.unwrap();
    info!("Display updated!");

    loop {


  

        // Timer::after(Duration::from_secs(1)).await;
        display.clear(BinaryColor::On).unwrap();
        Timer::after(Duration::from_millis(10)).await;
        display.clear(BinaryColor::Off).unwrap();
        Text::with_baseline("Hello , Rust! ", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    let s: &str = buffer.format(bob);
        Text::with_baseline(s, Point::new(18, 24), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    Line::new(Point::new(50, bob), Point::new(bob, 35))
    .translate(Point::new(-30, 10))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .draw(&mut display)
    .unwrap();   
Line::new(Point::new(bob, 20), Point::new(60, bob))
.translate(Point::new(-30, 10))
.into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
.draw(&mut display)
.unwrap();   
for mut i in 1..90{
    Line::new(Point::new(bob, 20 + (i * 2) - 5), Point::new(60 - (i * 2) , bob))
    .translate(Point::new(10, 20))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .draw(&mut display)
    .unwrap(); 
i += 1;
}
        display.flush().await.unwrap();

        if bob < 140 {
            bob += 1;
        }else{
            bob = 0;
        }
        println!("1 secs passed")
    }
}
