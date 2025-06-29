#![no_std]
#![feature(impl_trait_in_assoc_type)]

pub mod web;
pub mod wifi;
pub mod counter;
pub mod kiss;
pub mod getwifi;
pub mod http_web;
pub mod http_wifi;
pub mod piezo;
pub mod servo;
pub mod uart;

#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}