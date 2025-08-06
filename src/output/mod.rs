use embedded_hal::digital::{OutputPin, StatefulOutputPin};

use crate::{ActiveHigh, ActiveLow, OutputSwitch, Switch, StatefulOutputSwitch};

impl<T: OutputPin> OutputSwitch for Switch<T, ActiveHigh> {
    type Error = T::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }
}

impl<T: OutputPin> OutputSwitch for Switch<T, ActiveLow> {
    type Error = T::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

impl<T: StatefulOutputPin> StatefulOutputSwitch
    for Switch<T, ActiveLow>
{
    fn is_on(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_set_low()
    }

    fn is_off(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_set_high()
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}

impl<T: StatefulOutputPin> StatefulOutputSwitch
    for Switch<T, ActiveHigh>
{
    fn is_on(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_set_high()
    }

    fn is_off(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_set_low()
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}
