use std::fmt::Debug;

use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{ElementState, WindowEvent}, event_loop::EventLoop, raw_window_handle::{HasDisplayHandle, HasWindowHandle}, window::Window};

pub mod event;
use event::{Event, EventType};
pub mod layer;
use layer::Layer;

mod render_api;

use crate::{layer::LayerStack, render_api::{GraphicsContext, OpenGLContext}};


pub struct WindowsWindow {
    winit_window: Option<winit::window::Window>,
    window_props: WindowProps,
    graphics_context: Option<Box<dyn GraphicsContext>>
}

pub struct WindowProps {
    title: String,
    width: u32,
    height: u32,
    vsync: bool
}

impl WindowProps {

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        self.vsync = vsync;
    }

    pub fn is_vsync_enabled(&self) -> bool {
        self.vsync
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }
}

pub struct Application {
    layer_stack: LayerStack,
    window: WindowsWindow,
    should_close: bool,
    renderer: Renderer,
    last_frame_time: Instant
}

#[derive(Debug, Copy, Clone)]
pub struct Timestep(Duration);

impl Timestep {
    pub fn new(delta: Duration) -> Self {
        Self(delta)
    }

    pub fn as_secs(&self) -> f32 {
        self.0.as_secs_f32()
    }

    pub fn as_millis(&self) -> f32 {
        self.0.as_secs_f32()*1000.0 + self.0.subsec_nanos() as f32 / 1_000_000.0
    }

    pub fn duration(&self) -> Duration {
        self.0
    }
}

impl Application {

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {self.layer_stack.push_layer(layer);}
    // pub fn pop_layer() -> impl Layer {todo!()}

    pub fn new() -> Application {
        Application::new_with_properties(WindowProps {
            title: "Misfire App".to_string(),
                    width: 1280,
            height: 720,
            vsync: false
        })
    }

    pub fn new_with_properties(window_props: WindowProps) -> Application {
        Application {
            layer_stack: LayerStack::new(),
            window: WindowsWindow {
                winit_window: None,
                window_props,
                graphics_context: None
            },
            should_close: false,
            renderer: Renderer::new(RendererAPI::OpenGL),
            last_frame_time: Instant::now()
        }
    }

    pub fn run(app: &mut Application) {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let _ = event_loop.run_app(app);

    }

    fn handle_event<E: Event + Debug>(&self, event: &mut E) {
    // fn handle_event(&self, event: &mut dyn Event) { // Required debug trait impl for dyn event::Event

        let type_e = event.get_event_type();
        // println!("{:?}", &event);
        match type_e {
            EventType::WindowClose => {
                println!("Window Closed!");
                event.set_handled(true);
            },
            EventType::WindowResize =>  {
               
                let q = event.as_any().downcast_ref::<event::window_event::WindowResizeEvent>().unwrap();
                println!("W: {} H: {}",q.get_width(), q.get_height());
                event.set_handled(true);
            },
            _ => {}
        };
        
        for layer in &mut self.layer_stack {
            if *event.is_handled() {break};
            layer.on_event(event);
        }
    }

    fn app_init(&mut self) {
        self.last_frame_time = Instant::now();
        self.window.graphics_context.as_mut().unwrap().init();
    
        self.renderer.on_init();


        for layer in &mut self.layer_stack {
            //TODO: Could we combine on_init with on_attach?
            layer.on_init(*self.renderer.get_api());
        }
    }

    fn app_loop(&mut self) {        
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_frame_time);
        let ts = Timestep(delta_time);
        self.last_frame_time = now;

        self.window.graphics_context.as_mut().unwrap().set_vsync(self.window.window_props.vsync);

        for layer in &mut self.layer_stack {
            layer.on_update(&mut self.window.window_props, ts);
        }
        
        for layer in &mut self.layer_stack {
            layer.on_render(&mut self.renderer);
        }
        
        self.window.winit_window.as_ref().unwrap().set_title(&self.window.window_props.title);

        self.window.graphics_context.as_ref().unwrap().swap_buffers();
        // std::thread::sleep(time::Duration::from_millis(100));

    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window.winit_window = Some(event_loop.create_window(Window::default_attributes()
        .with_title(self.window.window_props.title).with_inner_size(PhysicalSize{width: self.window.window_props.width, height: self.window.window_props.height})).unwrap());

        let raw_win_handle= self.window.winit_window.as_ref().unwrap().window_handle().unwrap().as_raw();
        let raw_dsp_handle= self.window.winit_window.as_ref().unwrap().display_handle().unwrap().as_raw();

