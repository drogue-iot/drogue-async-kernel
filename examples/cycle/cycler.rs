use crate::{App, AppEvent, Cycle};
use drogue_kernel::kernel::Kernel;
use drogue_kernel::{Actor, Event, KernelEvent};

pub enum CycleEvent {
    Next,
    Set(Cycle),
}

pub struct Cycler {
    cycle: Cycle,
}

impl Cycler {
    pub fn new() -> Self {
        Self {
            cycle: Cycle::First,
        }
    }
}

impl Actor for Cycler {
    type Event = CycleEvent;

    fn process(&mut self, event: Event<CycleEvent>) {
        match event {
            Event::Kernel(KernelEvent::Initialize) => {
                App::dispatch_event(CycleEvent::Set(self.cycle).into());
            }
            Event::Actor(CycleEvent::Next) => {
                match self.cycle {
                    Cycle::First => self.cycle = Cycle::Second,
                    Cycle::Second => self.cycle = Cycle::First,
                }
                App::dispatch_event(CycleEvent::Set(self.cycle).into());
            }
            _ => {}
        }
    }
}

impl From<&AppEvent> for Option<CycleEvent> {
    fn from(event: &AppEvent) -> Self {
        match event {
            AppEvent::TriggerCycle => Some(CycleEvent::Next),
            _ => None,
        }
    }
}

impl From<CycleEvent> for AppEvent {
    fn from(event: CycleEvent) -> Self {
        match event {
            CycleEvent::Next => AppEvent::None,
            CycleEvent::Set(cycle) => AppEvent::Cycle(cycle),
        }
    }
}
