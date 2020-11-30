use stm32l4xx_hal::gpio::{PB14, Output, PushPull};
use crate::app_kernel::AppEvent;
use drogue_async_kernel::led::LEDEvent;

pub type LD2 = PB14<Output<PushPull>>;

impl From<&AppEvent> for Option<LEDEvent<LD2>> {
    fn from(event: &AppEvent) -> Self {
        match event {
            AppEvent::StartAlert => {
                Some(LEDEvent::off())
            }
            AppEvent::StopAlert => {
                Some(LEDEvent::on())
            }
            _ => {
                None
            }
        }
    }
}