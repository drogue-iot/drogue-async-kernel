#![no_std]
#![no_main]

mod b1;
mod ld1;
mod ld2;
mod logger;

extern crate cortex_m_rt as rt;
extern crate panic_rtt_target;

use rtt_target::rtt_init_print;

use log::LevelFilter;
use rtt_logger::RTTLogger;

use rt::{entry, exception};

use stm32l4xx_hal::{self as hal, prelude::*};

use stm32l4xx_hal::flash::FlashExt;
use stm32l4xx_hal::gpio::Edge;
use stm32l4xx_hal::pwr::PwrExt;
use stm32l4xx_hal::rcc::RccExt;

static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Debug);

use drogue_kernel::{button::Button, kernel, led::LED, Actor};

use crate::b1::{B1IrqHandler, B1};
use crate::ld1::LD1;
use crate::ld2::LD2;
use crate::logger::Logger;
use drogue_kernel::led::{ActiveHigh, InitialActive, InitialInactive};

#[derive(Copy, Clone, Debug)]
pub enum AppEvent {
    StartAlert,
    StopAlert,
}

kernel! {
    App<AppEvent> {
        logger: Logger,
        button: Button<B1, App> => B1IrqHandler,
        ld1: LED<LD1, ActiveHigh, InitialActive>,
        ld2: LED<LD2, ActiveHigh, InitialInactive>,
    }
}

impl From<&AppEvent> for Option<AppEvent> {
    fn from(event: &AppEvent) -> Self {
        Some(*event)
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    log::info!("Init");
    let mut device = hal::stm32::Peripherals::take().unwrap();
    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();
    let mut pwr = device.PWR.constrain(&mut rcc.apb1r1);
    let _clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = device.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = device.GPIOB.split(&mut rcc.ahb2);
    let mut gpioc = device.GPIOC.split(&mut rcc.ahb2);

    let mut b1 = gpioc
        .pc13
        .into_pull_up_input(&mut gpioc.moder, &mut gpioc.pupdr);
    b1.make_interrupt_source(&mut device.SYSCFG, &mut rcc.apb2);
    b1.enable_interrupt(&mut device.EXTI);
    b1.trigger_on_edge(&mut device.EXTI, Edge::RISING_FALLING);

    let b1 = Button::new(b1);

    let ld1 = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let ld2 = gpiob
        .pb14
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let ld1 = LED::new(ld1);
    let ld2 = LED::new(ld2);

    let logger = Logger {};

    App::start(logger, b1, ld1, ld2);
}

