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
    angle : String<32>

}
#[derive(serde::Serialize)]

struct ServoResponse{
    success: bool,
}





async fn piezo_handler(input: picoserve::extract::Json<PiezoRequest, 0>) -> impl IntoResponse {
    crate::piezo::PIEZO_STATE.store(input.0.is_on, core::sync::atomic::Ordering::Relaxed);

    picoserve::response::Json(PiezoResponse { success: true })
}




async fn servo_handler(picoserve::extract::Json(servo): picoserve::extract::Json<ServoRequest>) -> impl IntoResponse {

    let angle = parse_i32(&servo.angle).unwrap_or(90); 
    let clamped_angle = angle.clamp(0, 180);
    crate::servo::SERVO_ANGLE.store(clamped_angle, core::sync::atomic::Ordering::Relaxed);
    info!("Servo angle set to {}", clamped_angle);



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