        let context: Option<Box<(dyn GraphicsContext)>> = Some(Box::new(OpenGLContext::new_windows_context(raw_win_handle,raw_dsp_handle,self.window.window_props.height.clone(),self.window.window_props.width.clone())));

        self.window.graphics_context = context;
        self.window.graphics_context.as_mut().unwrap().init();

    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId, //Does this have any use to us?
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                let mut event = event::window_event::WindowCloseEvent::new();
                self.handle_event(&mut event);
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Draw.
                self.render();

                // Queue a RedrawRequested event.
        
                self.window.winit_window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::Resized(size) => {
                let mut event = event::window_event::WindowResizeEvent::new(size.width, size.height);
                self.window.window_props.height = size.height;
                self.window.window_props.width = size.width; 
                self.handle_event(&mut event);
            }
            WindowEvent::Moved(pos) => {
                let mut event = event::window_event::WindowMoved::new(pos.x, pos.y);
                self.handle_event(&mut event);
            },
            WindowEvent::Focused(is_focused) => {
                if is_focused {
                    let mut event = event::window_event::WindowFocus::new();
                    self.handle_event(&mut event);
                } 
                else {
                    let mut event =event::window_event::WindowLostFocus::new();
                    self.handle_event(&mut event);
                };
            },
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                if event.state.is_pressed() {
                    let mut event = event::key_event::KeyPressed::new(event.physical_key, event.text, if event.repeat {1} else {0});
                    self.handle_event(&mut event);
                } else {
                    let mut event = event::key_event::KeyReleased::new(event.physical_key, event.text);
                    self.handle_event(&mut event);
                }
            },
            WindowEvent::CursorMoved { device_id: _, position } => {
                let mut event = event::mouse_event::MouseMoved::new(position.x, position.y);
                self.handle_event(&mut event);
            },
            WindowEvent::MouseInput { device_id: _, state, button } => {
                let btn = match button {
                    winit::event::MouseButton::Left => event::mouse_event::MouseButton::Left,
                    winit::event::MouseButton::Right => event::mouse_event::MouseButton::Right,
                    winit::event::MouseButton::Middle => event::mouse_event::MouseButton::Middle,
                    winit::event::MouseButton::Back => event::mouse_event::MouseButton::Back,
                    winit::event::MouseButton::Forward => event::mouse_event::MouseButton::Forward,
                    winit::event::MouseButton::Other(num) => event::mouse_event::MouseButton::Other(num),
                };
                match state {
                    ElementState::Pressed => {
                        let mut event = event::mouse_event::MouseButtonPressed::new(btn);
                        self.handle_event(&mut event);
                    },
                    ElementState::Released => {
                        let mut event = event::mouse_event::MouseButtonReleased::new(btn);
                        self.handle_event(&mut event);
                    },
                }
            },
            WindowEvent::MouseWheel { device_id: _, delta, phase: _ } => {
                
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(delta_x, delta_y) => {
                        let mut event = event::mouse_event::MouseScrolled::new(delta_x,delta_y);
                        self.handle_event(&mut event);
                    },
                    _ => {}
                    // winit::event::MouseScrollDelta::PixelDelta(pos) => {},
                }
                
            },
            // WindowEvent::CursorEntered { device_id } => todo!(),
            // WindowEvent::CursorLeft { device_id } => todo!(),
            _ => {}
            // WindowEvent::Occluded(_) => todo!(),
            // WindowEvent::Destroyed => todo!(),
            // WindowEvent::DroppedFile(path_buf) => todo!(),
            // WindowEvent::HoveredFile(path_buf) => todo!(),
            // WindowEvent::HoveredFileCancelled => todo!(),
            
            // WindowEvent::ModifiersChanged(modifiers) => todo!(),
            // WindowEvent::Ime(ime) => todo!(),
        
            // WindowEvent::PinchGesture { device_id, delta, phase } => todo!(),
            // WindowEvent::PanGesture { device_id, delta, phase } => todo!(),
            // WindowEvent::DoubleTapGesture { device_id } => todo!(),
            // WindowEvent::RotationGesture { device_id, delta, phase } => todo!(),
            // WindowEvent::TouchpadPressure { device_id, pressure, stage } => todo!(),
            // WindowEvent::AxisMotion { device_id, axis, value } => todo!(),
            // WindowEvent::Touch(touch) => todo!(),
            // WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
            // WindowEvent::ThemeChanged(theme) => todo!(),
       
        }
    }
}





// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
