use crate::{render_api::{open_gl::{OpenGLIndexBuffer, OpenGLVertexBuffer}, IndexBuffer, VertexBuffer}, renderer::RendererAPI};

pub enum ShaderDataType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Mat4,
    Int,
    Int2,
    Int3,
    Int4,
    Bool
}

impl ShaderDataType {
    fn get_size(data_type: &ShaderDataType) -> u32 {
        match data_type {
            ShaderDataType::Float => 4,
            ShaderDataType::Vec2  => 4 * 2,
            ShaderDataType::Vec3  => 4 * 3,
            ShaderDataType::Vec4  => 4 * 4,
            ShaderDataType::Mat3  => 4 * 3 * 3,
            ShaderDataType::Mat4  => 4 * 4 * 4,
            ShaderDataType::Int   => 4,
            ShaderDataType::Int2  => 4 * 2,
            ShaderDataType::Int3  => 4 * 3,
            ShaderDataType::Int4  => 4 * 4,
            ShaderDataType::Bool  => 1,
        }
    }
}

//TODO: Could be re-exported in mods.rs (render_api)
pub fn create_vertex_buffer(vertices: &[f32], size: u32, buffer_layout: BufferLayout, api: RendererAPI) -> impl VertexBuffer {
    match api {
        RendererAPI::None => todo!("RendererAPI::None not supported currently!"),
        RendererAPI::OpenGL => OpenGLVertexBuffer::new(vertices, size, buffer_layout)
    }
}

//TODO: Could be ref &RendererAPI
pub fn create_index_buffer(indices: &[u32], size: usize, api: RendererAPI) -> impl IndexBuffer {
        match api {
        RendererAPI::None => todo!("RendererAPI::None not supported currently!"),
        RendererAPI::OpenGL => OpenGLIndexBuffer::new(indices, size)   
    }
}


pub struct BufferLayout {
    elements: Vec<BufferElement>,
    stride: u32
}

impl BufferLayout {
    pub fn new(elements: Vec<BufferElement>) -> Self {
        let mut layout = BufferLayout { elements , stride: 0};
        layout.calculate_offsets_and_stride();
        layout
    }
    pub fn get_elements(&self) -> &Vec<BufferElement> {
        &self.elements
    }
    pub fn get_stride(&self) -> u32 {
        self.stride
    }
    
    fn calculate_offsets_and_stride(&mut self) {
        let mut offset = 0;
        for element in self.elements.iter_mut() {
            element.offset = offset;
            offset += element.size;
            self.stride += element.size;
        }
    }
}

pub struct BufferElement {
    name: String,
    data_type: ShaderDataType,
    normalized: bool,
    size: u32,
    offset: u32
}

impl BufferElement {
    pub fn new(data_type: ShaderDataType, name: String, normalized: bool) -> Self {
        let size = ShaderDataType::get_size(&data_type);
        BufferElement { data_type, name, normalized, size, offset: 0}
    }

    //TODO: Should these just be public? we don't really write to them after creation tho.

    pub fn get_data_type(&self) -> &ShaderDataType {
        &self.data_type
    }

    pub fn get_size(&self) -> &u32 {
        &self.size
    }
    
    pub fn get_normailzed(&self) -> &bool {
        &self.normalized
    }

    pub fn get_offset(&self) -> u32 {
        self.offset
    }

    pub fn get_component_count(&self) -> i32 {
        match self.data_type {
            ShaderDataType::Float => 1,
            ShaderDataType::Vec2  => 2,
            ShaderDataType::Vec3  => 3,
            ShaderDataType::Vec4  => 4,
            ShaderDataType::Mat3  => 3*3,
            ShaderDataType::Mat4  => 4*4,
            ShaderDataType::Int   => 1,
            ShaderDataType::Int2  => 2,
            ShaderDataType::Int3  => 3,
            ShaderDataType::Int4  => 4,
            ShaderDataType::Bool  => 1,
        }
    }
}