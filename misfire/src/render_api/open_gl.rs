use std::{collections::HashMap, ffi::{CStr, CString}, fs, num::{NonZero, NonZeroU32}};

use glutin::{config::{ConfigTemplate, GlConfig}, context::{ContextAttributesBuilder, PossiblyCurrentContext}, prelude::{GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext}, surface::{GlSurface, SurfaceAttributesBuilder, SwapInterval, WindowSurface}};
use winit::raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::render_api::{buffer::{BufferLayout, ShaderDataType}, GraphicsContext, IndexBuffer, Shader, Texture2D, VertexArray, VertexBuffer};

use stb_image::image;

//TODO: Break into multiple files and Re-export?

pub mod gl {
    #![allow(clippy::all, warnings)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

// RenderAPI::Render::Command::DrawIndexed();

pub struct OpenGLContext {
    raw_win_handle: RawWindowHandle,
    raw_dsp_handle: RawDisplayHandle,
    window_height: u32, 
    window_width: u32,
    vsync_enabled: bool,
    context: Option<PossiblyCurrentContext>,
    surface: Option<glutin::surface::Surface<WindowSurface>>
}

impl OpenGLContext {
    pub fn new_windows_context(raw_win_handle: RawWindowHandle, raw_dsp_handle: RawDisplayHandle, window_height: u32, window_width: u32) -> Self {
        OpenGLContext { raw_win_handle, raw_dsp_handle, window_height, window_width, vsync_enabled: false, context: None, surface: None }
    }
}

impl GraphicsContext for OpenGLContext {

    fn init(&mut self) {

        let raw_win_handle=  self.raw_win_handle;
        let raw_dsp_handle = self.raw_dsp_handle;

        let gl_display = unsafe {
            glutin::display::Display::new(
                raw_dsp_handle,
                glutin::display::DisplayApiPreference::WglThenEgl(Some(raw_win_handle))
            ).unwrap()
        };

        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_win_handle));

        //TODO: Add config template builder
        // let test = ConfigTemplateBuilder::new()

        let configs  = unsafe {gl_display.find_configs(ConfigTemplate::default()).unwrap()};
        let config =  configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);
    
            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap();
    
        let raw_context = unsafe {
            //TODO: Figure out how to get the right config
            gl_display.create_context(&config, &context_attributes).unwrap()
        };
    
        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new()
        .build( raw_win_handle, 
            NonZero::new(self.window_width).unwrap(), 
            NonZero::new(self.window_height).unwrap());
    
        let surface = unsafe {
            gl_display.create_window_surface(&config, &surface_attributes).unwrap()
        };

        
        self.surface = Some(surface);
        
        self.context = Some(raw_context.make_current(self.surface.as_ref().unwrap()).unwrap());

       
        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_display.get_proc_address(symbol.as_c_str()) as *const _
        });

        unsafe  { 
            println!("OpenGL Renderer: {:?}",CStr::from_ptr(gl::GetString(gl::RENDERER).cast()));
            println!("OpenGL Vendor {:?}",CStr::from_ptr(gl::GetString(gl::VENDOR).cast())); 
            println!("OpenGL Version {:?}",CStr::from_ptr(gl::GetString(gl::VERSION).cast())); 
            println!("OpenGL Shader Version {:?}",CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION).cast()));   

        }

    }

    fn set_vsync(&mut self, vsync: bool) {
        if self.vsync_enabled != vsync {
            let interval = if vsync {
                self.vsync_enabled = true;
                SwapInterval::Wait(NonZeroU32::new(1).unwrap()) 
            } else {
                self.vsync_enabled = false;
                SwapInterval::DontWait
            };
            //TODO: Actually use this result
            let _ = self.surface.as_ref().unwrap().set_swap_interval(&self.context.as_ref().unwrap(), interval);
        }
    }

    fn swap_buffers(&self) {
        let _ = self.surface.as_ref().unwrap().swap_buffers(self.context.as_ref().unwrap());
    }

    fn release_context(&mut self) {
        let _ = self.context.take().unwrap().make_not_current().unwrap();
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        self.window_height = height;
        self.window_width = width;
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

}

pub struct OpenGLShader {
    renderer_id: u32
}

impl OpenGLShader {

    pub fn upload_uniform_int(&self, name: &CStr, int: i32) {
        unsafe {
            let uniform_loc = gl::GetUniformLocation(self.renderer_id, name.as_ptr());
            gl::Uniform1i(uniform_loc, int);
        }
    }

