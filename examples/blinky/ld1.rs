use stm32l4xx_hal::gpio::{PA5, Output, PushPull};
use crate::app_kernel::AppEvent;
use drogue_async_kernel::led::LEDEvent;

pub type LD1 = PA5<Output<PushPull>>;

impl From<&AppEvent> for Option<LEDEvent<LD1>> {
    fn from(event: &AppEvent) -> Self {
        match event {
            AppEvent::StartAlert => {
                Some(LEDEvent::on())
            }
            AppEvent::StopAlert => {
                Some(LEDEvent::off())
            }
            _ => {
                None
            }
        }
    }
}


