use embedded_hal_mock::eh1::digital::{Mock as PinMock, State as PinState, Transaction as PinTransaction};
use switch_hal::IntoSwitch;

#[allow(clippy::bool_assert_comparison)]
mod output_pin {
    use super::*;

    use embedded_hal::digital::InputPin;
    use switch_hal::OutputSwitch;

    #[test]
    fn active_high() {
        let expectations = [
            PinTransaction::set(PinState::High),
            PinTransaction::get(PinState::High),
        ];

        let pin = PinMock::new(&expectations);

        let mut switch = pin.into_active_high_switch();
        switch.on().unwrap();

        let mut pin = switch.into_pin();
        assert_eq!(true, pin.is_high().unwrap());
        pin.done();
    }

    #[test]
    fn active_low() {
        let expectations = [
            PinTransaction::set(PinState::Low),
            PinTransaction::get(PinState::Low),
        ];

        let pin = PinMock::new(&expectations);

        let mut switch = pin.into_active_low_switch();
        switch.on().unwrap();

        let mut pin = switch.into_pin();
        assert_eq!(true, pin.is_low().unwrap());
        pin.done();
    }
}

#[allow(clippy::bool_assert_comparison)]
mod input_pin {
    use super::*;
    use switch_hal::InputSwitch;

    #[test]
    fn active_high() {
        let expectations = [
            PinTransaction::get(PinState::High),
        ];

        let pin = PinMock::new(&expectations);

        let mut switch = pin.into_active_high_switch();
        assert_eq!(true, switch.is_active().unwrap());
        switch.into_pin().done();
    }

    #[test]
    fn active_low() {
        let expectations = [
            PinTransaction::get(PinState::Low),
        ];

        let pin = PinMock::new(&expectations);

        let mut switch = pin.into_active_low_switch();
        assert_eq!(true, switch.is_active().unwrap());
        switch.into_pin().done();
    }
}

mod wait_pin {
    use super::*;
    use switch_hal::WaitSwitch;

    #[async_std::test]
    async fn active_high() {
        let expectations = [
            PinTransaction::wait_for_state(PinState::High),
        ];

        let pin = PinMock::new(&expectations);

        let mut switch = pin.into_active_high_switch();
        switch.wait_active().await.unwrap();
        switch.into_pin().done();
    }

    #[async_std::test]
    async fn active_low() {
        let expectations = [
            PinTransaction::wait_for_state(PinState::Low),
        ];

        let pin = PinMock::new(&expectations);

        let mut switch = pin.into_active_low_switch();
        switch.wait_active().await.unwrap();
        switch.into_pin().done();
    }
}
