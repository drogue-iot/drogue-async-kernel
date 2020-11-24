use crate::actor::Actor;
use crate::kernel::Kernel;
use core::marker::PhantomData;
use embedded_hal::digital::v2::InputPin;

pub enum ButtonEvent<PIN: InputPin> {
    Down(PhantomData<PIN>),
    Up(PhantomData<PIN>),
}

impl<PIN: InputPin> ButtonEvent<PIN> {
    pub fn down() -> Self {
        Self::Down(PhantomData::default())
    }

    pub fn up() -> Self {
        Self::Up(PhantomData::default())
    }
}

pub struct Button<PIN: InputPin, K: Kernel> {
    pin: PIN,
    _kernel: PhantomData<K>,
}

impl<PIN: InputPin, K: Kernel> Button<PIN, K>
where
    ButtonEvent<PIN>: Into<K::Event>,
{
    pub fn new(pin: PIN) -> Self {
        Self {
            pin,
            _kernel: PhantomData::default(),
        }
    }

    pub fn pin(&self) -> &PIN {
        &self.pin
    }

    pub fn pin_mut(&mut self) -> &mut PIN {
        &mut self.pin
    }

    pub fn press(&self) {
        K::dispatch_event(ButtonEvent::<PIN>::down().into());
    }

    pub fn release(&self) {
        K::dispatch_event(ButtonEvent::<PIN>::up().into());
    }
}

impl<PIN: InputPin, K: Kernel> Actor for Button<PIN, K>
where
    ButtonEvent<PIN>: Into<K::Event>,
{
    type Event = ButtonEvent<PIN>;

    fn interrupt(&mut self) {
        if self.pin.is_high().unwrap_or(false) {
            self.release();
        } else {
            self.press();
        }
    }
}
