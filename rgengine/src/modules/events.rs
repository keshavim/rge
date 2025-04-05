use std::{any::Any, collections::HashMap, sync::Mutex};

use glfw::{Key, MouseButton};

use crate::log::rge_engine_warn;

use bitmask_enum::bitmask;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EventType {
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

///this is a bit array
#[bitmask(u8)]
pub enum EventCatagory {
    Application,
    Input,
    Keyboard,
    Mouse,
    MouseButton,
}

///basic trait that has all the function needed for events
pub trait Event: Send + Any + Sync {
    fn get_name(&self) -> &'static str;
    fn get_type(&self) -> EventType;
    fn get_catagory(&self) -> u8;
    fn to_string(&self) -> String;
    fn is_in_catagory(&self, catagory: EventCatagory) -> bool {
        catagory.bits() & self.get_catagory() != 0
    }
    fn set_handled(&mut self, handled: bool);
    fn is_handled(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}

// Dispatcher
pub struct EventDispatcher {
    handlers: Mutex<HashMap<EventType, Box<dyn EventHandler + Send + Sync>>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Mutex::new(HashMap::new()),
        }
    }

    pub fn register<E: Event + 'static>(
        &self,
        event_type: EventType,
        handler: impl FnMut(&E) + Send + Sync + 'static,
    ) {
        let wrapper = Box::new(EventHandlerWrapper::new(handler));
        let mut guard = self.handlers.lock().unwrap_or_else(|p| {
            rge_engine_warn!("Event handler poisined");
            p.into_inner()
        });
        guard.insert(event_type, wrapper);
    }

    pub fn dispatch(&self, event: &dyn Event) {
        if let Some(handler) = self.handlers.lock().unwrap().get_mut(&event.get_type()) {
            handler.handle(event);
        }
    }
}

// Handler infrastructure
trait EventHandler {
    fn handle(&mut self, event: &dyn Event);
}

struct EventHandlerWrapper<F, E> {
    handler: F,
    _marker: std::marker::PhantomData<E>,
}

impl<F, E> EventHandlerWrapper<F, E>
where
    F: FnMut(&E),
    E: Event + 'static,
{
    fn new(handler: F) -> Self {
        Self {
            handler,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<F, E> EventHandler for EventHandlerWrapper<F, E>
where
    F: FnMut(&E),
    E: Event + 'static,
{
    fn handle(&mut self, event: &dyn Event) {
        if let Some(e) = event.as_any().downcast_ref::<E>() {
            (self.handler)(e);
        }
    }
}

use paste::paste;
///create a event
///events are structs that have a type and a catagory from the avalible
///enum options
macro_rules! create_event {
    ($event_type:ident { $( $field_name:ident : $field_type:ty ),* }, [$($event_catagory:ident),+]) => {
        paste! {
            #[derive(Debug)]
            pub struct [<$event_type Event>]{
                pub handled: bool,
                $(pub $field_name: $field_type,)*
            }


            impl [<$event_type Event>]
            {
                pub fn new($( $field_name: $field_type ),*) -> Self {
                    Self {
                        handled : false,
                        $( $field_name, )*
                    }
                }
            }

            impl Event for [<$event_type Event>]
            {
                fn get_name(&self) -> &'static str{
                    stringify!([<$event_type Event>])
                }
                fn get_type(&self) -> EventType {
                    EventType::$event_type
                }
                fn get_catagory(&self) -> u8 {
                    0 $( | EventCatagory::$event_catagory.bits())+
                }
                fn to_string(&self) -> String{
                    let fields = vec![
                            $(format!("{}: {:?}", stringify!($field_name), self.$field_name)),*
                        ];
                    format!("{} {{ {} }}", stringify!([<$event_type Event>]), fields.join(", "))
                }
                fn set_handled(&mut self, handled:bool){
                    self.handled = handled;
                }
                fn is_handled(&self) -> bool{
                    self.handled
                }
                fn as_any(&self) -> &dyn Any {
                    self
                }
            }
        }
    };
//this makes things simpler
    ($event_type:ident, [$($event_catagory:ident),+]) => {
        paste!{
            #[derive(Debug)]
            pub struct [<$event_type Event>]
            {
                pub handled: bool,
            }
            impl [<$event_type Event>]
            {
                pub fn new() -> Self {
                    Self {
                        handled : false,
                    }
                }
            }
            impl Default for [<$event_type Event>]
            {
                fn default() -> Self {
                    Self {
                        handled: false,
                    }
                }
            }

             impl Event for [<$event_type Event>]
            {
                fn get_name(&self) -> &'static str{
                    stringify!( [<$event_type Event>] )
                }
                fn get_type(&self) -> EventType {
                    EventType::$event_type
                }
                fn get_catagory(&self) -> u8 {
                    0 $( | EventCatagory::$event_catagory.bits())+
                }
                fn to_string(&self) -> String{
                   stringify!([<$event_type Event>]).to_string()
                }
                fn set_handled(&mut self, handled:bool){
                    self.handled = handled;
                }
                fn is_handled(&self) -> bool{
                    self.handled
                }
                fn as_any(&self) -> &dyn Any {
                    self
                }
            }
        }
    };
}
create_event!(
    KeyPressed {
        key_code: Key,
        repeat: bool
    },
    [Keyboard, Input]
);

create_event!(KeyReleased { key_code: Key }, [Keyboard, Input]);

create_event!(
    MouseButtonPressed {
        mouse_code: MouseButton
    },
    [MouseButton, Mouse, Input]
);
create_event!(
    MouseButtonReleased {
        mouse_code: MouseButton
    },
    [MouseButton, Mouse, Input]
);
create_event!(
    MouseScrolled {
        x_offset: f64,
        y_offset: f64
    },
    [Mouse, Input]
);
create_event!(
    MouseMoved {
        mouse_x: f64,
        mouse_y: f64
    },
    [Mouse, Input]
);

create_event!(
    WindowResize {
        width: u32,
        height: u32
    },
    [Application]
);
create_event!(WindowClose, [Application]);
create_event!(WindowLostFocus, [Application]);
create_event!(WindowFocus, [Application]);
create_event!(WindowMoved, [Application]);

create_event!(AppTick, [Application]);
create_event!(AppRender, [Application]);
create_event!(AppUpdate, [Application]);
