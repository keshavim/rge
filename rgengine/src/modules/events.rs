use bitmask_enum::bitmask;

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
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
pub trait Event {
    fn get_name(&self) -> &'static str;
    fn get_type(&self) -> EventType;
    fn get_catagory(&self) -> u8;
    fn to_string(&self) -> String;
    fn is_in_catagory(&self, catagory: EventCatagory) -> bool {
        catagory.bits() & self.get_catagory() != 0
    }
    fn set_handled(&mut self, handled: bool);
    fn is_handled(&self) -> bool;
}

pub fn get_type_name<T: Event>() -> &'static str {
    let fullname = std::any::type_name::<T>();

    match fullname.rsplit("::").next() {
        Some(name) => name,
        None => fullname,
    }
}

//need to test this out, not sure if it works
#[derive(Debug)]
pub struct EventDispatcher<'a, E: Event> {
    event: &'a mut E,
}
impl<'a, E: Event> EventDispatcher<'a, E> {
    pub fn new(event: &'a mut E) -> Self {
        Self { event }
    }
    // Dispatch method
    pub fn dispatch<T, F>(&mut self, func: F) -> bool
    where
        T: Event,
        F: FnOnce(&mut T) -> bool,
    {
        if self.event.get_name() == get_type_name::<T>() {
            let event = unsafe { &mut *(self.event as *mut E as *mut T) }; // Downcasting
            let handled = func(event);
            self.event.set_handled(handled);
            return true;
        }
        false
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
            }
        }
    };
}

use glfw::{Key, MouseButton};

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
