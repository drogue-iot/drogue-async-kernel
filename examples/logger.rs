use crate::AppEvent;
use drogue_kernel::{Actor, Event};

pub struct Logger;

impl Actor for Logger {
    type Event = AppEvent;

    fn process(&mut self, event: Event<Self::Event>) {
        log::info!("event: {:?}", event);
    }
}
