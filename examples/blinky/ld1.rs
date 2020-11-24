use crate::AppEvent;
use drogue_kernel::led::LEDEvent;
use stm32l4xx_hal::gpio::gpioa::PA5;
use stm32l4xx_hal::gpio::{Output, PushPull};

pub type LD1 = PA5<Output<PushPull>>;

impl From<&AppEvent> for Option<LEDEvent<LD1>> {
    fn from(event: &AppEvent) -> Self {
        match event {
            AppEvent::StartAlert => Some(LEDEvent::on()),
            AppEvent::StopAlert => Some(LEDEvent::off()),
            _ => None,
        }
    }
}
