use std::{num::NonZero, vec};

use glium::{implement_vertex, IndexBuffer, Program, Surface};
use glutin::{config::{ConfigTemplate, GetGlConfig, GlConfig}, context::{AsRawContext, ContextAttributesBuilder}, display::Display, prelude::{GlDisplay, NotCurrentGlContext}, surface::{SurfaceAttributesBuilder, WindowSurface}};
use winit::raw_window_handle::{RawDisplayHandle, RawWindowHandle};

// use crate::WindowsWindow;


static mut POSITIONS: [[f32;3];3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

pub trait GraphicsContext {
    fn init(&mut self);
    fn swap_buffers(&self);
}

//TODO: Switch to gl_generator . glium is depricated
pub struct OpenGLContext {
    raw_win_handle: RawWindowHandle,
    raw_dsp_handle: RawDisplayHandle,
    window_height: u32, 
    window_width: u32,
    glium_display: Option<glium::Display<WindowSurface>>
}

impl OpenGLContext {
    pub fn new_windows_context(raw_win_handle: RawWindowHandle, raw_dsp_handle: RawDisplayHandle, window_height: u32, window_width: u32) -> Self {
        OpenGLContext { raw_win_handle, raw_dsp_handle, window_height, window_width, glium_display: None }
    }
}


impl GraphicsContext for OpenGLContext {
    fn init(&mut self) {
        let raw_win_handle=  self.raw_win_handle;
        let raw_dsp_handle = self.raw_dsp_handle;

        let glutin_dsp = unsafe {
            Display::new(
                raw_dsp_handle,
                glutin::display::DisplayApiPreference::WglThenEgl(Some(raw_win_handle))
            ).unwrap()
        };
        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_win_handle));
    
        //TODO: Add config template builder
        // let test = ConfigTemplateBuilder::new()
        let configs  = unsafe {glutin_dsp.find_configs(ConfigTemplate::default()).unwrap()};
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
            
            glutin_dsp.create_context(&config, &context_attributes).unwrap()
        };
    
        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new()
        .build( raw_win_handle, 
            NonZero::new(self.window_width).unwrap(), 
            NonZero::new(self.window_height).unwrap());
    
        let surface = unsafe {
            glutin_dsp.create_window_surface(&config, &surface_attributes).unwrap()
        };
    
        let context = raw_context.make_current(&surface).unwrap();

        self.glium_display = Some(glium::Display::new(context, surface).unwrap());
        unsafe  {
            POSITIONS = [[-0.5, 0.0, 0.0], [0.5, 0.0, 0.0], [0.0, 0.5, 0.0]];
        }

    }


    fn swap_buffers(&self) {
        let display = self.glium_display.as_ref().expect("Display should exist before trying to render");
        let mut frame = display.draw();
        frame.clear_color(0.1, 0.1, 0.1, 1.0);
         // Vertex Array
         #[derive(Copy, Clone, Debug)]
         struct TriangleVertex {
             position: [f32; 3]
         }
         implement_vertex!(TriangleVertex, position);

         
        //  let sin = 0.00174532836*2.0;
        //  let cos = 0.99999847691*2.0;
        let sin = 0.00349065141;
        let cos = 0.99999390765;

         unsafe  {
            //  println!("Pos1: {:?}, {:?}, {:?}, Pos2: {:?}, {:?}, {:?}, Pos3: {:?}, {:?}, {:?}", 
            //  POSITIONS[0][0],POSITIONS[0][1],POSITIONS[0][2],
            //  POSITIONS[1][0],POSITIONS[1][1],POSITIONS[1][2],
            //  POSITIONS[1][0],POSITIONS[1][1],POSITIONS[1][2]);
            POSITIONS[0] = [(POSITIONS[0][0]*cos)-(POSITIONS[0][1]*sin), (POSITIONS[0][0]*sin)+(POSITIONS[0][1]*cos), POSITIONS[0][2]];
            POSITIONS[1] = [(POSITIONS[1][0]*cos)-(POSITIONS[1][1]*sin), (POSITIONS[1][0]*sin)+(POSITIONS[1][1]*cos), POSITIONS[0][2]];
            POSITIONS[2] = [(POSITIONS[2][0]*cos)-(POSITIONS[2][1]*sin), (POSITIONS[2][0]*sin)+(POSITIONS[2][1]*cos), POSITIONS[0][2]];

            // POSITIONS[1] = [POSITIONS[1][0]*0.9961, POSITIONS[1][1]*0.087, POSITIONS[1][2]*1.0];
            // POSITIONS[2] = [POSITIONS[2][0]*0.9961, POSITIONS[2][1]*0.087, POSITIONS[2][2]*1.0];
         }

         let vertices = unsafe {vec![
            TriangleVertex { position: POSITIONS[0]},
            TriangleVertex { position: POSITIONS[1]},
            TriangleVertex { position: POSITIONS[2]}
         ]};

        //  println!("{:?}",vertices);


         // Vertex Buffer
         let vertex_buffer = glium::VertexBuffer::new(
            self.glium_display.as_ref().expect("Display should exist before trying to render"), 
            &vertices).unwrap();

        // Index Buffer
         let indices: Vec<u16> = vec![0,1,2];
         let index_buffer = IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

         let program = Program::from_source(
            display,
            r#"
                #version 410
                in vec3 position;
                void main() {
                    gl_Position = vec4(position, 1.0);
                }
            "#,
            r#"
                #version 410
                out vec4 color;
                void main() {
                    color = vec4(1.0, 1.0, 1.0, 1.0);
                }
            "#,
            None,
        ).unwrap();
        
        let _ = frame.draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms, &glium::DrawParameters::default());

        let _ =  frame.finish();

        
    }
}