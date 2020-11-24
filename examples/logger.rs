use drogue_kernel::{
    Event,
    Actor,
};
use crate::AppEvent;

pub struct Logger;

impl Actor for Logger {
    type Event = AppEvent;

    fn process(&mut self, event: Event<Self::Event>) {
        log::info!( "event: {:?}", event);
    }
}