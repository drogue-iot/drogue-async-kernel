use drogue_async_kernel::{
    kernel::Kernel,
    led::LED,
    button::Button,
};

use crate::{
    ld1::LD1,
    ld2::LD2,
    b1::B1,
};
use stm32l4xx_hal::stm32::Peripherals;
use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal::gpio::{Edge, PC13, Input, PullUp};
use async_embedded::unsync::Channel;
use drogue_async_kernel::button::ButtonEvent;
use drogue_async_kernel::led::LEDEvent;
use async_embedded::task::block_on;
use async_embedded::task;
use crate::b1::InterruptableB1;
use cortex_m::peripheral::NVIC;
use stm32l4::stm32l4x5::Interrupt::EXTI15_10;
use cortex_m::interrupt::Nr;

#[derive(Debug)]
pub enum AppEvent {
    None,
    StartAlert,
    StopAlert,
}

pub struct AppKernel {
    b1: InterruptableB1,
    ld1: Option<LED<LD1>>,
    ld2: Option<LED<LD2>>,
    channels: Channels,
}

impl AppKernel {
    pub fn new(mut device: Peripherals) -> Self {

        log::info!( "initializing");
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

        let b1 = Button::<_, AppKernel>::new(b1, "button-1");
        let b1 = InterruptableB1(b1);

        let ld1 = gpioa
            .pa5
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

        let ld1 = LED::new(ld1, "LD-1");

        let ld2 = gpiob
            .pb14
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

        let ld2 = LED::new(ld2, "LD-2");

        log::info!( "initialized");
        Self {
            b1,
            ld1: Some(ld1),
            ld2: Some(ld2),
            channels: Channels {
                ld1: Channel::new(),
                ld2: Channel::new(),
            },
        }
    }

    fn channel_ld1() -> &'static Channel<LEDEvent<LD1>> {
        unsafe {
            &KERNEL.as_ref().unwrap().channels.ld1
        }
    }

    fn channel_ld2() -> &'static Channel<LEDEvent<LD2>> {
        unsafe {
            &KERNEL.as_ref().unwrap().channels.ld2
        }
    }

    pub fn start(mut device: Peripherals) -> ! {
        let mut kernel = AppKernel::new(device);
        let mut ld1 = kernel.ld1.take().unwrap();
        let mut ld2 = kernel.ld2.take().unwrap();

        log::info!( "starting");
        task::spawn( async move {
            let channel = AppKernel::channel_ld1();
            loop {
                cortex_m::asm::nop();
                log::info!("led1 loop");
                ld1.on_event(channel.recv().await).await;
            }
        });

        task::spawn( async move {
            let channel = AppKernel::channel_ld2();
            loop {
                cortex_m::asm::nop();
                log::info!("led2 loop");
                ld2.on_event(channel.recv().await).await;
            }
        });


        unsafe {
            KERNEL.replace(kernel);
        }

        /*
        task::spawn( async {
            loop {
                let event = AppKernel::channel_ld2().recv().await;
                log::info!( "ld2 processing event {:?}", event);
                AppKernel::ld2().on_event(event).await;
            }
        });
         */


        log::info!( "started");


        block_on(async {
            cortex_m::interrupt::free( |cs| {
                unsafe {
                    NVIC::unmask( EXTI15_10 );
                }
            });
            loop {
                task::r#yield().await;
                cortex_m::asm::nop();
                //for _ in 1..100_000 {
                //}
                //task::r#yield().await;
            }
        } )



    }

    fn dispatch(&mut self, event: AppEvent) {
        log::info!("dispatch {:?}", event);
        self.channels.dispatch(event)
    }

    fn interrupt(&mut self, irqn: i16) {
        log::info!( "interrupt");
        if self.b1.is_interrupt(irqn) {
            self.b1.on_interrupt();
            self.b1.clear_interrupt();
        }
    }
}

pub struct Channels {
    ld1: Channel<LEDEvent<LD1>>,
    ld2: Channel<LEDEvent<LD2>>,
}

impl Channels {
    fn dispatch(&mut self, event: AppEvent) {
        if let Some(e) = (&event).into() {
            log::info!("try_send to ld1");
            self.ld1.try_send(e);
        }
        if let Some(e) = (&event).into() {
            self.ld2.try_send(e);
        }
    }
}

static mut KERNEL: Option<AppKernel> = None;

impl Kernel for AppKernel {
    type Event = AppEvent;

    fn dispatch(event: Self::Event) {
        unsafe {
            KERNEL.as_mut().unwrap().dispatch(event)
        }
    }

    fn interrupt(irqn: i16) {
        unsafe {
            KERNEL.as_mut().unwrap().interrupt(irqn);
        }
    }
}