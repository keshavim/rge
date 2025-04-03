use super::window::{Window, WindowProps, X11Window};

pub trait Application {
    fn run(&self) {
        let mut win = X11Window::new(WindowProps::default());

        while !win.should_close() {
            win.update();
        }
    }
}
