use defmt::info;
use embassy_net::Stack;
use embassy_time::Duration;
use esp_alloc as _;
use picoserve::{response::{File, IntoResponse}, routing, AppBuilder, AppRouter, Router};
use heapless::String;
pub struct Application;

impl AppBuilder for Application {
    type PathRouter = impl routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        picoserve::Router::new().route(
            "/",
            routing::get_service(File::html(include_str!("http_index.html"))),
           
            
        ) .route("/piezo", routing::post(piezo_handler))
        .route("/servo", routing::post(servo_handler))
        .route("/gps", routing::post(gps_track_handler))
        
    }
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


#[derive(serde::Deserialize)]
struct PiezoRequest {
    is_on: bool,
}
#[derive(serde::Serialize)]

struct PiezoResponse{
    success: bool,
}



#[derive(serde::Deserialize)]
struct ServoRequest {
    angle : String<32>,
    angle_b: String<32>

}

#[derive(serde::Deserialize)]
struct UartConfigRequest {
    message: String<32>,
    is_sender : bool,

}

#[derive(serde::Serialize)]

struct ServoResponse{
    success: bool,
}



#[derive(serde::Serialize)]
struct GpsTrackResponse {
    success: bool,

}


#[derive(serde::Deserialize)]
struct GpsTrackRequest {
    target_lat: String<16>,   // Reduced from 32
    target_lon: String<16>,   // Reduced from 32
    target_alt: String<16>,   // Reduced from 32
    observer_lat: String<16>, // Reduced from 32
    observer_lon: String<16>, // Reduced from 32
    observer_alt: String<16>, // Reduced from 32
}



async fn piezo_handler(input: picoserve::extract::Json<PiezoRequest, 0>) -> impl IntoResponse {
    crate::piezo::PIEZO_STATE.store(input.0.is_on, core::sync::atomic::Ordering::Relaxed);

    picoserve::response::Json(PiezoResponse { success: true })
}

async  fn uart_handler(picoserve::extract::Json(uart_data): picoserve::extract::Json<UartConfigRequest>) -> impl IntoResponse{


let message: Result<i32, ()> = parse_i32(&uart_data.message);
let is_sender = &uart_data.is_sender;
    info!(" Users Message {} \n UART SENDER MODE? => {}",message, is_sender);


    if *is_sender {
        crate::uart::UART_STATE.store(uart_data.is_sender, core::sync::atomic::Ordering::Release);

        // need to make it so i can store the value as bytes and later decode 
        
    }else{
        crate::uart::UART_STATE.store(uart_data.is_sender, core::sync::atomic::Ordering::Release);

    }



    // goal is to store the string into atomic as bytes 


    // and have a atomic bollean that detements when to use uart or not like it won't run if the boolean is false and will run other application






    picoserve::response::Json(PiezoResponse { success: true })
}
async fn gps_track_handler(picoserve::extract::Json(gps): picoserve::extract::Json<GpsTrackRequest>) -> impl IntoResponse {

    let target_lat = parse_f64(&gps.target_lat).unwrap_or(0.0);
    let target_lon = parse_f64(&gps.target_lon).unwrap_or(0.0);
    let target_alt = parse_f64(&gps.target_alt).unwrap_or(0.0);
    let observer_lat = parse_f64(&gps.observer_lat).unwrap_or(46.065799);
    let observer_lon = parse_f64(&gps.observer_lon).unwrap_or(18.169254);
    let observer_alt = parse_f64(&gps.observer_alt).unwrap_or(134.57);

    info!("target lat ==> {} target lon ==> {} target alt ==> {} observer lat ==> {} observer lon ==> {} observer alt ==> {}", 
    target_lat, target_lon, target_alt, observer_lat, observer_lon, observer_alt);
    info!("GPS handler called");

    let target = crate::servo::GpsCoord { latitude: target_lat, longitude: target_lon, altitude: target_alt };
    let observer = crate::servo::GpsCoord { latitude: observer_lat, longitude: observer_lon, altitude: observer_alt };
    let azel = crate::servo::gps_to_azimuth_elevation(&observer, &target);
    let (servo_a, servo_b) = crate::servo::azimuth_elevation_to_servo_angles(&azel);
    

    crate::servo::SERVO_ANGLE.store(servo_a, core::sync::atomic::Ordering::Relaxed);
    crate::servo::SERVO_ANGLE_B.store(servo_b, core::sync::atomic::Ordering::Relaxed);
    info!("GPS Track: Az={} El={} → Servos A={} B={}", 
    azel.azimuth as i32, azel.elevation as i32, servo_a, servo_b);
    // picoserve::response::Json(GpsTrackResponse { success: true })
    picoserve::response::Json(ServoResponse { success: true })
    


}


async fn servo_handler(picoserve::extract::Json(servo): picoserve::extract::Json<ServoRequest>) -> impl IntoResponse {

    let angle = parse_i32(&servo.angle).unwrap_or(90); 
    let angle_b = parse_i32(&servo.angle_b).unwrap_or(90);


    let clamped_angle = angle.clamp(0, 180);
    let clamped_angle_b = angle_b.clamp(0, 180);

    crate::servo::SERVO_ANGLE.store(clamped_angle, core::sync::atomic::Ordering::Relaxed);
    crate::servo::SERVO_ANGLE_B.store(clamped_angle_b, core::sync::atomic::Ordering::Relaxed);

    info!("Servo A angle set to {} \n Servo B angle set to {}", clamped_angle,clamped_angle_b);



    picoserve::response::Json(ServoResponse { success: true })
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

pub fn parse_i32(s: &str) -> Result<i32, ()> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return Err(());
    }

    let mut i = 0;
    let mut negative = false;

    if bytes[0] == b'-' {
        negative = true;
        i += 1;
    }

    let mut result: i32 = 0;

    while i < bytes.len() {
        let b = bytes[i];
        if b < b'0' || b > b'9' {
            return Err(());
        }
        let digit = (b - b'0') as i32;

        result = match result.checked_mul(10).and_then(|r| r.checked_add(digit)) {
            Some(v) => v,
            None => return Err(()), // overflow
        };

        i += 1;
    }

    if negative {
        result = -result;
    }

    Ok(result)
}


pub fn parse_f64(s: &str) -> Result<f64, ()> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return Err(());
    }

    let mut i = 0;
    let mut negative = false;
    let mut integer_part: f64 = 0.0;
    let mut decimal_part: f64 = 0.0;
    let mut decimal_divisor: f64 = 1.0;
    let mut found_decimal = false;

    if bytes[0] == b'-' {
        negative = true;
        i += 1;
    }

    while i < bytes.len() {
        let b = bytes[i];
        
        if b == b'.' {
            if found_decimal {
                return Err(());
            }
            found_decimal = true;
        } else if b >= b'0' && b <= b'9' {
            let digit = (b - b'0') as f64;
            
            if found_decimal {
                decimal_divisor *= 10.0;
                decimal_part = decimal_part * 10.0 + digit;
            } else {
                integer_part = integer_part * 10.0 + digit;
            }
        } else {
            return Err(());
        }
        
        i += 1;
    }

    let mut result = integer_part + (decimal_part / decimal_divisor);
    
    if negative {
        result = -result;
    }

    Ok(result)
}

