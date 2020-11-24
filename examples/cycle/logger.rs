use crate::AppEvent;
use drogue_kernel::{Actor, Event};

pub struct Logger;

#[derive(Debug)]
pub struct LogEvent(AppEvent);

impl Actor for Logger {
    type Event = LogEvent;

    fn process(&mut self, event: Event<Self::Event>) {
        log::info!("event: {:?}", event);
    }
}


impl From<&AppEvent> for Option<LogEvent> {
    fn from(event: &AppEvent) -> Self {
        Some(LogEvent(*event))
    }
}