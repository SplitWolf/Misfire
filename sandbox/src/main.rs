use std::{f32::consts::PI, path::Path, sync::RwLock};

use glam::{self, Quat};
use misfire::{self, RendererAPI, VertexArray};

static  CUBE_VERTICES: [f32; 24*3] = [
    // Front Face
    -0.5, -0.5, 0.5,
     0.5, -0.5, 0.5,
    -0.5,  0.5, 0.5,
     0.5,  0.5, 0.5,
     // Back Face
    -0.5, -0.5, -0.5,
     0.5, -0.5, -0.5,
    -0.5,  0.5, -0.5,
     0.5,  0.5, -0.5,
    // Top Face
    -0.5,  0.5, -0.5,
     0.5,  0.5, -0.5,
    -0.5,  0.5,  0.5,
     0.5,  0.5,  0.5,
    // Bottom Face
    -0.5,  -0.5, -0.5,
     0.5,  -0.5, -0.5,
    -0.5,  -0.5,  0.5,
     0.5,  -0.5,  0.5,
    // Left Face
    -0.5,  -0.5,  0.5,
    -0.5,  -0.5, -0.5,
    -0.5,   0.5, -0.5,
    -0.5,   0.5,  0.5,
    // Right Face
    0.5,  -0.5,  0.5,
    0.5,  -0.5, -0.5,
    0.5,   0.5, -0.5,
    0.5,   0.5,  0.5,

];

static CUBE_INDICES: [u32;36] = [
    // Front face (vertices 0-3)
    0, 2, 3,
    0, 3, 1,

    // Back face (vertices 4-7)
    4, 5, 7,
    4, 7, 6,

    // Top face (vertices 8-11)
    8, 10, 11,
    8, 11, 9,

    // Bottom face (vertices 12-15)
    12, 13, 15,
    12, 15, 14,

    // Left face (vertices 16-19)
    16, 18, 19,
    16, 18, 17,

    // Right face (vertices 20-23)
    20, 21, 23,
    21, 23, 22,
];

static  SQUARE_VERTICIES: [f32; 5*4] = [
    -0.5, 0.0, -0.5, 0.0, 0.0,
     0.5, 0.0, -0.5, 1.0, 0.0,
     0.5, 0.0,  0.5, 1.0, 1.0,
    -0.5, 0.0,  0.5, 0.0, 1.0,
];

static SQUARE_INDICIES: [u32;6] = [
    // Front face (vertices 0-3)
    0, 1, 2,
    0, 2, 3
];

static mut DISTANCE: f32 = 0.0;
static mut OFFSET: f32 = 0.1;
static mut ROTATION_ANGLE: f32 = 0.0;

struct ExampleLayer {
    camera: misfire::renderer::OrthographicCamera,
    vertex_array: Option<misfire::Ref<dyn misfire::VertexArray>>,
    vertex_buffer: Option<misfire::Ref<dyn misfire::VertexBuffer>>,
    index_buffer: Option<misfire::Ref<dyn misfire::IndexBuffer>>,
    shader: Option<misfire::Ref<dyn misfire::Shader>>,
    vertex_array_text: Option<misfire::Ref<dyn misfire::VertexArray>>,
    vertex_buffer_text: Option<misfire::Ref<dyn misfire::VertexBuffer>>,
    index_buffer_text: Option<misfire::Ref<dyn misfire::IndexBuffer>>,
    shader_text: Option<misfire::Ref<dyn misfire::Shader>>,
    texture: Option<misfire::Ref<dyn misfire::Texture2D>>,
}

impl misfire::Layer for ExampleLayer {

    //TODO: Can we just pass the RenderAPI instead of the whole renderer
    fn on_init(&mut self, api: misfire::RendererAPI) {
        self.vertex_array = Some(misfire::Ref::new(RwLock::new(misfire::OpenGLVertexArray::new())));
        
        let layout = misfire::BufferLayout::new(vec![
            misfire::BufferElement::new(misfire::ShaderDataType::Vec3, String::from("a_Position"), false),
            // BufferElement::new(ShaderDataType::Vec4, String::from("a_Color"), false),
            // BufferElement::new(ShaderDataType::Vec3, String::from("a_TexCoord"))
            ]);

        self.vertex_buffer = Some(misfire::Ref::new(RwLock::new(
            misfire::create_vertex_buffer(
                &CUBE_VERTICES, 
                (CUBE_VERTICES.len()*std::mem::size_of::<f32>()) as u32, 
                layout, 
                api
        ))));
        
        self.index_buffer = Some(misfire::Ref::new(RwLock::new(misfire::create_index_buffer(&CUBE_INDICES, CUBE_INDICES.len(), api))));

        let mut vertex_array = self.vertex_array.as_ref().unwrap().write().unwrap();

        vertex_array.add_vertex_buffer(self.vertex_buffer.as_ref().unwrap().clone());
        vertex_array.set_index_buffer(self.index_buffer.as_ref().unwrap().clone());

        let shader = misfire::create_shader(&Path::new("assets/shaders/FlatColorShader.glsl"), api);
        // let shader = misfire::create_shader_from_strings(vert_src, frag_src, *renderer.get_api());

        self.shader = Some(misfire::Ref::new(RwLock::new(shader)));


        let shader_texture = misfire::create_shader(&Path::new("assets/shaders/Texture.glsl"), api);


        self.shader_text = Some(misfire::Ref::new(RwLock::new(shader_texture)));

        let vert_array = misfire::OpenGLVertexArray::new();
        vert_array.bind();

        self.vertex_array_text = Some(misfire::Ref::new(RwLock::new(vert_array)));
        self.index_buffer_text = Some(misfire::Ref::new(RwLock::new(misfire::create_index_buffer(&SQUARE_INDICIES, SQUARE_INDICIES.len(), api))));

        
        let layout_text = misfire::BufferLayout::new(vec![
            misfire::BufferElement::new(misfire::ShaderDataType::Vec3, String::from("a_Position"), false),
            misfire::BufferElement::new(misfire::ShaderDataType::Vec2, String::from("a_TexCoord"), false)
            // BufferElement::new(ShaderDataType::Vec4, String::from("a_Color"), false),
            ]);

        self.vertex_buffer_text = Some(misfire::Ref::new(RwLock::new(
            misfire::create_vertex_buffer(
                &SQUARE_VERTICIES, 
                (SQUARE_VERTICIES.len()*std::mem::size_of::<f32>()) as u32, 
                layout_text, 
                api
        ))));

        // self.index_buffer_text = Some(misfire::Ref::new(RwLock::new(misfire::create_index_buffer(&SQUARE_INDICIES, SQUARE_INDICIES.len(), *renderer.get_api()))));
        // RefCell?

        let mut vertex_array_text = self.vertex_array_text.as_ref().unwrap().write().unwrap();

        vertex_array_text.add_vertex_buffer(self.vertex_buffer_text.as_ref().unwrap().clone());
        vertex_array_text.set_index_buffer(self.index_buffer_text.as_ref().unwrap().clone());

        self.texture = Some(misfire::create_texture_2d(Path::new("assets/textures/Checkerboard.png"), api));

        let shader_tex = self.shader_text.as_ref().unwrap().read().unwrap();

        let shader = self.shader.as_ref().unwrap().read().unwrap();

        shader.bind();
        if let Some(gl_shader) = shader.as_any().downcast_ref::<misfire::OpenGLShader>() {
            gl_shader.upload_uniform_vec4(c"u_Color", &glam::vec4(0.8, 0.2, 0.8, 1.0));
        };


        shader_tex.bind();
        if let Some(gl_shader) = shader_tex.as_any().downcast_ref::<misfire::OpenGLShader>() {
            gl_shader.upload_uniform_int(c"u_Texture", 0);
        };
    }

