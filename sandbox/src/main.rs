use rgengine::application::Application;
use rgengine::engine_run;

#[derive(Default)]
struct Sandbox;

impl Sandbox {
    fn new() -> Self {
        Self
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {}
}

engine_run!(Sandbox);