    pub fn upload_uniform_vec4(&self, name: &CStr, vec4: &glam::Vec4) {
        unsafe {
            let uniform_loc = gl::GetUniformLocation(self.renderer_id, name.as_ptr());
            gl::Uniform4fv(uniform_loc, 1, vec4.as_ref().as_ptr());
        }
    }

   pub fn upload_uniform_mat4(&self, name: &CStr, matrix: &glam::Mat4) {
        unsafe {
            let uniform_loc = gl::GetUniformLocation(self.renderer_id, name.as_ptr());
            gl::UniformMatrix4fv(uniform_loc, 1, gl::FALSE, matrix.to_cols_array().as_ptr());
        }

   }
}

fn shader_type_from_string(type_str: &str) -> gl::types::GLenum {
    match type_str {
        "vertex" => {
            gl::VERTEX_SHADER
        }
        "fragment" | "pixel" => {
            gl::FRAGMENT_SHADER
        }
        _ => {
            assert!(false, "Invaild shader type");
            0
        }
    }
}


fn pre_process_shader(source: String) -> HashMap<gl::types::GLenum, String> {
    let mut shader_sources= HashMap::new();

    let type_token = "#type";
    let type_token_length = type_token.len();
    let mut pos_opt = source.find(type_token);
    while let Some(pos) = pos_opt {
        let eol = source[pos..].find(|c| (c == '\r' || c == '\n')).unwrap()+pos;
        let begin = pos+type_token_length+1;
        let shader_type: String = source[begin..eol].trim().to_string();
        
        let next_line_pos =  source[eol..].find(|c| !"\r\n".contains(c)); //TODO: Should be next char that isn't these

        if let Some(next_line) = next_line_pos {
            pos_opt = source[next_line+eol..].find(type_token).map(|p| p + next_line + eol);
            if let Some(pos) = pos_opt {
                shader_sources.insert(shader_type_from_string(&shader_type), source[next_line+eol..pos].trim_start().to_string());
            } else {
                shader_sources.insert(shader_type_from_string(&shader_type), source[next_line+eol..].trim_start().to_string());
                break;
            }
        } else {
            break;
        }
    }

    shader_sources
}

fn compile_shaders(shader_sources: &HashMap<gl::types::GLenum, String>) -> u32 {
   let program = unsafe {
        gl::CreateProgram()
    };

    assert!(shader_sources.len() <= 2, "Too many shaders!");

    let gl_shader_ids: [u32; 2] = [0, 0];
    let mut insert_index = 0;

    for (shader_type, source) in shader_sources {
        unsafe {
            let shader = gl::CreateShader(*shader_type);
            // let clean_source: String = source.chars().filter(|&c| c != '\0').collect();
            let source_c_str = CString::new(source.as_str()).expect("C String new failed");
            gl::ShaderSource(shader, 1, [source_c_str.as_bytes_with_nul().as_ptr().cast()].as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut is_compiled =  std::mem::zeroed();
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut is_compiled);
            if is_compiled == gl::FALSE.into() {
                let mut max_length = std::mem::zeroed();
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut max_length);

                let mut info_log: Vec<u8> = Vec::new();
                info_log.resize(max_length as usize, 0);
                gl::GetShaderInfoLog(shader,max_length, &mut max_length, info_log.as_mut_ptr() as *mut i8);

                let log = String::from_utf8_lossy(&info_log).trim_end_matches("\0").to_string();

                gl::DeleteShader(shader);

                //TODO: Logging System
                println!("Shader erorr\n{}",log);
                println!("Shader Type\n{}",shader_type);

                assert!(false, "Shader compliation failure!");
                break;
            }
            gl::AttachShader(program, shader);
            gl_shader_ids[insert_index];
            insert_index = insert_index + 1;
        }
    }

    unsafe {

        let mut is_linked = std::mem::zeroed();
        gl::LinkProgram(program);
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut is_linked);

        if is_linked == gl::FALSE.into() {
            let mut max_length = std::mem::zeroed();
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut max_length);

            let mut info_log: Vec<u8> = Vec::new();
            info_log.resize(max_length as usize, 0);
            gl::GetProgramInfoLog(program,max_length, &mut max_length, info_log.as_mut_ptr() as *mut i8);

            let log = String::from_utf8_lossy(&info_log).trim_end_matches("\0").to_string();

            gl::DeleteProgram(program);

            for id in &gl_shader_ids {
                gl::DeleteShader(*id);
            }
            //TODO: Logging System
            println!("{}",log);
            assert!(false, "Shader link Failure");

        }
    }

    for id in gl_shader_ids {
        unsafe {
            gl::DetachShader(program, id);
        }
    }

    program
        
}

