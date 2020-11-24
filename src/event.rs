use core::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub enum Event<E> {
    Kernel(KernelEvent),
    Actor(E)
}

#[derive(Debug, Copy, Clone)]
pub enum KernelEvent {
    Initialize,
    Start,
    Shutdown,
}

