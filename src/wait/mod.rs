use embedded_hal_async::digital::Wait;

use crate::{ActiveHigh, ActiveLow, WaitSwitch, Switch};

impl<T: Wait> WaitSwitch for Switch<T, ActiveHigh> {
    type Error = T::Error;

    async fn wait_active(&mut self) -> Result<(), Self::Error> {
        self.pin.wait_for_high().await
    }
}

impl<T: Wait> WaitSwitch for Switch<T, ActiveLow> {
    type Error = T::Error;

    async fn wait_active(&mut self) -> Result<(), Self::Error> {
        self.pin.wait_for_low().await
    }
}
