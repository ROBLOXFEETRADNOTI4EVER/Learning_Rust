use defmt::info;
use embassy_executor::Spawner;
use embassy_net::{DhcpConfig, Runner, Stack, StackResources};
use embassy_time::{Duration, Timer};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::rng::Rng;
use esp_println as _;
use esp_println::println;
use esp_wifi::wifi::{self, WifiController, WifiDevice, WifiEvent, WifiState};
use esp_wifi::EspWifiController;
use esp_storage::FlashStorage;
use embedded_storage::{ReadStorage, Storage};
use crate::mk_static;
use critical_section::Mutex;
use core::cell::RefCell;
use heapless; 
use esp_hal::gpio::{self, Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::{clock::CpuClock, peripheral};

static SSID_STORAGE: Mutex<RefCell<[u8; 32]>> = Mutex::new(RefCell::new([0; 32]));
static PASSWORD_STORAGE: Mutex<RefCell<[u8; 64]>> = Mutex::new(RefCell::new([0; 64]));
static SSID_LEN: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));
static PASSWORD_LEN: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0));
fn set_global_credentials(ssid: &str, password: &str) {
    critical_section::with(|cs| {
        let ssid_bytes = ssid.as_bytes();
        let pass_bytes = password.as_bytes();
        
        let ssid_len = ssid_bytes.len().min(31);
        let pass_len = pass_bytes.len().min(63);
        
        let mut ssid_storage = SSID_STORAGE.borrow_ref_mut(cs);
        ssid_storage[..ssid_len].copy_from_slice(&ssid_bytes[..ssid_len]);
        ssid_storage[ssid_len] = 0; // null terminator
        *SSID_LEN.borrow_ref_mut(cs) = ssid_len;
        
        let mut pass_storage = PASSWORD_STORAGE.borrow_ref_mut(cs);
        pass_storage[..pass_len].copy_from_slice(&pass_bytes[..pass_len]);
        pass_storage[pass_len] = 0; // null terminator
        *PASSWORD_LEN.borrow_ref_mut(cs) = pass_len;
    });
}

fn get_global_ssid() -> Option<[u8; 32]> {
    critical_section::with(|cs| {
        let storage = SSID_STORAGE.borrow_ref(cs);
        let len = *SSID_LEN.borrow_ref(cs);
        if len > 0 {
            Some(*storage)
        } else {
            None
        }
    })
}

fn get_global_password() -> Option<[u8; 64]> {
    critical_section::with(|cs| {
        let storage = PASSWORD_STORAGE.borrow_ref(cs);
        let len = *PASSWORD_LEN.borrow_ref(cs);
        if len > 0 {
            Some(*storage)
        } else {
            None
        }
    })
}
#[embassy_executor::task]
async fn connection_task(mut controller: WifiController<'static>,spawner: Spawner,   led_pin: esp_hal::gpio::GpioPin<2> ) {
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.capabilities());
    let mut led_spawned = false;
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::StaConnected => {
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }

        const STORAGE_OFFSET : u32 = 0x110000;
        let mut buffer = [0u8; 32];

        let read_memory = match FlashStorage::new().read(STORAGE_OFFSET,&mut buffer){
            Ok(_) => {
                if buffer.iter().all(|&b| b == 0xFF) {
                    info!("Not impmented for this case just its emmpty the logic failed i should have aded auto reset and bollean reset aswell well ummmm idk")
                }
                else{
                    info!("Data found => : {:?}", defmt::Debug2Format(&buffer));
                    let data_end = buffer.iter()
                    .position(|&x| x == 0)
                    .unwrap_or(buffer.len());
                
                let existing_data = core::str::from_utf8(&buffer[..data_end])
                    .unwrap_or("Invalid UTF-8");
                
                info!("  Existing credentials: {}", existing_data);


                let parts: heapless::Vec<&str, 2> = existing_data.split(':').collect();
                let ssid_parsed = parts.get(0).unwrap_or(&"");
                let pass_parsed = parts.get(1).unwrap_or(&"");

                set_global_credentials(ssid_parsed, pass_parsed);

                }
            }
            Err(_) =>{

            }
        };

        if !matches!(controller.is_started(), Ok(true)) {
            let (ssid_string, password_string) = critical_section::with(|cs| {
                let ssid_storage = SSID_STORAGE.borrow_ref(cs);
                let pass_storage = PASSWORD_STORAGE.borrow_ref(cs);
                let ssid_len = *SSID_LEN.borrow_ref(cs);
                let pass_len = *PASSWORD_LEN.borrow_ref(cs);
                
                let ssid_str = core::str::from_utf8(&ssid_storage[..ssid_len]).unwrap_or("");
                let pass_str = core::str::from_utf8(&pass_storage[..pass_len]).unwrap_or("");
                let ssid_string: heapless::String<32> = heapless::String::try_from(ssid_str)
                .unwrap_or_else(|_| heapless::String::new());
            let password_string: heapless::String<64> = heapless::String::try_from(pass_str)
                .unwrap_or_else(|_| heapless::String::new());
                (ssid_string, password_string)
            });

            let client_config = wifi::Configuration::Client(wifi::ClientConfiguration {
                ssid: ssid_string,
                password: password_string,
                ..Default::default()
            });
            
            controller.set_configuration(&client_config).unwrap();
            println!("Starting wifi");
            controller.start_async().await.unwrap();
            println!("Wifi started!");
        }
        match controller.connect_async().await {
            Ok(_) => { println!("Wifi connected!");
            if !led_spawned {
                let led = Output::new(led_pin, Level::Low, OutputConfig::default());
                spawner.must_spawn(blink_led(led));
                led_spawned = true;
            }
                        break;
        }
            ,
            Err(e) => {
                println!("Failed to connect to wifi: {:?}", e);
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }

        }

   

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}

#[embassy_executor::task]
async  fn blink_led(mut led: Output<'static>){


    for _ in (1..7) {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

pub async fn start_wifi(
    esp_wifi_ctrl: &'static EspWifiController<'static>,
    wifi: esp_hal::peripherals::WIFI,
    mut rng: Rng,
    spawner: &Spawner,
    led_pin: esp_hal::gpio::GpioPin<2>
) -> Stack<'static> {
    let (controller, interfaces) = esp_wifi::wifi::new(&esp_wifi_ctrl, wifi).unwrap();
    let wifi_interface = interfaces.sta;
    let net_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    let dhcp_config = DhcpConfig::default();
    let net_config = embassy_net::Config::dhcpv4(dhcp_config);

    // Init network stack
    let (stack, runner) = embassy_net::new(
        wifi_interface,
        net_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        net_seed,
    );

    spawner.spawn(connection_task(controller, spawner.clone(), led_pin)).ok();
            spawner.spawn(net_task(runner)).ok();

    wait_for_connection(stack).await;

    stack
}

async fn wait_for_connection(stack: Stack<'_>) {
    println!("Waiting for link to be up");
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    println!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            println!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}