impl Shader for OpenGLShader {
    fn new_from_strings(vertex_src: String, fragment_src: String) -> Self where Self: Sized {

        let mut shader_source = HashMap::new();
        shader_source.insert(gl::VERTEX_SHADER, vertex_src);
        shader_source.insert(gl::FRAGMENT_SHADER, fragment_src);

        let program  = compile_shaders(&shader_source);        

         OpenGLShader {
            renderer_id: program
        }
    }
    //TODO: Decide if this should just have asserts or return a result
    fn new(file: &std::path::Path) -> Self {

        //TODO: Error Handling
        let source = fs::read_to_string(file).unwrap();

        let shader_source = pre_process_shader(source);

        let program  = compile_shaders(&shader_source);        

        OpenGLShader {
            renderer_id: program
        }
    }

    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.renderer_id); // bind prog
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::UseProgram(0); 
        }
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    

}

impl Drop for OpenGLShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.renderer_id); 
        }
    }
}

pub struct OpenGLTexture2D {
    path: &'static std::path::Path,
    width: i32, 
    height: i32,
    renderer_id: u32
}

fn flip_vertical(mut pixels: Vec<u8>, width: usize, height: usize, channels: usize) -> Vec<u8> {
    let row_stride = width * channels;
    for y in 0..(height / 2) {
        let top = y * row_stride;
        let bottom = (height - 1 - y) * row_stride;

        // split_at_mut ensures the borrows don't overlap
        let (head, tail) = pixels.split_at_mut(bottom);
        let row_top = &mut head[top..top + row_stride];
        let row_bottom = &mut tail[..row_stride];

        row_top.swap_with_slice(row_bottom);
    }
    pixels
}

impl Texture2D for OpenGLTexture2D {
    fn new(path: &'static std::path::Path) -> Self where Self: Sized {
        let (width, height, id) = unsafe {
            let mut  id = std::mem::zeroed();

            
            let img = image::load(path);
            let mut pixels: Vec<u8> = vec![];
            let (width, height) = match img {
                image::LoadResult::ImageU8(img) => {
                    println!("Loaded {}x{} ({} channels)", img.width, img.height, img.depth);
                    pixels = flip_vertical(img.data, img.width, img.height, img.depth); // Safe Vec<u8>
                    (img.width as i32, img.height as i32)
                }
                image::LoadResult::ImageF32(_img) => {
                    // HDR Images
                    (0,0)
                }
                image::LoadResult::Error(e) => {
                    eprintln!("Failed to load: {}", e);
                    (0, 0)
                }
            };

            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id);
            gl::TextureStorage2D(id, 1, gl::RGB8,width,height);

            gl::TextureParameteri(id, gl::TEXTURE_MIN_FILTER, gl::LINEAR.try_into().unwrap());
            gl::TextureParameteri(id, gl::TEXTURE_MAG_FILTER, gl::NEAREST.try_into().unwrap());

            //TODO: Load image pixels
            gl::TextureSubImage2D(id, 0,0,0, width, height, gl::RGB, gl::UNSIGNED_BYTE, pixels.as_ptr() as *const _);
            (width, height, id)
        };

        OpenGLTexture2D { path: path, width, height, renderer_id: id }
    }

    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self) -> i32 {
        self.height
    }

    fn bind(&self, slot: u32) {
        unsafe {
            gl::BindTextureUnit(slot, self.renderer_id);
        }
    }
}

impl Drop for OpenGLTexture2D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.renderer_id);
        }
    }
}

pub struct OpenGLVertexArray {
    renderer_id: u32,
    vertex_buffers: Vec<crate::Ref<dyn VertexBuffer>>,
    index_buffer: Option<crate::Ref<dyn IndexBuffer>>
}

