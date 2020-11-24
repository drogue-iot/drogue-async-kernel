use crate::event::Event;
use bare_metal::Nr;

/// A live component of the system. May be written in a re-usable style.
///
/// Given the non-HALness of actual interrupt setting and clearing, an
/// `Actor` may be paired with an `InterruptHandler<_,_>` for a complete,
/// interrupt-driven component.
pub trait Actor {

    /// The type of event this actor work in terms of.
    type Event;

    /// Process an event, possibly doing nothing.
    fn process(&mut self, _event: Event<Self::Event>) {}

    /// Handle an associated interrupt.
    fn interrupt(&mut self) {}
}

/// A trait which may be associated (but not implemented by)
/// an actor, in order to perform platform-specific interrupt
/// checking and clearing.
pub trait InterruptHandler<C: Actor, Irq: Nr + Copy> {

    /// The interrupt line associated with this handler.
    /// Given that several semantic interrupts may be linked
    /// to a given line (EXTI for instance), further checking
    /// will be performed by `check_interrupt(...)`.
    const IRQ: Irq;

    /// Check if the interrupt line actually desires to
    /// operate *this* particular action.
    fn check_interrupt(component: &mut C) -> bool;

    /// Clear the interrupt for this particular action.
    fn clear_interrupt(component: &mut C);
}
