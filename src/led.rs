use embedded_hal::digital::v2::OutputPin;
use crate::event::Event;
use core::marker::PhantomData;
use async_embedded::{
    task::spawn,
    unsync::Channel,
};

use core::fmt::{Debug, Formatter};
use async_embedded::task::block_on;
use heapless::{
    consts::*,
    String,
};

pub struct LED<PIN: OutputPin> {
    pin: PIN,
    name: String<U16>,
}

impl<PIN: OutputPin> LED<PIN> {
    pub fn new(pin: PIN, name: &str) -> Self {
        Self {
            pin,
            name: name.into(),
        }
    }

    pub async fn on_event(&mut self, event: LEDEvent<PIN>) {
        match event {
            LEDEvent::On(_) => {
                self.pin.set_high();
            }
            LEDEvent::Off(_) => {
                self.pin.set_low();
            }
        }
    }
}

pub enum LEDEvent<Discriminant> {
    On(PhantomData<Discriminant>),
    Off(PhantomData<Discriminant>),
}

impl<Discriminant> Debug for LEDEvent<Discriminant> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            LEDEvent::On(_) => {
                f.write_str("LEDState:On")
            }
            LEDEvent::Off(_) => {
                f.write_str("LEDState:Off")
            }
        }
    }
}

impl<Disciminant> LEDEvent<Disciminant> {
    pub fn on() -> LEDEvent<Disciminant> {
        LEDEvent::On(PhantomData::default())
    }

    pub fn off() -> LEDEvent<Disciminant> {
        LEDEvent::Off(PhantomData::default())
    }
}

/*
impl<PIN: OutputPin> LED<PIN> {
    type Event = LEDState<PIN>;

    fn start(&'static mut self, channel: &'static Channel<Event<Self::Event>>) {
        spawn(async move {
            loop {
                let event = channel.recv().await;
                match event {
                    Event::Actor(LEDState::On(_)) => {
                        self.pin.set_high();
                    }
                    Event::Actor(LEDState::Off(_)) => {
                        self.pin.set_low();
                    }
                    _ => {}
                }
            }
        })
    }
}
 */