//#![cfg_attr(not(test), no_std)]
#![no_std]
//pub mod app;

pub mod actor;
pub mod button;
pub mod event;
pub mod kernel;
pub mod led;

pub use actor::{Actor, InterruptHandler};
pub use event::{Event, KernelEvent};

pub extern crate heapless;

/*
#[cfg(test)]
mod tests {
    use crate::component::Component;
    use crate::event::Event;
    use crate::button::{Button, ButtonEvent};
    use crate::kernel;
    use crate::kernel::Kernel;


    #[derive(Debug)]
    pub struct MyState {
        pub total: i32,
    }

    impl Default for MyState {
        fn default() -> Self {
            Self {
                total: 0
            }
        }
    }

    impl From<&MyState> for () {
        fn from(_: &MyState) -> Self {
            ()
        }
    }

    #[derive(Debug)]
    pub enum MyEvent {
        ButtonPressed,
        ButtonReleased,
        ThingyTapped,
    }

    #[derive(Debug)]
    pub struct ButtonA;

    #[derive(Debug)]
    pub struct ButtonB;

    impl From<ButtonEvent<ButtonA>> for Option<MyEvent> {
        fn from(event: ButtonEvent<ButtonA>) -> Self {
            match event {
                ButtonEvent::Down(_) => {
                    Some(MyEvent::ButtonPressed)
                }
                ButtonEvent::Up(_) => {
                    Some(MyEvent::ButtonReleased)
                }
            }
        }
    }

    impl From<ButtonEvent<ButtonB>> for Option<MyEvent> {
        fn from(event: ButtonEvent<ButtonB>) -> Self {
            match event {
                ButtonEvent::Down(_) => {
                    Some(MyEvent::ThingyTapped)
                }
                ButtonEvent::Up(_) => {
                    None
                }
            }
        }
    }

    impl<'event, Discriminant> From<&'event MyEvent> for Option<ButtonEvent<Discriminant>> {
        fn from(_: &'event MyEvent) -> Self {
            None
        }
    }

    kernel! {
        MyKernel<MyEvent> {
            button_a: Button<ButtonA, MyKernel>,
            button_b: Button<ButtonB, MyKernel>,
        }
    }

    #[test]
    fn it_works() {

        let mut button_a = Button::<ButtonA, MyKernel>::new();
        let mut button_b = Button::<ButtonB, MyKernel>::new();

        MyKernel::initialize(button_a, button_b).unwrap();

        MyKernel::button_a().press();
        MyKernel::button_b().press();
        MyKernel::button_b().release();
        MyKernel::button_a().release();

        MyKernel::event_loop();

    }
}
*/
