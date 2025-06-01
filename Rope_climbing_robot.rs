#![no_std]
#![no_main]

use alloc::string::String;
use embassy_executor::{Spawner};

use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{self, Input, InputConfig, Level, Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;
use esp_wifi::wifi::event::StaAuthmodeChange;
use log::info;
use tiny_rng::{Rand, Rng};
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}



extern crate alloc;
#[embassy_executor::task]
async  fn sensor_check(){
    
}
#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 72 * 1024);


//     const SSID: &str = "Servo";
// const PASSWORD: &str = "fasz123@";
    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");
        let timer1 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    // let mut led: Output<'_> = Output::new(peripherals.GPIO2, Level::Low,OutputConfig::default());
    let mut motor: Output<'_> = Output::new(peripherals.GPIO23, Level::Low,OutputConfig::default());
    let mut motor_1: Output<'_> = Output::new(peripherals.GPIO32, Level::Low, OutputConfig::default());
        let mut motor_2: Output<'_> = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());

        let mut motor_3: Output<'_> = Output::new(peripherals.GPIO27, Level::Low, OutputConfig::default());
        let mut motor_4: Output<'_> = Output::new(peripherals.GPIO26, Level::Low, OutputConfig::default());



        
        let ir_sensor_front: Input<'_> = Input::new(peripherals.GPIO15, InputConfig::default());
        let ir_sensor_back: Input<'_> = Input::new(peripherals.GPIO5, InputConfig::default());
        
     
    // TODO: Spawn some tasks
    let _ = spawner;

    let mut move_right: bool = true; // Direction state
    let mut rng = Rng::from_seed(12345);
// make a boolean like flip it if number is even and otherwise fasle yeah and it does it actives crazy mode 
      loop {
// let timeout: u8 =  (rng.rand_u8() + 1 / 2 ); 
        // stop everything first
        motor_1.set_low();         
        motor_2.set_low();         
        motor_3.set_low();         
        motor_4.set_low();         
        // motor.set_high();
        motor.set_high();
        if ir_sensor_front.is_low() {
            info!("Front detected - Moving LEFT");
            motor.set_high();

            // let value: u8 =  (rng.rand_u8() + 1 / 2 );
            
            // if value % 2 == 0{
            //     info!(" Even {value}");

            // }  else{
            //     info!(" Odd {value}");

            // }


            // to do make it so it moves and stops randomly and wats randomly
            // stoping is moving and stoping  at pre positions  and waiint rnadom time
    
            move_right = false;
            
            // motor.set_low();           // relay on
            motor_1.set_low();         
            motor_2.set_low();         
            motor_3.set_low();         
            motor_4.set_low();         
            Timer::after(Duration::from_secs((rng.rand_u8() % 15 + 1) as u64)).await;
    
            // left turn - motors spin opposite ways
            motor_1.set_high();        
            motor_2.set_low();         
            motor_3.set_high();        
            motor_4.set_low();     
            motor.set_high();

        } else if ir_sensor_back.is_low() {
            info!("Back detected - Moving RIGHT");  
            move_right = true;
            
                    // stop everything first
        motor_1.set_low();         
        motor_2.set_low();         
        motor_3.set_low();         
        motor_4.set_low();         
        Timer::after(Duration::from_secs((rng.rand_u8() % 15 + 1) as u64)).await;
        // right turn - flip motor directions
            motor_1.set_low();         
            motor_2.set_high();        
            motor_3.set_low();         
            motor_4.set_high();        
            motor.set_high();
 
        } else {
            info!("No detection - Keep moving in current direction");

            
            if move_right {
//                 motor.set_high();

                motor_1.set_low();     
                motor_2.set_high();    
                motor_3.set_low();     
                motor_4.set_high();  
                // keep going right
            } else {
                // keep going left
//                 motor.set_high();

                motor_1.set_high();    
                motor_2.set_low();     
                motor_3.set_high();    
                motor_4.set_low();     
            }
        }
        Timer::after(Duration::from_millis(50)).await;
    }
    
    
    
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin

}
