use crate::events::get_type_name;

use super::{
    events::{Event, KeyPressedEvent},
    log::rge_engine_info,
    window::{Window, WindowProps, X11Window},
};

pub trait Application {
    fn run(&self) {
        let key: &dyn Event = &KeyPressedEvent::new(1, false);
        let name = get_type_name::<KeyPressedEvent>();

        rge_engine_info!(
            "{} = {} == {}",
            key.get_name(),
            name,
            name == key.get_name()
        )
        //let mut win = X11Window::new(WindowProps::default());
        //
        //while !win.should_close() {
        //    win.update();
        //}
    }
}
