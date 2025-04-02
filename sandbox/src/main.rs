use rgengine::application::Application;
use rgengine::engine_run;

#[derive(Default)]
struct Sandbox;

impl Application for Sandbox {}
impl Drop for Sandbox {
    fn drop(&mut self) {}
}

engine_run!(Sandbox);
