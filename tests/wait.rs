extern crate switch_hal;

use embedded_hal_mock::eh1::digital::{Mock as PinMock, State as PinState, Transaction as PinTransaction};
use switch_hal::{WaitSwitch, Switch};

mod active_high_switch {
    use super::*;

    use switch_hal::ActiveHigh;

    mod is_active {
        use embedded_hal::digital::OutputPin;

        use super::*;

        #[async_std::test]
        async fn true_when_pin_high() {
            let expectations = [
                PinTransaction::set(PinState::High),
                PinTransaction::wait_for_state(PinState::High),
            ];

            let mut pin = PinMock::new(&expectations);
            pin.set_high().unwrap();

            let mut button = Switch::<_, ActiveHigh>::new(pin);
            button.wait_active().await.unwrap();
            button.into_pin().done();
        }

        #[async_std::test]
        async fn false_when_pin_low() {
            let expectations = [
                PinTransaction::set(PinState::Low),
                PinTransaction::wait_for_state(PinState::High),
            ];

            let mut pin = PinMock::new(&expectations);
            pin.set_low().unwrap();

            let mut button = Switch::<_, ActiveHigh>::new(pin);
            button.wait_active().await.unwrap();
            button.into_pin().done();
        }
    }
}

mod active_low_switch {
    use super::*;

    use switch_hal::ActiveLow;

    mod is_active {
        use embedded_hal::digital::OutputPin;

        use super::*;

        #[async_std::test]
        async fn false_when_pin_high() {
            let expectations = [
                PinTransaction::set(PinState::High),
                PinTransaction::wait_for_state(PinState::Low),
            ];

            let mut pin = PinMock::new(&expectations);
            pin.set_high().unwrap();

            let mut button = Switch::<_, ActiveLow>::new(pin);
            button.wait_active().await.unwrap();
            button.into_pin().done();
        }

        #[async_std::test]
        async fn true_when_pin_low() {
            let expectations = [
                PinTransaction::set(PinState::Low),
                PinTransaction::wait_for_state(PinState::Low),
            ];

            let mut pin = PinMock::new(&expectations);
            pin.set_low().unwrap();

            let mut button = Switch::<_, ActiveLow>::new(pin);
            button.wait_active().await.unwrap();
            button.into_pin().done();
        }
    }
}
