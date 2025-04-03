use crate::events::{Event, EventCatagory};
use crate::log::*;

use super::events::KeyPressedEvent;

pub trait Application {
    fn run(&self) {
        let a = KeyPressedEvent::new(1, false);

        if a.is_in_catagory(EventCatagory::Keyboard) {
            rge_engine_trace!("runing {}", a.to_string());
        }
        if a.is_in_catagory(EventCatagory::Input) {
            rge_engine_trace!("runing {}", a.to_string());
        }
    }
}
