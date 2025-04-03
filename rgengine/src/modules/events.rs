use bitmask_enum::bitmask;

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
    fn get_type(&self) -> EventType;
    fn get_catagory(&self) -> u8;
    fn to_string(&self) -> String;
    fn is_in_catagory(&self, catagory: EventCatagory) -> bool {
        catagory.bits() & self.get_catagory() != 0
    }
    fn set_handled(&mut self, handled: bool);
    fn is_handled(&self) -> bool;
}

//need to test this out, not sure if it works
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
        let event = unsafe { &mut *(self.event as *mut E as *mut T) }; // Downcasting
        let handled = func(event);
        self.event.set_handled(handled);
        handled
    }
}

///create a event
///events are structs that have a type and a catagory from the avalible
///enum options
macro_rules! create_event {
    ($struct_name:ident { $( $field_name:ident : $field_type:ty ),* }, $event_type:ident [$($event_catagory:ident),+]) => {
        #[derive(Debug)]
        pub struct $struct_name {
            pub handled: bool,
            $(pub $field_name: $field_type,)*
        }

        impl $struct_name {
            pub fn new($( $field_name: $field_type ),*) -> Self {
                Self {
                    handled : false,
                    $( $field_name, )*
                }
            }
        }

        impl Event for $struct_name {
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
                format!("{} {{ {} }}", stringify!($struct_name), fields.join(", "))
            }
            fn set_handled(&mut self, handled:bool){
                self.handled = handled;
            }
            fn is_handled(&self) -> bool{
                self.handled
            }
        }
    };
//this makes things simpler
    ($struct_name:ident, $event_type:ident [$($event_catagory:ident),+]) => {
        #[derive(Debug)]
        pub struct $struct_name {
            pub handled: bool,
        }
        impl $struct_name {
            pub fn new() -> Self {
                Self {
                    handled : false,
                }
            }
        }
        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    handled: false,
                }
            }
        }

        impl Event for $struct_name {
            fn get_type(&self) -> EventType {
                EventType::$event_type
            }
            fn get_catagory(&self) -> u8 {
                0 $( | EventCatagory::$event_catagory.bits())+
            }
            fn to_string(&self) -> String{
                stringify!($struct_name).to_string()
            }
            fn set_handled(&mut self, handled:bool){
                self.handled = handled;
            }
            fn is_handled(&self) -> bool{
                self.handled
            }
        }
    };
}

create_event!(
    KeyPressedEvent {
        key_code: u8,
        repeat: bool
    },
    KeyPressed
    [Keyboard, Input]
);
create_event!(KeyReleasedEvent { key_code: u8 }, KeyReleased [Keyboard, Input]);

create_event!(
    MouseButtonPressedEvent {
        mouse_code: u8,
        repeat: bool
    },
    MouseButtonPressed
    [MouseButton, Mouse, Input]
);
create_event!(MouseButtonReleasedEvent { mouse_code: u8 }, MouseButtonReleased [MouseButton, Mouse, Input]);
create_event!(MouseScrolledEvent { x_offset: f32, y_offset: f32 }, MouseScrolled [ Mouse, Input]);
create_event!(MouseMovedEvent { mouse_x: f32, mouse_y: f32 }, MouseMoved [ Mouse, Input]);

create_event!(
    WindowResizeEvent {
        width: f32,
        height: f32
    },
    WindowResize[Application]
);
create_event!(WindowCloseEvent, WindowClose[Application]);
create_event!(WindowLostFocusEvent, WindowLostFocus[Application]);
create_event!(WindowFocusEvent, WindowFocus[Application]);
create_event!(WindowMovedEvent, WindowMoved[Application]);

create_event!(AppTickEvent, AppTick[Application]);
create_event!(AppRenderEvent, AppRender[Application]);
create_event!(AppUpdateEvent, AppUpdate[Application]);
