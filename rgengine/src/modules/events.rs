use bitflag::bitflag;
use glfw::{Action, Key, Modifiers, MouseButton, Scancode, WindowEvent};

use super::engine::GameEngine;

//might need to change how all this works
////need a much better way of comparing event types
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum EventType {
    Unknown,
    WindowMoved { x: i32, y: i32 },
    WindowResize { width: i32, height: i32 },
    WindowClose,
    WindowFocus,
    WindowLostFocus,
    KeyPressed { key: Key, repeat: bool },
    KeyReleased { key: Key },
    MouseButtonPressed { button: MouseButton },
    MouseButtonReleased { button: MouseButton },
    MouseMoved { x: f64, y: f64 },
    MouseScrolled { x_offset: f64, y_offset: f64 },
}

#[bitflag(u8)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum EventCategory {
    Unknown = 0,
    Engine = 1,
    Input = 1 << 1,
    Keyboard = 1 << 2,
    Mouse = 1 << 3,
    MouseButton = 1 << 4,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Event {
    pub event_type: EventType,
    pub category: EventCategory,
    pub handled: bool,
}

impl Event {
    fn dummy() -> Self {
        Self {
            event_type: EventType::Unknown,
            category: EventCategory::Unknown,
            handled: false,
        }
    }
}

//converts a glfw event to my events
impl From<&WindowEvent> for Event {
    fn from(glfw_event: &WindowEvent) -> Self {
        let (event_type, category) = match *glfw_event {
            WindowEvent::Pos(x, y) => (EventType::WindowMoved { x, y }, EventCategory::Engine),

            WindowEvent::Size(width, height) => (
                EventType::WindowResize { width, height },
                EventCategory::Engine,
            ),

            WindowEvent::Close => (EventType::WindowClose, EventCategory::Engine),

            WindowEvent::Focus(focused) => (
                if focused {
                    EventType::WindowFocus
                } else {
                    EventType::WindowLostFocus
                },
                EventCategory::Engine,
            ),

            WindowEvent::Key(key, _, action, _) => match action {
                glfw::Action::Press => (
                    EventType::KeyPressed { key, repeat: false },
                    EventCategory::Keyboard | EventCategory::Input,
                ),
                glfw::Action::Release => (
                    EventType::KeyReleased { key },
                    EventCategory::Keyboard | EventCategory::Input,
                ),
                glfw::Action::Repeat => (
                    EventType::KeyPressed { key, repeat: true },
                    EventCategory::Keyboard | EventCategory::Input,
                ),
            },

            WindowEvent::MouseButton(button, action, _) => match action {
                glfw::Action::Press | glfw::Action::Repeat => (
                    EventType::MouseButtonPressed { button },
                    EventCategory::MouseButton | EventCategory::Mouse | EventCategory::Input,
                ),
                glfw::Action::Release => (
                    EventType::MouseButtonReleased { button },
                    EventCategory::MouseButton | EventCategory::Mouse | EventCategory::Input,
                ),
            },

            WindowEvent::CursorPos(x, y) => (
                EventType::MouseMoved { x, y },
                EventCategory::Mouse | EventCategory::Input,
            ),

            WindowEvent::Scroll(x_offset, y_offset) => (
                EventType::MouseScrolled { x_offset, y_offset },
                EventCategory::Mouse | EventCategory::Input,
            ),

            _ => return Event::dummy(),
        };
        Event {
            event_type,
            handled: false,
            category,
        }
    }
}

// this whole thing needs masive reworks
pub struct EventSystem {
    event_queue: Vec<glfw::WindowEvent>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            event_queue: Vec::with_capacity(128),
        }
    }

    pub fn dispatch<F: FnMut(Event) -> bool>(event: &mut Event, mut func: F) {
        //self.event_queue.push(event);

        event.handled = func(event.clone());
    }

    pub fn process_events(&mut self, engine: &mut GameEngine) {
        while let Some(event) = self.event_queue.pop() {
            match event {
                // Handle specific event types
                _ => {}
            }
        }
    }
}
