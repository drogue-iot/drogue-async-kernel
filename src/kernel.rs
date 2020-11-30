

pub trait Kernel {
    type Event;
    /// Non-block dispatch of events into the kernel.
    ///
    /// Each component which dispatches should be able to `.into()`
    /// the application's event type.
    fn dispatch(event: Self::Event);
    fn interrupt(arg: i16);
}