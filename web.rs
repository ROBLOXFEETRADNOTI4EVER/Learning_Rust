use embassy_net::Stack;
use embassy_time::Duration;
use esp_alloc as _;
use picoserve::{response::{File, IntoResponse}, routing, AppBuilder, AppRouter, Router};
use core::{include_str, sync::atomic::Ordering};
pub struct  Application;
impl AppBuilder for Application {
    type PathRouter = impl routing::PathRouter;
    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        picoserve::Router::new().route(
            "/", 
        routing::get_service(File::html(include_str!("index.html"))),
    )
    .route("/piezo", routing::post(piezo_handler))
    .route("/led", routing::post(led_handler))
    }
}

pub const WEB_TASK_POOL_SIZE: usize = 2;

pub struct  WebApp{
    pub router: &'static Router<<Application as AppBuilder>::PathRouter>,
    pub config: &'static picoserve::Config<Duration>,
}
#[derive(serde::Deserialize)]
struct LedRequest {
    is_on: bool,
}



#[derive(serde::Serialize)]
struct LedResponse {
    success: bool,
}

#[derive(serde::Deserialize)]
struct PiezoRequest {
    is_on: bool,
}
#[derive(serde::Serialize)]

struct PiezoResponse{
    success: bool,
}
impl Default for WebApp{
    fn default() -> Self {
        let router = picoserve::make_static!(AppRouter<Application>, Application.build_app());
        let config = picoserve::make_static!(
            picoserve::Config<Duration>,
            picoserve::Config::new(picoserve::Timeouts{
                start_read_request: Some(Duration::from_secs(5)),
                read_request: Some(Duration::from_secs(1)),
                write: Some(Duration::from_secs(1)),

            })
            .keep_connection_alive()
        );
        Self {router,config}
    }
    
}

async fn piezo_handler(input: picoserve::extract::Json<PiezoRequest, 0>) -> impl IntoResponse {
    crate::piezo::PIEZO_STATE.store(input.0.is_on, Ordering::Relaxed);

    picoserve::response::Json(PiezoResponse { success: true })
}

async fn led_handler(input: picoserve::extract::Json<LedRequest, 0>) -> impl IntoResponse {
    crate::led::LED_STATE.store(input.0.is_on, Ordering::Relaxed);

    picoserve::response::Json(LedResponse { success: true })
}
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
    let mut http_buffer = [0; 20248];
    picoserve::listen_and_serve(
        id, router, config, stack, port, &mut tcp_rx_buffer,&mut  tcp_tx_buffer,&mut  http_buffer).await
}



