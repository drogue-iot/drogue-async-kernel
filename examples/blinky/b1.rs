use stm32l4xx_hal::gpio::{PC13, Input, PullUp, ExtiPin};
use drogue_async_kernel::button::{ButtonEvent, Button};
use crate::app_kernel::{AppEvent, AppKernel};
use stm32l4::stm32l4x5::Interrupt::EXTI15_10;
use cortex_m::interrupt::Nr;

pub type B1 = PC13<Input<PullUp>>;

impl From<ButtonEvent<B1>> for AppEvent {
    fn from(event: ButtonEvent<B1>) -> Self {
        match event {
            ButtonEvent::Down(_) => {
                AppEvent::StartAlert
            }
            ButtonEvent::Up(_) => {
                AppEvent::StopAlert
            }
        }
    }
}

pub struct InterruptableB1(pub Button<B1, AppKernel>);

impl InterruptableB1 {

    pub fn on_interrupt(&mut self) {
        self.0.on_interrupt();
    }

    pub fn is_interrupt(&mut self, irqn: i16) -> bool {
        if EXTI15_10.nr() == irqn as u8 {
            self.0.pin_mut().check_interrupt()
        } else {
            false
        }
    }

    pub fn clear_interrupt(&mut self) {
        self.0.pin_mut().clear_interrupt_pending_bit()
    }

}
