use std::{cell::RefCell, rc::Rc, time::Instant};

use crate::{layers::Layer, window::WindowManager};
use glfw::MouseButton;
use imgui::Context;
use imgui_glfw_rs::ImguiGLFW;

use super::events::{EventCategory, RGEvent};

//will remove glfw support pack age and ave it builtin later

//have remove imgui_glfw_rs and make it on my own
pub struct ImGuiLayer {
    imgui: Context,
    imgui_glfw: ImguiGLFW,
    last_frame: Instant,
    id: usize,
    mouse_press: [bool; 5],
    window: Rc<RefCell<WindowManager>>,
}

//window manager may be better as a shared referance
impl ImGuiLayer {
    pub fn new(window: &Rc<RefCell<WindowManager>>, id: usize) -> Self {
        let mut imgui = Context::create();
        let mut window = Rc::clone(window);
        let imgui_glfw = ImguiGLFW::new(&mut imgui, window.borrow_mut().native_window_mut());

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
            mouse_press: [false; 5],
            window,
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

    fn on_render(&mut self) {
        let mut w = self.window.borrow_mut();
        // Start new frame
        let ui = self
            .imgui_glfw
            .frame(w.native_window_mut(), &mut self.imgui);

        // Demo window
        ui.show_demo_window(&mut true);

        // Render commands will be handled by the renderer
        self.imgui_glfw.draw(ui, w.native_window_mut());
    }
    //will make this work with my events
    fn on_event(&mut self, event: &RGEvent) {
        if !event.is_in_category(EventCategory::Input) {
            return;
        }
        match *event {
            RGEvent::MouseButtonPressed(e) => {
                let index = match e.button {
                    MouseButton::Button1 => 0,
                    MouseButton::Button2 => 1,
                    MouseButton::Button3 => 2,
                    MouseButton::Button4 => 3,
                    MouseButton::Button5 => 4,
                    _ => 0,
                };
                self.mouse_press[index] = true;
                self.imgui.io_mut().mouse_down = self.mouse_press;
            }

            RGEvent::MouseButtonReleased(e) => {
                let index = match e.button {
                    MouseButton::Button1 => 0,
                    MouseButton::Button2 => 1,
                    MouseButton::Button3 => 2,
                    MouseButton::Button4 => 3,
                    MouseButton::Button5 => 4,
                    _ => 0,
                };
                self.mouse_press[index] = false;
                self.imgui.io_mut().mouse_down = self.mouse_press;
            }
            RGEvent::MouseMoved(e) => {
                self.imgui.io_mut().mouse_pos = [e.x as f32, e.y as f32];
            }
            RGEvent::MouseScrolled(e) => {
                self.imgui.io_mut().mouse_wheel = e.y_offset as f32;
            }
            //WindowEvent::Char(character) => {
            //    imgui.io_mut().add_input_character(character);
            //}
            RGEvent::KeyPressed(e) => {
                self.imgui.io_mut().keys_down[e.key as usize] = true;
            }
            RGEvent::KeyReleased(e) => {
                self.imgui.io_mut().keys_down[e.key as usize] = false;
            }
            RGEvent::KeyTyped(e) => {
                self.imgui.io_mut().add_input_character(e.key);
            }
            _ => {}
        }
        //self.imgui_glfw.handle_event(&mut self.imgui, event);
    }
    fn get_id(&self) -> usize {
        self.id
    }
}
