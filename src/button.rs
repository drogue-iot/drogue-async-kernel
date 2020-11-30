use embedded_hal::digital::v2::InputPin;

use heapless::{
    String,
    consts::*
};
use crate::kernel::Kernel;
use core::marker::PhantomData;

pub enum ButtonEvent<Discriminant> {
    Down(PhantomData<Discriminant>),
    Up(PhantomData<Discriminant>)
}

impl<Discriminant> ButtonEvent<Discriminant> {

    pub fn down() -> Self {
        ButtonEvent::Down(PhantomData::default())
    }

    pub fn up() -> Self {
        ButtonEvent::Up(PhantomData::default())
    }
}

pub struct Button<PIN: InputPin, KERNEL: Kernel> {
    pin: PIN,
    name: String<U16>,
    _kernel: PhantomData<KERNEL>
}

impl<PIN: InputPin, KERNEL: Kernel> Button<PIN, KERNEL>
    where KERNEL::Event: From<ButtonEvent<PIN>>
{
    pub fn new(pin: PIN, name: &str) -> Self {
        Self {
            pin,
            name: name.into(),
            _kernel: PhantomData::default(),
        }
    }

    pub fn on_interrupt(&mut self) {
        if self.pin.is_low().unwrap_or(false) {
            KERNEL::dispatch( ButtonEvent::down().into())
        } else {
            KERNEL::dispatch( ButtonEvent::up().into())
        }
    }

    pub fn pin(&self) -> &PIN {
        &self.pin
    }

    pub fn pin_mut(&mut self) -> &mut PIN {
        &mut self.pin

    }
}