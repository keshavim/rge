use glfw::{PWindow, Window, WindowEvent};
use imgui::{BackendFlags, Context, Io, Key};
use imgui_opengl_renderer::Renderer;
use std::cell::RefCell;
use std::rc::Rc;

use crate::log::rge_engine_error;
use crate::{rge_info, rge_warn};

use super::layer::Layer;
use super::log::rge_engine_info;

pub struct ImGuiLayer {
    debugname: String,
    id: usize,
    imgui: Option<Context>,
    renderer: Option<Renderer>,
    window: Rc<RefCell<PWindow>>,
    platform: ImguiGlfw,
}

// Custom GLFW input handler
struct ImguiGlfw {
    mouse_pressed: [bool; 5],
    mouse_wheel: f32,
    keys_down: [bool; 512], // GLFW keycode range
}

impl ImguiGlfw {
    fn new() -> Self {
        Self {
            mouse_pressed: [false; 5],
            mouse_wheel: 0.0,
            keys_down: [false; 512],
        }
    }

    fn prepare_frame(&mut self, io: &mut Io, window: &Window) {
        let (win_w, win_h) = window.get_size();
        let (fb_w, fb_h) = window.get_framebuffer_size();

        io.display_size = [win_w as f32, win_h as f32];
        io.display_framebuffer_scale = [
            (fb_w as f32) / (win_w as f32).max(1.0),
            (fb_h as f32) / (win_h as f32).max(1.0),
        ];

        // Mouse position
        let (x, y) = window.get_cursor_pos();
        io.mouse_pos = [x as f32, y as f32];
        io.mouse_down = self.mouse_pressed;
        io.mouse_wheel = self.mouse_wheel;
        self.mouse_wheel = 0.0;

        // Keyboard state
        for (i, &down) in self.keys_down.iter().enumerate() {
            io.keys_down[i] = down;
        }
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseButton(button, action, _) => {
                if let Some(idx) = match button {
                    glfw::MouseButtonLeft => Some(0),
                    glfw::MouseButtonRight => Some(1),
                    glfw::MouseButtonMiddle => Some(2),
                    _ => None,
                } {
                    self.mouse_pressed[idx] = *action == glfw::Action::Press;
                }
            }
            WindowEvent::Scroll(_, yoffset) => {
                self.mouse_wheel = *yoffset as f32;
            }
            //WindowEvent::Key(key, _, action, _) => {
            //    if let Some(keycode) = map_key(*key) {
            //        self.keys_down[keycode as usize] = *action != glfw::Action::Release;
            //    }
            //}
            //WindowEvent::Char(codepoint) => {
            //    // Handle text input (needs proper UTF-8 handling)
            //    if *codepoint <= std::char::MAX as u32 {
            //        if let Some(c) = std::char::from_u32(*codepoint) {
            //            // Add to ImGui IO's input queue
            //        }
            //    }
            //}
            _ => {}
        }
    }
}

impl ImGuiLayer {
    pub fn new(window: Rc<RefCell<PWindow>>) -> Self {
        Self {
            debugname: "ImGuiLayer".to_string(),
            id: 0,
            imgui: None,
            renderer: None,
            window,
            platform: ImguiGlfw::new(),
        }
    }
    fn recreate_renderer(&mut self) {
        if let Some(ctx) = &mut self.imgui {
            self.renderer = Some(Renderer::new(ctx, |s| {
                self.window.borrow_mut().get_proc_address(s) as *const _
            }));
        }
    }
}

impl Layer for ImGuiLayer {
    fn attach(&mut self) {
        // Initialize ImGui context
        let ctx = self.imgui.get_or_insert_with(Context::create);
        ctx.style_mut().use_dark_colors();

        // Initialize renderer
        self.renderer = Some(Renderer::new(ctx, |s| {
            self.window.borrow_mut().get_proc_address(s) as *const _
        }));

        // Setup key mappings
        let io = ctx.io_mut();
        io.backend_flags |= BackendFlags::HAS_MOUSE_CURSORS | BackendFlags::HAS_SET_MOUSE_POS;
        set_imgui_keymap(io);
    }

    fn detach(&mut self) {
        // Cleanup resources
        self.renderer = None;
    }

    fn update(&mut self) {
        if let Some(ctx) = &mut self.imgui {
            // Prepare new frame
            self.platform
                .prepare_frame(ctx.io_mut(), &self.window.borrow());

            rge_engine_info!("frame prepared");
            // Build UI
            let ui = ctx.frame();
            ui.window("Demo").build(|| {
                ui.text("Hello from ImGuiLayer!");
            });
            rge_engine_info!("ui built");

            if let Some(renderer) = &mut self.renderer {
                // Safe render call - handles buffer validation internally
                renderer.render(ctx);
            }
            rge_engine_info!("rendered");
        }
    }

    fn event(&mut self, e: &dyn super::events::Event) {
        //if let Some(event) = e.downcast_ref::<WindowEvent>() {
        //    self.platform.handle_event(event);
        //}
    }

    fn id(&self) -> u32 {
        self.id as u32
    }
}

//temporary
fn set_imgui_keymap(io: &mut imgui::Io) {
    // Map GLFW keys to ImGui keys
    io.key_map[Key::Tab as usize] = glfw::Key::Tab as u32;
    io.key_map[Key::LeftArrow as usize] = glfw::Key::Left as u32;
    io.key_map[Key::RightArrow as usize] = glfw::Key::Right as u32;
    io.key_map[Key::UpArrow as usize] = glfw::Key::Up as u32;
    io.key_map[Key::DownArrow as usize] = glfw::Key::Down as u32;
    io.key_map[Key::PageUp as usize] = glfw::Key::PageUp as u32;
    io.key_map[Key::PageDown as usize] = glfw::Key::PageDown as u32;
    io.key_map[Key::Home as usize] = glfw::Key::Home as u32;
    io.key_map[Key::End as usize] = glfw::Key::End as u32;
    io.key_map[Key::Insert as usize] = glfw::Key::Insert as u32;
    io.key_map[Key::Delete as usize] = glfw::Key::Delete as u32;
    io.key_map[Key::Backspace as usize] = glfw::Key::Backspace as u32;
    io.key_map[Key::Space as usize] = glfw::Key::Space as u32;
    io.key_map[Key::Enter as usize] = glfw::Key::Enter as u32;
    io.key_map[Key::Escape as usize] = glfw::Key::Escape as u32;
    io.key_map[Key::A as usize] = glfw::Key::A as u32;
    io.key_map[Key::C as usize] = glfw::Key::C as u32;
    io.key_map[Key::V as usize] = glfw::Key::V as u32;
    io.key_map[Key::X as usize] = glfw::Key::X as u32;
    io.key_map[Key::Y as usize] = glfw::Key::Y as u32;
    io.key_map[Key::Z as usize] = glfw::Key::Z as u32;
}
