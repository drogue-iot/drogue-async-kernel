use crate::actor::Actor;
use core::marker::PhantomData;
use embedded_hal::digital::v2::OutputPin;
use crate::{Event, KernelEvent};

pub enum LEDEvent<Discriminant> {
    On(PhantomData<Discriminant>),
    Off(PhantomData<Discriminant>),
}

impl<Discriminant> LEDEvent<Discriminant> {
    pub fn on() -> LEDEvent<Discriminant> {
        LEDEvent::On(PhantomData::default())
    }

    pub fn off() -> LEDEvent<Discriminant> {
        LEDEvent::Off(PhantomData::default())
    }
}

pub struct LED<PIN: OutputPin, Active: ActiveState<PIN>, Initial: InitialState<PIN, Active>> {
    pin: PIN,
    _active: PhantomData<Active>,
    _initial: PhantomData<Initial>,
}

impl<PIN: OutputPin, Active: ActiveState<PIN>, Initial: InitialState<PIN, Active>> LED<PIN, Active, Initial> {
    pub fn new(pin: PIN) -> Self {
        Self {
            pin,
            _active: PhantomData::default(),
            _initial: PhantomData::default(),
        }
    }
}

pub trait ActiveState<PIN: OutputPin> {
    fn set_active(pin: &mut PIN) -> Result<(), PIN::Error>;
    fn set_inactive(pin: &mut PIN) -> Result<(), PIN::Error>;
}

pub struct ActiveHigh;
pub struct ActiveLow;

impl<PIN: OutputPin> ActiveState<PIN> for ActiveHigh {
    fn set_active(pin: &mut PIN) -> Result<(), PIN::Error>{
        pin.set_high()
    }

    fn set_inactive(pin: &mut PIN) -> Result<(), PIN::Error>{
        pin.set_low()
    }
}
impl<PIN: OutputPin> ActiveState<PIN> for ActiveLow {
    fn set_active(pin: &mut PIN) -> Result<(), PIN::Error>{
        pin.set_low()
    }

    fn set_inactive(pin: &mut PIN) -> Result<(), PIN::Error>{
        pin.set_high()
    }
}

pub trait InitialState<PIN: OutputPin, Active: ActiveState<PIN>> {
    fn initialize(pin: &mut PIN) -> Result<(), PIN::Error>;
}

pub struct InitialActive;
pub struct InitialInactive;

impl<PIN: OutputPin, Active: ActiveState<PIN>> InitialState<PIN, Active> for InitialActive {
    fn initialize(pin: &mut PIN) -> Result<(), PIN::Error>{
        Active::set_active(pin)
    }
}

impl<PIN: OutputPin, Active: ActiveState<PIN>> InitialState<PIN, Active> for InitialInactive {
    fn initialize(pin: &mut PIN) -> Result<(), PIN::Error>{
        Active::set_inactive(pin)
    }
}

impl<PIN: OutputPin, Active: ActiveState<PIN>, Initial: InitialState<PIN, Active>> Actor for LED<PIN, Active, Initial> {
    type Event = LEDEvent<PIN>;

    fn process(&mut self, event: Event<Self::Event>) {
        match event {
            Event::Actor(LEDEvent::On(_)) => {
                Active::set_active( &mut self.pin ).ok();
            }
            Event::Actor(LEDEvent::Off(_)) => {
                Active::set_inactive( &mut self.pin ).ok();
            }
            Event::Kernel(KernelEvent::Initialize) => {
                Initial::initialize(& mut self.pin).ok();
            }
            _ => {}
        }
    }
}