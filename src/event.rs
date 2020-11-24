use core::fmt::Debug;


/// Marker trait to allow for application events to denote a `None` variant
/// since orphan rules prevent `Into`/`From` for `Option<T>` even if `T` is
/// local to the crate.
pub trait Optional {
    /// Determine if this event variant is equivalent to `None`
    fn is_none(&self) -> bool;
}


/// An event to be handled.
///
/// The `Kernel` variant is only creatable by the kernel
/// itself and denotes lifecycle events.
///
/// The `Actor` variant will wrap an application or actor-specific
/// event for ingress or egress.
#[derive(Debug, Copy, Clone)]
pub enum Event<E> {
    Kernel(KernelEvent),
    Actor(E),
}

/// A kernel lifecycle event.
#[derive(Debug, Copy, Clone)]
pub enum KernelEvent {
    /// Initialization prior to interrupts or the first event-loop.
    Initialize,
    Start,
    Shutdown,
}
