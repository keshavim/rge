use std::any::Any;

use glfw::{Action, Key, Modifiers, MouseButton, Scancode, WindowEvent};

use paste::paste;
//might need to change how all this works
////need a much better way of comparing event types

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum EventType {
    WindowMoved,
    WindowResize,
    WindowClose,
    WindowFocus,
    WindowLostFocus,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
    Unknown,
}

bitflags::bitflags! {
#[derive(Clone, Copy, Debug)]
    pub struct EventCategory: u8{
    const Unknown = 0;
    const Engine = 1;
    const Input = 1 << 1;
    const Keyboard = 1 << 2;
    const Mouse = 1 << 3;
    const MouseButton = 1 << 4;
    }
}
#[macro_export]
macro_rules! rgevent {
    ($name:ident $(, $field_name:ident)*) => {

            RGEvent::$name($name::new($($field_name),*))
    };
}

#[derive(Debug)]
pub enum RGEvent {
    WindowClose(WindowClose),
    WindowFocus(WindowFocus),
    WindowLostFocus(WindowLostFocus),
    WindowMoved(WindowMoved),
    WindowResize(WindowResize),
    KeyPressed(KeyPressed),
    KeyReleased(KeyReleased),
    MouseButtonPressed(MouseButtonPressed),
    MouseButtonReleased(MouseButtonReleased),
    MouseMoved(MouseMoved),
    MouseScrolled(MouseScrolled),
}

macro_rules! makeFn {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> $type {
            match self {
                RGEvent::WindowClose(event) => event.$name(),
                RGEvent::WindowFocus(event) => event.$name(),
                RGEvent::WindowLostFocus(event) => event.$name(),
                RGEvent::WindowMoved(event) => event.$name(),
                RGEvent::WindowResize(event) => event.$name(),
                RGEvent::KeyPressed(event) => event.$name(),
                RGEvent::KeyReleased(event) => event.$name(),
                RGEvent::MouseButtonPressed(event) => event.$name(),
                RGEvent::MouseButtonReleased(event) => event.$name(),
                RGEvent::MouseMoved(event) => event.$name(),
                RGEvent::MouseScrolled(event) => event.$name(),
            }
        }
    };
}

impl RGEvent {
    makeFn!(get_type, EventType);
    makeFn!(get_category, EventCategory);
    makeFn!(is_handled, bool);

    pub fn is_in_category(&self, category: EventCategory) -> bool {
        self.get_category().contains(category)
    }
}

macro_rules! create_event_struct {
    ($event_type:ident, $event_category:expr $(, $field_name:ident: $field_type:ty)*) => {
        paste! {
            #[derive(Debug, Clone, Copy)]
            pub struct $event_type {
                pub handled: bool,
                $(pub $field_name: $field_type),*
            }


            impl $event_type {
                /// Constructor for the event struct
                pub fn new($($field_name: $field_type),*) -> Self {
                    Self {
                        handled: false,
                        $($field_name),*
                    }
                }
                pub fn get_type(&self) -> EventType {
                    EventType::$event_type
                }

                pub fn get_category(&self) -> EventCategory {
                    $event_category
                }

                pub fn to_string(&self) -> String {
                    format!("{:?}", self.get_type())
                }

                pub fn is_in_category(&self, category: EventCategory) -> bool {
                    self.get_category().contains(category)
                }

                pub fn is_handled(&self) -> bool{
                    self.handled
                }
                pub fn set_handled(&mut self, b: bool){
                    self.handled = b;
                }
            }
        }
    };
}

create_event_struct!(WindowClose, EventCategory::Engine);
create_event_struct!(WindowFocus, EventCategory::Engine);
create_event_struct!(WindowLostFocus, EventCategory::Engine);
create_event_struct!(WindowMoved, EventCategory::Engine, x:i32, y:i32);
create_event_struct!(WindowResize, EventCategory::Engine, width:i32, height:i32);

create_event_struct!(KeyPressed, EventCategory::Keyboard | EventCategory::Input, key:glfw::Key, repeat:bool);
create_event_struct!(KeyReleased,EventCategory::Keyboard | EventCategory::Input, key:glfw::Key);
create_event_struct!(MouseButtonPressed,EventCategory::MouseButton | EventCategory::Input | EventCategory::Mouse, button:glfw::MouseButton);
create_event_struct!(MouseButtonReleased,EventCategory::MouseButton | EventCategory::Input | EventCategory::Mouse, button:glfw::MouseButton);

create_event_struct!(MouseMoved, EventCategory::Input | EventCategory::Mouse, x:f64, y:f64);
create_event_struct!(MouseScrolled, EventCategory::Input | EventCategory::Mouse,x_offset:f64, y_offset:f64);
