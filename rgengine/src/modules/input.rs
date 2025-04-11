use std::{cell::RefCell, rc::Rc};

use glfw::WindowMode;

use super::{
    engine::GameEngine,
    window::{self, WindowManager},
};

pub struct InputHandler {
    window: Rc<RefCell<WindowManager>>,
}

impl InputHandler {
    pub fn new(window: &Rc<RefCell<WindowManager>>) -> Self {
        Self {
            window: Rc::clone(window),
        }
    }

    pub fn is_key_pressed(&self, keycode: glfw::Key) -> bool {
        let window = self.window.borrow();

        let act = window.native_window().get_key(keycode);
        act == glfw::Action::Press || act == glfw::Action::Repeat
    }

    pub fn is_mouse_button_pressed(&self, button: glfw::MouseButton) -> bool {
        let window = self.window.borrow();

        let act = window.native_window().get_mouse_button(button);
        act == glfw::Action::Press
    }
    pub fn get_mouse_pos(&self) -> (f64, f64) {
        let window = self.window.borrow();

        window.native_window().get_cursor_pos()
    }
    pub fn get_mouse_x(&self) -> f64 {
        self.get_mouse_pos().0
    }
    pub fn get_mouse_y(&self) -> f64 {
        self.get_mouse_pos().1
    }
}