impl OpenGLVertexArray {
    //FIXME: Find final place for this function
    fn shader_type_to_open_gl_type(data_type: &ShaderDataType) -> u32 {
        match data_type {
            ShaderDataType::Float => gl::FLOAT,
            ShaderDataType::Vec2 => gl::FLOAT,
            ShaderDataType::Vec3 => gl::FLOAT,
            ShaderDataType::Vec4 => gl::FLOAT,
            ShaderDataType::Mat3 => gl::FLOAT,
            ShaderDataType::Mat4 => gl::FLOAT,
            ShaderDataType::Int => gl::INT,
            ShaderDataType::Int2 => gl::INT,
            ShaderDataType::Int3 => gl::INT,
            ShaderDataType::Int4 => gl::INT,
            ShaderDataType::Bool => gl::BOOL,
        }
    } 
}

impl VertexArray for OpenGLVertexArray {

    fn new() -> Self {
         let id = unsafe {
            let mut id = std::mem::zeroed();
            gl::CreateVertexArrays(1, &mut id);
            id
        };
        OpenGLVertexArray {
            renderer_id: id,
            vertex_buffers: vec![],
            index_buffer: None
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.renderer_id);
        }
    }

    fn unbind(&self) {
        unsafe {gl::BindVertexArray(0);}
    }    

    fn add_vertex_buffer(&mut self, vertex_buffer: crate::Ref<dyn VertexBuffer>) {
        let buf = vertex_buffer.read().unwrap();
        let layout = buf.get_layout();

        unsafe { gl::BindVertexArray(self.renderer_id); }
        buf.bind();

       // TODO: Implement IntoIterator for BufferLayout
        for (index, element) in layout.get_elements().iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(index as u32);
                gl::VertexAttribPointer(
                index as u32, 
                element.get_component_count(), 
                OpenGLVertexArray::shader_type_to_open_gl_type(element.get_data_type()), 
                if *element.get_normailzed() {gl::TRUE} else {gl::FALSE},
                layout.get_stride() as gl::types::GLsizei, 
                element.get_offset() as *const _);
            }
        }
        self.vertex_buffers.push(vertex_buffer.clone());
    }

    fn set_index_buffer(&mut self, index_buffer: crate::Ref<dyn IndexBuffer>) {
        let buf = index_buffer.read().unwrap();
        unsafe { gl::BindVertexArray(self.renderer_id); }
        buf.bind();

        self.index_buffer = Some(index_buffer.clone());
    }

    fn get_index_buffer(&self) -> crate::Ref<dyn IndexBuffer> {
        self.index_buffer.as_ref().unwrap().clone()
    }
}

impl Drop for OpenGLVertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.renderer_id)}
    }
}

pub struct OpenGLVertexBuffer {
    renderer_id: u32,
    buffer_layout: BufferLayout
}

impl VertexBuffer for OpenGLVertexBuffer {

    fn new(vertices: &[f32], size: u32, buffer_layout: BufferLayout) -> Self {
        let id = unsafe {
            let mut id = std::mem::zeroed();
            gl::CreateBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(gl::ARRAY_BUFFER, size as gl::types::GLsizeiptr, 
                vertices.as_ptr() as *const _, gl::STATIC_DRAW); 
            id
        };
        OpenGLVertexBuffer { renderer_id: id, buffer_layout }
    }

    fn bind(&self) {
        unsafe {
            //TODO: Error Checking
            gl::BindBuffer(gl::ARRAY_BUFFER, self.renderer_id);
        }
    }

    fn unbind(&self) {
        unsafe {
            //TODO: Error Checking
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
    
    fn set_layout(&mut self, buffer_layout: BufferLayout) {
        self.buffer_layout = buffer_layout;
    }
    
    fn get_layout(&self) -> &BufferLayout {
        &self.buffer_layout
    }
}

impl Drop for OpenGLVertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.renderer_id);
        }
    }
}
// #endregion

pub struct OpenGLIndexBuffer {
    count: usize,
    renderer_id: u32
}

impl IndexBuffer for OpenGLIndexBuffer {
    fn new(indices: &[u32], count: usize) -> Self {
        let id = unsafe {
            let mut id = std::mem::zeroed();
            gl::CreateBuffers(1, &mut id);
            println!("{}", id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (std::mem::size_of::<u32>()*count) as gl::types::GLsizeiptr, 
                indices.as_ptr() as *const _, gl::STATIC_DRAW); 
            id
        };
        OpenGLIndexBuffer { count, renderer_id: id }
    }

    fn bind(&self) {
        unsafe {
            //TODO: Error Checking
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.renderer_id);
        }
    }

    fn unbind(&self) {
        unsafe {
            //TODO: Error Checking
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
    
    fn get_count(&self) -> usize {
        self.count
    }
}

impl Drop for OpenGLIndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.renderer_id);
        }
    }
}