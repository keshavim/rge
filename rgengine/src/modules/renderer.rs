use std::{cell::RefCell, rc::Rc};

use glfw::Context;

extern crate gl;
use self::gl::types::*;

use super::window::WindowManager;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;
use std::sync::mpsc::Receiver;

const vertexShaderSource: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const fragmentShaderSource: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

// very much a skeliton
// nedd to add alot more to this
pub struct Renderer {
    window: Rc<RefCell<WindowManager>>,
    shaderProgram: u32,
    VAO: u32,
}

impl Renderer {
    pub fn new(window: &Rc<RefCell<WindowManager>>) -> Self {
        let window = Rc::clone(window);
        gl::load_with(|s| window.borrow_mut().native_window_mut().get_proc_address(s) as *const _);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::SCISSOR_TEST);
        }
        let (shaderProgram, VAO) = unsafe {
            // build and compile our shader program
            // ------------------------------------
            // vertex shader
            let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertexShaderSource.as_bytes()).unwrap();
            gl::ShaderSource(vertexShader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertexShader);

            // check for shader compile errors
            let mut success = gl::FALSE as GLint;
            let mut infoLog = Vec::with_capacity(512);
            gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    vertexShader,
                    512,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    str::from_utf8(&infoLog).unwrap()
                );
            }

            // fragment shader
            let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragmentShaderSource.as_bytes()).unwrap();
            gl::ShaderSource(fragmentShader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragmentShader);
            // check for shader compile errors
            gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    fragmentShader,
                    512,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                    str::from_utf8(&infoLog).unwrap()
                );
            }

            // link shaders
            let shaderProgram = gl::CreateProgram();
            gl::AttachShader(shaderProgram, vertexShader);
            gl::AttachShader(shaderProgram, fragmentShader);
            gl::LinkProgram(shaderProgram);
            // check for linking errors
            gl::GetProgramiv(shaderProgram, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shaderProgram,
                    512,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                    str::from_utf8(&infoLog).unwrap()
                );
            }
            gl::DeleteShader(vertexShader);
            gl::DeleteShader(fragmentShader);

            // set up vertex data (and buffer(s)) and configure vertex attributes
            // ------------------------------------------------------------------
            // HINT: type annotation is crucial since default for float literals is f64
            let vertices: [f32; 9] = [
                -0.5, -0.5, 0.0, // left
                0.5, -0.5, 0.0, // right
                0.0, 0.5, 0.0, // top
            ];
            let (mut VBO, mut VAO) = (0, 0);
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);
            // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
            gl::BindVertexArray(VAO);

            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
            // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
            gl::BindVertexArray(0);

            (shaderProgram, VAO)
        };

        Self {
            window,
            shaderProgram,
            VAO,
        }
    }

    pub fn render_frame(&mut self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UseProgram(self.shaderProgram);
            gl::BindVertexArray(self.VAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window.borrow_mut().native_window_mut().swap_buffers();
    }
}
