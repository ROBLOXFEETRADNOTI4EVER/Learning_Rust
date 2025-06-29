use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use defmt::info;
use embassy_time::{Duration, Timer};
use esp_hal::{gpio::Output, peripheral, peripherals, uart::{self, Config, Uart}};
use core::fmt::Write;
use heapless::{String, Vec};
use esp_hal::Async;

pub static UART_STATE: AtomicBool = AtomicBool::new(false);
pub static UART_MESSAGE : AtomicUsize = AtomicUsize::new(8192);
// pub static UART_MESSAGEE : Atomic

#[embassy_executor::task]
// will pass the bytes as an argument much easier

//  i could do 1 file instead of relying on 2 hmmmmmmm hmmmmmmmm   
pub async fn uart_task(mut uart:  Uart<'static, esp_hal::Blocking> ) {
    loop {
        // if UART_STATE.load(Ordering::Relaxed) {
        //     piezo.set_high();
        //     Timer::after(Duration::from_millis(500)).await;
        //     piezo.set_low();
        // } else {
        //     piezo.set_low();
        // }
        // Timer::after(Duration::from_millis(50)).await;

        // IF UART STATE IS OFF IT WILL START SENDING DATA 
            // So Sender mode will be on




        if UART_STATE.load(Ordering::Relaxed){
            // Sending Data
  let mut counter = 0u32;
    let mut bufff: Vec<_, 512> = Vec::new(); // make sure keep vector size so it will work the the txt lenght more then the vector size it will still compile but you will have no idea why it won't run 
        let txt = "\n I like a girl named Panna  Panna \n 1282181828128 VIGHFERI a21882181288 VIGHFERRIIi 123456789 I  123456789 II 123456789 III "; // here need to get the data
        bufff.extend_from_slice(txt.as_bytes()).unwrap();

    info!("Starting UART random even number sender");
    loop {

        uart.write(&bufff).ok();
        info!("{}",bufff);

        // Timer::after(Duration::from_secs(1)).await;

        let num = counter % 256;
        
        let mut buf = String::<32>::new();
        write!(buf, "{}\r\n", num).unwrap();
        info!("{}",buf);
        uart.write(buf.as_bytes()).ok();
        counter += 1;
        Timer::after(Duration::from_millis(50)).await;


        
    }



        }else{
            // Receving data mode On 



        }





    }
}