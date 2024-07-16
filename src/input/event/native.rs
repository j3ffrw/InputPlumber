use evdev::AbsoluteAxisCode;

use crate::input::capability::{Capability, Gamepad, GamepadButton};

use super::{evdev::EvdevEvent, value::InputValue};

/// A native event represents an InputPlumber event
#[derive(Debug, Clone)]
pub struct NativeEvent {
    /// The capability of the input event. Target input devices that implement
    /// this capability will be able to emit this event.
    capability: Capability,
    /// Optional source capability of the input event if this event was translated
    /// from one type to another. This can allow downstream target input devices
    /// to have different behavior for events that have been translated from one
    /// type to another.
    source_capability: Option<Capability>,
    /// The value of the input event.
    value: InputValue,
}

impl NativeEvent {
    /// Returns a new [NativeEvent] with the given capability and value
    pub fn new(capability: Capability, value: InputValue) -> NativeEvent {
        NativeEvent {
            capability,
            value,
            source_capability: None,
        }
    }

    /// Returns a new [NativeEvent] with the original un-translated source
    /// capability, the translated capability, and value.
    pub fn new_translated(
        source_capability: Capability,
        capability: Capability,
        value: InputValue,
    ) -> NativeEvent {
        NativeEvent {
            capability,
            source_capability: Some(source_capability),
            value,
        }
    }

    /// Returns the capability that this event implements
    pub fn as_capability(&self) -> Capability {
        self.capability.clone()
    }

    /// Returns the value of this event
    pub fn get_value(&self) -> InputValue {
        self.value.clone()
    }

    /// Returns true if this event is a translated event and has a source
    /// capability defined.
    pub fn is_translated(&self) -> bool {
        self.source_capability.is_some()
    }

    /// Set the source capability of the event if this is a translated event
    pub fn set_source_capability(&mut self, cap: Capability) {
        self.source_capability = Some(cap);
    }

    /// Returns the source capability that this event was translated from
    pub fn get_source_capability(&self) -> Option<Capability> {
        self.source_capability.clone()
    }

    /// Returns whether or not the event is "pressed"
    pub fn pressed(&self) -> bool {
        self.value.pressed()
    }

    pub fn from_evdev_raw(event: EvdevEvent, hat_state: Option<i32>) -> NativeEvent {
        // If this is a Dpad input, figure out with button this event is for
        let capability = if let Some(old_state) = hat_state {
            let axis = AbsoluteAxisCode(event.as_input_event().code());
            let value = event.as_input_event().value();

            match axis {
                AbsoluteAxisCode::ABS_HAT0X => match value {
                    -1 => Capability::Gamepad(Gamepad::Button(GamepadButton::DPadLeft)),
                    1 => Capability::Gamepad(Gamepad::Button(GamepadButton::DPadRight)),
                    0 => match old_state {
                        -1 => Capability::Gamepad(Gamepad::Button(GamepadButton::DPadLeft)),
                        1 => Capability::Gamepad(Gamepad::Button(GamepadButton::DPadRight)),
                        _ => Capability::NotImplemented,
                    },
                    _ => Capability::NotImplemented,
                },
                AbsoluteAxisCode::ABS_HAT0Y => match value {
                    -1 => Capability::Gamepad(Gamepad::Button(GamepadButton::DPadUp)),
                    1 => Capability::Gamepad(Gamepad::Button(GamepadButton::DPadDown)),
                    0 => match old_state {
                        -1 => Capability::Gamepad(Gamepad::Button(GamepadButton::DPadUp)),
                        1 => Capability::Gamepad(Gamepad::Button(GamepadButton::DPadDown)),
                        _ => Capability::NotImplemented,
                    },
                    _ => Capability::NotImplemented,
                },

                _ => Capability::NotImplemented,
            }
        } else {
            event.as_capability()
        };

        let value = event.get_value();

        NativeEvent {
            capability,
            value,
            source_capability: None,
        }
    }
}

impl From<EvdevEvent> for NativeEvent {
    /// Convert the [EvdevEvent] into a [NativeEvent]
    fn from(item: EvdevEvent) -> Self {
        let capability = item.as_capability();
        let value = item.get_value();
        NativeEvent {
            capability,
            value,
            source_capability: None,
        }
    }
}
