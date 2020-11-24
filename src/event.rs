use core::fmt::Debug;

pub trait Optional {
    fn is_none(&self) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub enum Event<E> {
    Kernel(KernelEvent),
    Actor(E),
}

#[derive(Debug, Copy, Clone)]
pub enum KernelEvent {
    Initialize,
    Start,
    Shutdown,
}
