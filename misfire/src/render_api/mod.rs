#![allow(dead_code)]
pub mod buffer;
// TODO: Conditional Compile
//TODO: Make not pub
pub mod open_gl;

use std::sync::RwLock;

use crate::{render_api::buffer::BufferLayout, RendererAPI};

pub trait GraphicsContext {
    fn init(&mut self);
    fn set_vsync(&mut self, vsync: bool);
    fn swap_buffers(&self);
    fn release_context(&mut self);
    fn on_resize(&mut self, width: u32, height: u32);
}

#[derive(Debug)]
pub enum ShaderComplieError {
    VertexShaderCompileFailed,
    FragmentShaderCompileFailed,
    ShaderLinkFailure
}

pub trait Shader {
    fn new(file: &std::path::Path) -> Self where Self: Sized;
    fn new_from_strings(vertex_src: String, fragment_src: String) -> Self where Self: Sized;
    fn bind(&self);
    fn unbind(&self);
    fn as_any(&self) -> &dyn std::any::Any;
}

pub fn create_shader_from_strings(vertex_src: String, fragment_src: String, api: RendererAPI) -> impl Shader {
    match api {
        RendererAPI::None => todo!("RendererAPI::None not supported currently!"),
        RendererAPI::OpenGL => open_gl::OpenGLShader::new_from_strings(vertex_src, fragment_src)
    }
}

pub fn create_shader(path: &std::path::Path, api: RendererAPI) -> impl Shader {
    match api {
        RendererAPI::None => todo!("RendererAPI::None not supported currently!"),
        RendererAPI::OpenGL => open_gl::OpenGLShader::new(path),
    }
}

pub trait Texture {
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;

    fn bind(&self, slot: u32);
}
impl<T: Texture2D> Texture for T {
    fn get_width(&self) -> i32 {
        self.get_width()
    }

    fn get_height(&self) -> i32 {
        self.get_height()
    }

    fn bind(&self, slot: u32) {
        self.bind(slot);
    }
}

pub trait Texture2D {
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;

    fn bind(&self, slot: u32);
    fn new(path: &'static std::path::Path) -> Self where Self: Sized;
}

pub fn create_texture_2d(path: &'static std::path::Path, api: RendererAPI) -> crate::Ref<dyn Texture2D> {
    match api {
        RendererAPI::None => todo!(),
        RendererAPI::OpenGL => crate::Ref::new(RwLock::new(open_gl::OpenGLTexture2D::new(path))),
    }
}

pub trait VertexArray {
    fn new() -> Self where Self: Sized;
    fn bind(&self);
    fn unbind(&self);

    fn add_vertex_buffer(&mut self, vertex_buffer: crate::Ref<dyn VertexBuffer>);
    fn set_index_buffer(&mut self, index_buffer: crate::Ref<dyn IndexBuffer>);
    fn get_index_buffer(&self) -> crate::Ref<dyn IndexBuffer>;
}

pub trait VertexBuffer {
    fn new(vertices: &[f32], size: u32, buffer_layout: BufferLayout) -> Self where Self: Sized;
    fn bind(&self);
    fn unbind(&self);

    fn set_layout(&mut self, buffer_layout: BufferLayout);
    fn get_layout(&self) -> &BufferLayout;
}

pub trait IndexBuffer {
    fn new(indices: &[u32], size: usize) -> Self where Self: Sized;
    fn get_count(&self) -> usize;
    fn bind(&self);
    fn unbind(&self);
}

#[allow(non_snake_case)]
pub mod RenderCommand {
    use crate::{render_api::{open_gl::gl, VertexArray}, renderer::RendererAPI};

    pub fn set_clear_color(color: glam::Vec4, render_api: &RendererAPI) { // PARAM: VEC4
        match render_api {
            RendererAPI::None => todo!(),
            RendererAPI::OpenGL => {
                unsafe {
                    gl::ClearColor(color.x, color.y, color.z, color.w);
                }
            },
        }
    } 

    pub fn clear(render_api: &RendererAPI) {
        match render_api {
            RendererAPI::None => todo!(),
            RendererAPI::OpenGL => {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }
            },
        }
    }

    // FIXME: Does this allow dynamic dispatch?
    pub fn draw_indexed<VA: VertexArray + ? Sized>(vertex_array: &VA, render_api: &RendererAPI) {
        match render_api {
            RendererAPI::None => todo!(),
            RendererAPI::OpenGL => {
                unsafe {
                    let index_buffer = vertex_array.get_index_buffer();
                    let buffer_lock = index_buffer.read().unwrap();
                    gl::DrawElements(gl::TRIANGLES, buffer_lock.get_count() as i32,  gl::UNSIGNED_INT, std::ptr::null());
                }
            },
        }
    }
}