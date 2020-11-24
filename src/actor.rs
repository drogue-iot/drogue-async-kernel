use bare_metal::Nr;
use crate::event::Event;

pub trait Actor {
    type Event;
    fn process(&mut self, _event: Event<Self::Event>) {

    }

    fn interrupt(&mut self) {

    }
}

pub trait InterruptHandler<C: Actor, Irq: Nr + Copy> {
    const IRQ: Irq;
    fn check_interrupt(component: &mut C) -> bool;
    fn clear_interrupt(component: &mut C);
}


