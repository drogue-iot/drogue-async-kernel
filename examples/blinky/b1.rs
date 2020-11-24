use crate::{App, AppEvent};
use drogue_kernel::{
    button::{Button, ButtonEvent},
    InterruptHandler,
};
use stm32l4xx_hal::gpio::gpioc::PC13;
use stm32l4xx_hal::gpio::{ExtiPin, Input, PullUp};
use stm32l4xx_hal::pac::Interrupt::{self, EXTI15_10};

pub type B1 = PC13<Input<PullUp>>;

impl From<ButtonEvent<B1>> for AppEvent {
    fn from(event: ButtonEvent<B1>) -> Self {
        match event {
            ButtonEvent::Down(_) => AppEvent::StartAlert,
            ButtonEvent::Up(_) => AppEvent::StopAlert,
        }
    }
}

impl From<&AppEvent> for Option<ButtonEvent<B1>> {
    fn from(_event: &AppEvent) -> Self {
        None
    }
}

pub struct B1IrqHandler;

impl InterruptHandler<Button<B1, App>, Interrupt> for B1IrqHandler {
    const IRQ: Interrupt = EXTI15_10;

    fn check_interrupt(component: &mut Button<B1, App>) -> bool {
        component.pin_mut().check_interrupt()
    }

    fn clear_interrupt(component: &mut Button<B1, App>) {
        component.pin_mut().clear_interrupt_pending_bit()
    }
}
