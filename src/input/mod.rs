use crate::{ActiveHigh, ActiveLow, InputSwitch, Switch};
use embedded_hal::digital::InputPin;

impl<T: InputPin> InputSwitch for Switch<T, ActiveHigh> {
    type Error = T::Error;

    fn is_active(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }
}

impl<T: InputPin> InputSwitch for Switch<T, ActiveLow> {
    type Error = T::Error;

    fn is_active(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_low()
    }
}
