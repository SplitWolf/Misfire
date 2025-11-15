#![allow(dead_code)]

use std::f32::consts::PI;

use glam;

use crate::{Timestep, input::InputSystem, render_api::{self, Shader, VertexArray, open_gl::{OpenGLShader, gl}}};

#[derive(Debug, Copy, Clone)]
pub enum RendererAPI {
    None,
    OpenGL
}

// Projection*View * Model/World Matrix * Vertex Pos in gl | in Dx vp*model*view * proj combine via TRS
// Need model matrix in VS therefore do VP * model * vp in shader 

pub struct OrthographicCameraController {

    camera: OrthographicCamera,
    zoom_level: f32,
    aspect_ratio: f32,
    camera_positon: glam::Vec3,
    camera_rotation: glam::Quat,
    camera_rotation_speed: f32,
    camera_movment_speed: f32
}

impl OrthographicCameraController {
    pub fn new(aspect_ratio: f32) -> Self {
        let zoom_level = 3.0;
        OrthographicCameraController {
            camera: OrthographicCamera::new(glam::vec3(0.0, 0.0, 0.0),glam::Quat::IDENTITY,-aspect_ratio*zoom_level,aspect_ratio*zoom_level,-zoom_level,zoom_level),
            zoom_level: 3.0,
            aspect_ratio: aspect_ratio,
            camera_rotation_speed: PI / 36.0,
            camera_movment_speed: 1.5,
            camera_positon: glam::vec3(0.0, 0.0, 0.0),
            camera_rotation: glam::Quat::IDENTITY,
        }
    }

    pub fn get_camera(&self) -> &OrthographicCamera {
        &self.camera
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn on_update(&mut self, ts: Timestep, input: &InputSystem) {
        if input.is_key_pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW)) {
            self.camera_positon = self.camera_positon - glam::vec3(0.0, self.camera_movment_speed*ts.as_secs(), 0.0);
            self.camera.set_position( self.camera_positon);
        }

        if input.is_key_pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyS)) {
            self.camera_positon = self.camera_positon + glam::vec3(0.0, self.camera_movment_speed*ts.as_secs(), 0.0);
            self.camera.set_position( self.camera_positon);
        }

        if input.is_key_pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD)) {
            self.camera_positon = self.camera_positon - glam::vec3(self.camera_movment_speed*ts.as_secs(), 0.0, 0.0);
            self.camera.set_position( self.camera_positon);
        }
 
        if input.is_key_pressed(winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA)) {
            self.camera_positon = self.camera_positon + glam::vec3(self.camera_movment_speed*ts.as_secs(),0.0, 0.0);
            self.camera.set_position( self.camera_positon);
        }
    }


    pub fn on_event(&mut self, event: &mut dyn crate::event::Event) {
        match event.get_event_type() {
            crate::event::EventType::WindowResize => {
                let e = event.as_any().downcast_ref::<crate::event::window_event::WindowResizeEvent>().unwrap();
                self.on_window_resize(e);
            } ,
            crate::event::EventType::MouseScrolled => {
                let e = event.as_any().downcast_ref::<crate::event::mouse_event::MouseScrolled>().unwrap();
                self.on_mouse_scrolled(e);
            },
            _ => {},
        }
    }

    fn on_mouse_scrolled(&mut self, event: &crate::event::mouse_event::MouseScrolled) {
        self.zoom_level = -event.get_x_offset()*0.25;
    }

    fn on_window_resize(&mut self, event: &crate::event::window_event::WindowResizeEvent) {
        self.set_aspect_ratio(event.get_width() as f32 / event.get_height() as f32);
    }
}

pub struct OrthographicCamera {
    // View Matrix
    position: glam::Vec3,
    rotation: glam::Quat,
    // Projection Matrix
    projection: glam::Mat4,
    view_projection_matrix: glam::Mat4
}

impl OrthographicCamera {
    pub fn new(position: glam::Vec3, rotation: glam::Quat, left: f32, right: f32, bottom: f32, top: f32) -> Self {
        let mut cam = OrthographicCamera { position, rotation, projection: glam::Mat4::orthographic_rh_gl(left, right, bottom, top, -1.0, 1.0), view_projection_matrix:  glam::Mat4::IDENTITY};
        cam.calcute_view_projection();
        cam
    }

    fn calcute_view_projection(&mut self) {
        let translation = glam::Mat4::from_translation(self.position);
        let rotation = glam::Mat4::from_quat(self.rotation*glam::Quat::from_rotation_x(PI/2.0));

        self.view_projection_matrix = self.projection*((rotation*translation).inverse());

        // self.view_projection_matrix = glam::Mat4::orthographic_rh_gl(-width_hor,width_hor,-width_vert,width_vert,-2.0,2.0)*((rotation*translation).inverse());
    }

    pub fn get_view_projection_matrix(&self) -> glam::Mat4 {
        self.view_projection_matrix
    }

    pub fn set_projection(&mut self, left: f32, right: f32, top: f32, bottom: f32)  {
        self.projection = glam::Mat4::orthographic_rh_gl(left, right, bottom, top, -1.0, 1.0);
    }

    pub fn set_position(&mut self, position: glam::Vec3) {
        self.position = position;
        self.calcute_view_projection();
    }

    pub fn set_rotation(&mut self, rotation: glam::Quat) {
        self.rotation = rotation;
        self.calcute_view_projection();
    }
}

struct SceneData {
    render_api: RendererAPI,
    pub view_projection: glam::Mat4
}

static mut SCENE_DATA: SceneData = SceneData {
    render_api: RendererAPI::OpenGL,
    view_projection: glam::Mat4::IDENTITY
};


pub struct Renderer {
    renderer_api: RendererAPI,
    view_projection: Option<glam::Mat4>
}

impl Renderer {

    pub fn on_init(&self) {
        unsafe {
            //TODO: Move to open gl specific area
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        }
    }

    pub fn new(renderer_api: RendererAPI) -> Self {
        Renderer { renderer_api, view_projection: None }
    }

    pub fn begin_scene(&mut self, camera: &OrthographicCamera) {
        self.view_projection = Some(camera.get_view_projection_matrix());
        unsafe {
            SCENE_DATA.view_projection = camera.get_view_projection_matrix()
        }
    }

    pub fn end_scene(&self) {

    }

    pub fn submit<VA: VertexArray + ? Sized, S: Shader + ? Sized>(&self, va: &VA, shader: &S, transform: glam::Mat4) {
        shader.bind();
        if let Some(gl_shader) = shader.as_any().downcast_ref::<OpenGLShader>() {
            gl_shader.upload_uniform_mat4(c"u_ViewProjectionMatrix", &self.view_projection.unwrap());
            gl_shader.upload_uniform_mat4(c"u_Transform", &transform);
        };
        va.bind();
        render_api::RenderCommand::draw_indexed(va, &self.renderer_api);
    }

    pub fn get_api(&self) -> &RendererAPI {
        &self.renderer_api
    }

}