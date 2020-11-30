#![no_std]
#![no_main]


mod app_kernel;
mod b1;
mod ld1;
mod ld2;

extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use rtt_target::{ rtt_init_print, rprintln};

use log::LevelFilter;
use rtt_logger::RTTLogger;
use async_embedded::task::block_on;

//use rt::{entry, exception};
use cortex_m_rt::{ entry, exception };

use stm32l4xx_hal::{self as hal, prelude::*};

use stm32l4xx_hal::flash::FlashExt;
use stm32l4xx_hal::gpio::{Edge, Output, PushPull};
use stm32l4xx_hal::pwr::PwrExt;
use stm32l4xx_hal::rcc::RccExt;

static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Debug);

use drogue_async_kernel::led::{LED, LEDEvent};
use async_embedded::unsync::Channel;
use stm32l4xx_hal::gpio::gpioa::PA5;
use drogue_async_kernel::event::Event;
use core::cell::Cell;
use stm32l4xx_hal::gpio::gpiob::PB14;
use core::future::Future;
use crate::app_kernel::{AppKernel, AppEvent};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Init");

    AppKernel::start(
    hal::stm32::Peripherals::take().unwrap()
    );
}

use drogue_async_kernel::kernel::Kernel;

#[exception]
fn DefaultHandler(irqn: i16) {
    AppKernel::interrupt(irqn);
}
