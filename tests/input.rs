extern crate switch_hal;

use embedded_hal_mock::eh1::digital::{Mock as PinMock, State as PinState, Transaction as PinTransaction};
use switch_hal::{InputSwitch, Switch};

#[allow(clippy::bool_assert_comparison)]
mod active_high_switch {
    use super::*;

    use switch_hal::ActiveHigh;

    mod is_active {
        use embedded_hal::digital::OutputPin;

        use super::*;

        #[test]
        fn true_when_pin_high() {
            let expectations = [
                PinTransaction::set(PinState::High),
                PinTransaction::get(PinState::High),
            ];

            let mut pin = PinMock::new(&expectations);
            pin.set_high().unwrap();

            let mut button = Switch::<_, ActiveHigh>::new(pin);
            assert_eq!(true, button.is_active().unwrap());
            button.into_pin().done();
        }

        #[test]
        fn false_when_pin_low() {
            let expectations = [
                PinTransaction::set(PinState::Low),
                PinTransaction::get(PinState::Low),
            ];

            let mut pin = PinMock::new(&expectations);
            pin.set_low().unwrap();

            let mut button = Switch::<_, ActiveHigh>::new(pin);
            assert_eq!(false, button.is_active().unwrap());
            button.into_pin().done();
        }
    }
}

#[allow(clippy::bool_assert_comparison)]
mod active_low_switch {
    use super::*;

    use switch_hal::ActiveLow;

    mod is_active {
        use embedded_hal::digital::OutputPin;

        use super::*;

        #[test]
        fn false_when_pin_high() {
            let expectations = [
                PinTransaction::set(PinState::High),
                PinTransaction::get(PinState::High),
            ];

            let mut pin = PinMock::new(&expectations);
            pin.set_high().unwrap();

            let mut button = Switch::<_, ActiveLow>::new(pin);
            assert_eq!(false, button.is_active().unwrap());
            button.into_pin().done();
        }

        #[test]
        fn true_when_pin_low() {
            let expectations = [
                PinTransaction::set(PinState::Low),
                PinTransaction::get(PinState::Low),
            ];

            let mut pin = PinMock::new(&expectations);
            pin.set_low().unwrap();

            let mut button = Switch::<_, ActiveLow>::new(pin);
            assert_eq!(true, button.is_active().unwrap());
            button.into_pin().done();
        }
    }
}
