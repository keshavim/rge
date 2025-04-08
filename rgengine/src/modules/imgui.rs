use std::time::Instant;

use crate::{layers::Layer, window::WindowManager};
use imgui::Context;
use imgui_glfw_rs::ImguiGLFW;

//have remove imgui_glfw_rs and make it on my own
pub struct ImGuiLayer {
    imgui: Context,
    imgui_glfw: ImguiGLFW,
    last_frame: Instant,
    id: usize,
}

//window manager may be better as a shared referance
impl ImGuiLayer {
    pub fn new(window: &mut WindowManager, id: usize) -> Self {
        let mut imgui = Context::create();
        let imgui_glfw = ImguiGLFW::new(&mut imgui, window.native_window());

        // Configure ImGui
        imgui.set_ini_filename(None);
        imgui.style_mut().use_dark_colors();
        imgui.io_mut().backend_flags |= imgui::BackendFlags::HAS_MOUSE_CURSORS;
        imgui.io_mut().backend_flags |= imgui::BackendFlags::HAS_SET_MOUSE_POS;

        Self {
            imgui,
            imgui_glfw,
            last_frame: Instant::now(),
            id,
        }
    }

    pub fn set_ini_filename<T: Into<Option<imgui::ImString>>>(&mut self, ini_filename: T) {
        self.imgui.set_ini_filename(ini_filename)
    }
}

impl Layer for ImGuiLayer {
    fn on_update(&mut self, _dt: f32) {
        // Update logic if needed
    }

    fn on_render(&mut self, window: &mut WindowManager) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;

        // Pass delta_time to ImGui
        self.imgui.io_mut().delta_time = delta_time;

        self.imgui.io_mut().display_size = [window.data.width as f32, window.data.height as f32];

        // Start new frame
        let ui = self
            .imgui_glfw
            .frame(window.native_window(), &mut self.imgui);

        // Demo window
        ui.show_demo_window(&mut true);

        // Render commands will be handled by the renderer
        self.imgui_glfw.draw(ui, window.native_window());
    }
    //will make this work with my events
    fn on_event(&mut self, event: &glfw::WindowEvent) {
        self.imgui_glfw.handle_event(&mut self.imgui, event);
    }
    fn get_id(&self) -> usize {
        self.id
    }
}
