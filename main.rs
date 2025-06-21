#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
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


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
use embassy_time::{self, Duration, Timer}; 
extern crate alloc;
use esp_storage::FlashStorage;
use embedded_storage::{ReadStorage, Storage};
use esp_wifi::EspWifiController;
use wifi_ap as lib;


#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.3.1

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);
    info!("Embassy initialized!");
//     let mut counter = ReloadCounter::new();
// let reload_times = counter.get_and_increment();
// defmt::info!("Device has been reloaded {} times", reload_times.await);

    let timer1 = TimerGroup::new(peripherals.TIMG0);
    // let _init = esp_wifi::init(
    //     timer1.timer0,
    //     esp_hal::rng::Rng::new(peripherals.RNG),
    //     peripherals.RADIO_CLK,
    // )
    // .unwrap();
let mut servo = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());
let ledc = Ledc::new(peripherals.LEDC);


let mut hstimer0 = ledc.timer::<HighSpeed>(timer::Number::Timer0);
hstimer0
    .configure(timer::config::Config {
        duty: timer::config::Duty::Duty5Bit,
        clock_source: timer::HSClockSource::APBClk,
        frequency: Rate::from_khz(24),
    })
    .unwrap();



    let mut channel0 = ledc.channel(channel::Number::Channel0, &mut servo);
    channel0
        .configure(channel::config::Config {
            timer: &hstimer0,
            duty_pct: 10,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();


        let max_duty_cycle = channel0.max_duty_cycle() as u32;

        let min_duty = (25 * max_duty_cycle) / 1000;
        // Maximum duty (12.5%)
        // For 12bit -> 125 * 4096 /1000 => 512
        let max_duty = (125 * max_duty_cycle) / 1000;
        // 512 - 102 => 410
        let duty_gap = max_duty - min_duty;







        /**
         *  TO Do First use wifi to control servos and then after its done do some logic and try to combine both servoes to move in the code so it will only recive smth like
         * Directon like compass so it knows where and from there more precis and then U or D for up or down 
         */
















    let rng = Rng::new(peripherals.RNG);
    let esp_wifi_ctrl = &*lib::mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(timer1.timer0, rng.clone(), peripherals.RADIO_CLK,).unwrap()
    );
    let mut buffer = [0u8; 32];
spawner.must_spawn(reset_memory(Input::new(peripherals.GPIO4, InputConfig::default().with_pull(Pull::Up))));

    let stack = match FlashStorage::new().read(STORAGE_OFFSET, &mut buffer) {
        Ok(_) => {
            if buffer.iter().all(|&b| b == 0xFF) {
                info!("No credentials - using AP mode stack");
                lib::wifi::start_wifi(esp_wifi_ctrl, peripherals.WIFI, rng, &spawner).await.unwrap()
            } else {
                info!("Credentials found - using client mode stack");  
                let led_pin = peripherals.GPIO2;
                lib::http_wifi::start_wifi(esp_wifi_ctrl, peripherals.WIFI, rng, &spawner, led_pin).await
            }
        }
        Err(_) => {
            info!("Error reading flash - defaulting to AP mode");
            lib::wifi::start_wifi(esp_wifi_ctrl, peripherals.WIFI, rng, &spawner).await.unwrap()
        }
    };

    // bollean to spawn the good one if check from memory if the ssid and pass is pressent then do that
    // later on add a reset button to turn the bollean back if pressed more then 10 sec( the button)
    let web_app = lib::web::WebApp::default();
    let mut buffer = [0u8; 32];
    let mut host_wifi: bool = true;

    const STORAGE_OFFSET : u32 = 0x110000;


    spawner.must_spawn(lib::piezo::piezo_task(Output::new(
        peripherals.GPIO5,
        Level::Low,
        OutputConfig::default(),
    )));


    let read_memory = match FlashStorage::new().read(STORAGE_OFFSET, &mut buffer) {
        Ok(_) =>{
            if buffer.iter().all(|&b| b == 0xFF) {
                info!(" LOG LOG LOG  Theres nothing in the wifi so i stop");
                let mut host_wifi: bool = true;

                let counter = lib::kiss::get_and_increment();
                spawner.must_spawn(counter);
                for id in 0..lib::web::WEB_TASK_POOL_SIZE {
                    spawner.must_spawn(lib::web::web_task(
                        id,
                        stack,
                        web_app.router,
                        web_app.config,
                    ));
                }
                info!("Web server started...");
                return ;
            }else{ 
                info!("Data found => : {:?}", defmt::Debug2Format(&buffer));
   
                let data_end = buffer.iter()
                    .position(|&x| x == 0)
                    .unwrap_or(buffer.len());
                
                let existing_data = core::str::from_utf8(&buffer[..data_end])
                    .unwrap_or("Invalid UTF-8");
                info!("  Existing credentials: {}", existing_data);
                let web_app = lib::http_web::WebApp::default();
                for id in 0..lib::http_web::WEB_TASK_POOL_SIZE {
                    spawner.must_spawn(lib::http_web::web_task(
                        id,
                        stack,
                        web_app.router,
                        web_app.config,
                    ));
                }
                info!("Web server started...");
            }
        }
        Err(_) =>{
            info!(" LOG LOG LOG  There was an error idk what");
        }
    };


}

#[embassy_executor::task]
async fn reset_memory(mut button : Input<'static>){
    let mut last_state = button.is_high();
    let mut button_pressed_count: i8 = 0;
    let mut start_time = embassy_time::Instant::now();
    const STORAGE_OFFSET : u32 = 0x110000;
    let mut buffer = [0u8; 32];

    loop {
        let current_state = button.is_high();
        
     if current_state != last_state {
    if button.is_low() && button_pressed_count < 14 {
        button_pressed_count += 1;
            info!("Button PRESSED => {} Times", button_pressed_count);
            
            if button_pressed_count == 1 {
                start_time = embassy_time::Instant::now();
            }
            if button_pressed_count == 14 {
                let elapsed = embassy_time::Instant::now() - start_time;
                if elapsed <= embassy_time::Duration::from_secs(10) {
                    let read_memory = match FlashStorage::new().read(STORAGE_OFFSET, &mut buffer) {
                        Ok(_) => {
                            if buffer.iter().all(|&b| b == 0xFF) {
                                info!("No credentials were found - already clean");
                            } else {
                                info!("Credentials found - Cleaning now");
                                let erase_buffer = [0xFF; 32]; 
                                
                                match FlashStorage::new().write(STORAGE_OFFSET, &erase_buffer) {
                                    Ok(_) => {
                                        info!("🦀 Flash memory cleaned successfully!");
                                        info!("🦀 Device will reboot in 3 seconds...");
                                        
                                        let mut countdown = 3;
                                        let rust: &str = "🦀";
                                        
                                        while countdown > 0 {
                                            info!("Rebooting in  {}...{}", countdown,rust);

                                            Timer::after(Duration::from_millis(1000)).await;
                                            countdown -= 1;
                                        }
                                        
                                        software_reset();
                                    }
                                    Err(e) => {
                                        info!("Failed to clean flash memory: {:?}", defmt::Debug2Format(&e));
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            info!("Error reading flash memory: {:?}", defmt::Debug2Format(&e));
                        }
                    };
                } else {
                    info!("14 presses but took too long ({:?} seconds)", elapsed.as_secs());
                }
                button_pressed_count = 0;
            }
        } else {
            info!("Button RELEASED (Button was pressed before => {} Times)", button_pressed_count);
            }
            last_state = current_state;
        }
        button.wait_for_any_edge().await;
    }

 
}