    fn on_attach(&mut self) -> () {

    }

    fn on_detach(&mut self) -> () {

    }

    fn on_update(&mut self, window_props: &mut misfire::WindowProps, input: &misfire::InputSystem, timestep: misfire::Timestep) -> () {
        window_props.set_vsync(true);
        let dt = timestep.as_secs();
        // println!("Delta Time: {}", dt);
        let quat_y = glam::Quat::from_rotation_y(-PI/4.0);
        let quat_ang =  unsafe  {
            if DISTANCE > 1.05 || (DISTANCE < -1.05){
                OFFSET = OFFSET*-1.0;
            }
            DISTANCE = DISTANCE+(OFFSET*dt);

            ROTATION_ANGLE = (ROTATION_ANGLE + (PI/6.0)*dt) % (2.0*PI);
            glam::Quat::from_rotation_x(ROTATION_ANGLE)
        };

        // self.camera.set_position(glam::Vec3::new(unsafe {DISTANCE}, 0.0, 0.0));
        self.camera.set_rotation(quat_ang);
        // self.camera.set_aspect_ratio(window_props.get_width() as f32/window_props.get_height() as f32);
        // window_props.set_title(format!("Wee DISTANCE: {}", unsafe {DISTANCE}));
        
    }

    fn on_render(&mut self, renderer: &mut misfire::renderer::Renderer) {

        renderer.begin_scene(&self.camera);
        
        misfire::RenderCommand::set_clear_color(glam::vec4(0.2, 0.2, 0.2, 1.0),renderer.get_api());
        misfire::RenderCommand::clear(renderer.get_api());

        let shader = self.shader.as_ref().unwrap().read().unwrap();
        let vertex_array = self.vertex_array.as_deref().unwrap().read().unwrap();
        let vertex_array_tex = self.vertex_array_text.as_deref().unwrap().read().unwrap();
        let shader_tex = self.shader_text.as_ref().unwrap().read().unwrap();

        renderer.submit(&*vertex_array, &*shader, glam::Mat4::from_quat(Quat::from_rotation_z(PI/4.0)));
        renderer.submit(&*vertex_array, &*shader, glam::Mat4::from_translation(glam::vec3(1.0, 0.0, 1.0)));

        let text = self.texture.as_ref().unwrap().read().unwrap();
        text.bind(0);
        renderer.submit(&*vertex_array_tex, &*shader_tex, glam::Mat4::from_translation(glam::vec3(3.0, 0.0, 0.0))*glam::Mat4::from_scale(glam::vec3(3.0, 1.0, 3.0)));

        renderer.end_scene();
    }

    fn on_event(&mut self, event: &mut dyn misfire::event::Event) {
        // println!("{:?}", event.get_event_type());
        match event.get_event_type() {
            misfire::event::EventType::KeyPressed => {
                let key = try_cast::<misfire::event::key_event::KeyPressed>(event).unwrap();
                // println!("Key: {:?}",key.get_key());
                event.set_handled(true);
            },
            _ => {}
        }
    }
}


fn main() {

    let mut app = misfire::Application::new();

    let layer = Box::new(ExampleLayer {
        camera: misfire::renderer::OrthographicCamera::new(glam::Vec3::default(), glam::Quat::IDENTITY, -1.0, 1.0, -1.0, 1.0),
        vertex_array: None,
        vertex_buffer: None,
        index_buffer: None,
        shader: None,
        texture: None,
        vertex_array_text: None,
        vertex_buffer_text: None,
        index_buffer_text: None,
        shader_text: None,
    });

    app.push_layer(layer);
    
    misfire::Application::run(&mut app);
}
