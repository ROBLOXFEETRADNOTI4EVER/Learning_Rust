use defmt::info;
use embassy_net::Stack;
use embassy_time::Duration;
use esp_alloc as _;
use esp_hal::gpio::Input;
use picoserve::{response::{File, IntoResponse}, routing, AppBuilder, AppRouter, Router};
use serde::Deserialize;
use heapless::String;
use heapless::Vec;
use esp_hal::peripherals::Peripherals;
use esp_hal::system::software_reset;
use esp_storage::FlashStorage;
use embedded_storage::{ReadStorage, Storage};
use core::{fmt::Write, panic::AssertUnwindSafe};
const STORAGE_OFFSET : u32 = 0x110000;
pub struct Application;

impl AppBuilder for Application {
    type PathRouter = impl routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        picoserve::Router::new().route(
            "/",
            routing::get_service(File::html(include_str!("index.html"))),
        ).route("/post-wifi", routing::post(wifi_handler))
    }
}
fn trigger_reset() -> ! {
    software_reset();
}
pub const WEB_TASK_POOL_SIZE: usize = 2;

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
pub async fn web_task(
    id: usize,
    stack: Stack<'static>,
    router: &'static AppRouter<Application>,
    config: &'static picoserve::Config<Duration>,
) -> ! {
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve(
        id,
        router,
        config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await
}

pub struct WebApp {
    pub router: &'static Router<<Application as AppBuilder>::PathRouter>,
    pub config: &'static picoserve::Config<Duration>,
}

impl Default for WebApp {
    fn default() -> Self {
        let router = picoserve::make_static!(AppRouter<Application>, Application.build_app());

        let config = picoserve::make_static!(
            picoserve::Config<Duration>,
            picoserve::Config::new(picoserve::Timeouts {
                start_read_request: Some(Duration::from_secs(5)),
                read_request: Some(Duration::from_secs(1)),
                write: Some(Duration::from_secs(1)),
            })
            .keep_connection_alive()
        );

        Self { router, config }
    }
}
async fn wifi_handler(picoserve::extract::Json(wifi_request): picoserve::extract::Json<WifiRequest>) -> impl IntoResponse {
    
    let ssid = &wifi_request.ssid;
    let password = &wifi_request.password;

    // info!("{:?}",defmt::Debug2Format(ssid));
    // info!("{:?}",defmt::Debug2Format(password));
    info!("SSID: {}", defmt::Display2Format(ssid));
    info!("Password: {}", defmt::Display2Format(password));
    
    // info!("{}",password);


    let mut buffer = [0u8; 32];  

   
    let current_wifi = match FlashStorage::new().read(STORAGE_OFFSET, &mut buffer) {
        Ok(_) => {
            if buffer.iter().all(|&b| b == 0xFF) {
                info!("Flash is empty/Erased - Writing new credentials");
                
                let mut buf: Vec<u8, 32> = Vec::new(); 
                buf.extend_from_slice(ssid.as_bytes()).unwrap();
                buf.extend_from_slice(":".as_bytes()).unwrap();
                buf.extend_from_slice(password.as_bytes()).unwrap();
                
                while buf.len() < 32 {
                    buf.push(0).unwrap();
                }
                
                if let Err(e) = FlashStorage::new().write(STORAGE_OFFSET, &buf) {
                    defmt::error!("Failed to write: {:?}", defmt::Debug2Format(&e));
                } else {
                    info!("Successfully wrote new credentials to flash");
                    trigger_reset();

                }
                
            } else {
                info!("Data found => : {:?}", defmt::Debug2Format(&buffer));
                
                let data_end = buffer.iter()
                    .position(|&x| x == 0)
                    .unwrap_or(buffer.len());
                
                let existing_data = core::str::from_utf8(&buffer[..data_end])
                    .unwrap_or("Invalid UTF-8");
                
                info!("Existing credentials: {}", existing_data);
                
                let mut current_creds = heapless::String::<64>::new();
                write!(current_creds, "{} {}", ssid.as_str(), password.as_str()).unwrap();
                if existing_data != current_creds {
                    info!("Updating credentials...");
                    
                    let mut buf: Vec<u8, 32> = Vec::new(); 
                    buf.extend_from_slice(ssid.as_bytes()).unwrap();
                    buf.extend_from_slice(":".as_bytes()).unwrap();
                    buf.extend_from_slice(password.as_bytes()).unwrap();
                    
                    while buf.len() < 32 {
                        buf.push(0).unwrap();
                    }
                    
                    if let Err(e) = FlashStorage::new().write(STORAGE_OFFSET, &buf) {
                        defmt::error!("Failed to update: {:?}", defmt::Debug2Format(&e));
                    } else {
                        info!("Successfully updated credentials");
                        let mut parts = existing_data.splitn(2, ':');
                    let ssid_parsed = parts.next().unwrap_or("");
                    let pass_parsed = parts.next().unwrap_or("");
                    info!("{}, {}",ssid_parsed , pass_parsed);


                    trigger_reset();
                    }
                } else {
                    info!("Credentials unchanged");
                    let mut parts = existing_data.splitn(2, ':');
                    let ssid_parsed = parts.next().unwrap_or("");
                    let pass_parsed = parts.next().unwrap_or("");

                    info!("{}, {}",ssid_parsed , pass_parsed);
                    trigger_reset();
                }
            }
        }
        Err(_) => {
            info!("There was an error while reading flash");
        }
    };
    
}


#[derive(serde::Deserialize)]
struct WifiRequest {
    ssid: String<32>,
    password: String<32>,
}


#[derive(serde::Serialize)]
struct WifiResponse {
    success: bool,
}
