extern crate switch_hal;

use embedded_hal_mock::eh1::digital::{Mock as PinMock, State as PinState, Transaction as PinTransaction};
use switch_hal::{OutputSwitch, Switch};

#[allow(clippy::bool_assert_comparison)]
mod active_high_switch {
    use super::*;
    use embedded_hal::digital::InputPin;
    use switch_hal::ActiveHigh;

    #[test]
    fn when_on_pin_is_high() {
        let expectations = [
            PinTransaction::set(PinState::High),
            PinTransaction::get(PinState::High),
        ];

        let pin = PinMock::new(&expectations);

        let mut led = Switch::<_, ActiveHigh>::new(pin);
        led.on().unwrap();

        let mut pin = led.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
        pin.done();
    }

    #[test]
    fn when_off_pin_is_low() {
        let expectations = [
            PinTransaction::set(PinState::Low),
            PinTransaction::get(PinState::Low),
        ];

        let pin = PinMock::new(&expectations);

        let mut led = Switch::<_, ActiveHigh>::new(pin);
        led.off().unwrap();

        let mut pin = led.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
        pin.done();
    }

    #[test]
    fn is_toggleable() {
        use switch_hal::StatefulOutputSwitch;

        let expectations = [
            PinTransaction::set(PinState::Low),
            PinTransaction::toggle(),
            PinTransaction::get(PinState::High),
        ];

        let pin = PinMock::new(&expectations);

        let mut led = Switch::<_, ActiveHigh>::new(pin);
        led.off().unwrap();

        led.toggle().unwrap();

        let mut pin = led.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
        pin.done();
    }

    #[test]
    fn not_on_when_low() {
        use switch_hal::StatefulOutputSwitch;

        let expectations = [
            PinTransaction::set(PinState::Low),
            PinTransaction::get_state(PinState::Low),
            PinTransaction::get(PinState::Low),
        ];

        let pin = PinMock::new(&expectations);

        let mut led = Switch::<_, ActiveHigh>::new(pin);
        led.off().unwrap();

        assert_eq!(false, led.is_on().unwrap());

        let mut pin = led.into_pin();
        assert_eq!(false, pin.is_high().unwrap());
        pin.done();
    }

    #[test]
    fn is_on_when_high() {
        use switch_hal::StatefulOutputSwitch;

        let expectations = [
            PinTransaction::set(PinState::High),
            PinTransaction::get_state(PinState::High),
            PinTransaction::get(PinState::High),
        ];

        let pin = PinMock::new(&expectations);

        let mut led = Switch::<_, ActiveHigh>::new(pin);
        led.on().unwrap();

        assert_eq!(true, led.is_on().unwrap());

        let mut pin = led.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
        pin.done();
    }
}

#[allow(clippy::bool_assert_comparison)]
mod active_low_switch {
    use super::*;
    use embedded_hal::digital::InputPin;
    use switch_hal::ActiveLow;

    #[test]
    fn when_on_pin_is_low() {
        let expectations = [
            PinTransaction::set(PinState::Low),
            PinTransaction::get(PinState::Low),
        ];

        let pin = PinMock::new(&expectations);

        let mut led = Switch::<_, ActiveLow>::new(pin);
        led.on().unwrap();

        let mut pin = led.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
        pin.done();
    }

    #[test]
    fn when_off_pin_is_high() {
        let expectations = [
            PinTransaction::set(PinState::High),
            PinTransaction::get(PinState::High),
        ];

        let pin = PinMock::new(&expectations);

        let mut led = Switch::<_, ActiveLow>::new(pin);
        led.off().unwrap();

        let mut pin = led.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
        pin.done();
    }

    #[test]
    fn is_toggleable() {
        use switch_hal::StatefulOutputSwitch;
        
        let expectations = [
            PinTransaction::set(PinState::High),
            PinTransaction::toggle(),
            PinTransaction::get(PinState::Low),
        ];

        let pin = PinMock::new(&expectations);

        let mut led = Switch::<_, ActiveLow>::new(pin);
        led.off().unwrap();

        led.toggle().unwrap();

        let mut pin = led.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
        pin.done();
    }

    #[test]
    fn not_on_when_high() {
        use switch_hal::StatefulOutputSwitch;

        let expectations = [
            PinTransaction::set(PinState::High),
            PinTransaction::get_state(PinState::High),
            PinTransaction::get(PinState::High),
        ];

        let pin = PinMock::new(&expectations);

        let mut led = Switch::<_, ActiveLow>::new(pin);
        led.off().unwrap();

        assert_eq!(false, led.is_on().unwrap());

        let mut pin = led.into_pin();
        assert_eq!(false, pin.is_low().unwrap());
        pin.done();
    }

    #[test]
    fn is_on_when_low() {
        use switch_hal::StatefulOutputSwitch;

        let expectations = [
            PinTransaction::set(PinState::Low),
            PinTransaction::get_state(PinState::Low),
            PinTransaction::get(PinState::Low),
        ];

        let pin = PinMock::new(&expectations);

        let mut led = Switch::<_, ActiveLow>::new(pin);
        led.on().unwrap();

        assert_eq!(true, led.is_on().unwrap());

        let mut pin = led.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
        pin.done();
    }
}
