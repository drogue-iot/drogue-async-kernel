use crate::{AppEvent, Cycle};
use drogue_kernel::led::LEDEvent;
use stm32l4xx_hal::gpio::gpiob::PB14;
use stm32l4xx_hal::gpio::{Output, PushPull};

pub type LD2 = PB14<Output<PushPull>>;

impl From<&AppEvent> for Option<LEDEvent<LD2>> {
    fn from(event: &AppEvent) -> Self {
        match event {
            AppEvent::Cycle(Cycle::First) => Some(LEDEvent::off()),
            AppEvent::Cycle(Cycle::Second) => Some(LEDEvent::on()),
            _ => None,
        }
    }
}
