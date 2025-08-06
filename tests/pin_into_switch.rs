use switch_hal::mock::Pin;
use embedded_hal::digital::PinState;
use switch_hal::IntoSwitch;

#[allow(clippy::bool_assert_comparison)]
mod output_pin {
    use super::*;

    use embedded_hal::digital::InputPin;
    use switch_hal::OutputSwitch;

    #[test]
    fn active_high() {
        let pin = Pin::new();
        let mut switch = pin.into_active_high_switch();
        switch.on().unwrap();

        let mut pin = switch.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
    }

    #[test]
    fn active_low() {
        let pin = Pin::new();
        let mut switch = pin.into_active_low_switch();
        switch.on().unwrap();

        let mut pin = switch.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
    }
}

#[allow(clippy::bool_assert_comparison)]
mod input_pin {
    use super::*;
    use switch_hal::InputSwitch;

    #[test]
    fn active_high() {
        let pin = Pin::with_state(PinState::High);
        let mut switch = pin.into_active_high_switch();
        assert_eq!(true, switch.is_active().unwrap());
    }

    #[test]
    fn active_low() {
        let pin = Pin::with_state(PinState::Low);
        let mut switch = pin.into_active_low_switch();
        assert_eq!(true, switch.is_active().unwrap());
    }
}
