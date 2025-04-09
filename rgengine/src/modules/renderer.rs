use super::window::WindowManager;

// very much a skeliton
// nedd to add alot more to this
pub struct Renderer {}

impl Renderer {
    pub fn new(window: &mut WindowManager) -> Self {
        gl::load_with(|s| window.window.get_proc_address(s) as *const _);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::SCISSOR_TEST);
        }
        Self {}
    }

    pub fn render_frame(&mut self, window: &mut WindowManager) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
}
