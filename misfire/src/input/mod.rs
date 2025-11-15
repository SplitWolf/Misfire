use std::collections::HashMap;

use crate::event::{self, EventType, key_event::{KeyPressed, KeyReleased}, mouse_event::{MouseButton, MouseButtonPressed, MouseButtonReleased, MouseMoved}, try_cast};


#[derive(Debug, PartialEq, Copy, Clone)]
enum KeyState {
    Pressed,
    Released
}


//TODO: Implement Own Type for Keyboard Codes

//TODO: Convert this to array based for speed
pub struct InputSystem {
    key_states: HashMap<winit::keyboard::PhysicalKey, KeyState>,
    mouse_button_states: HashMap<MouseButton, KeyState>,
    mouse_position: (f64, f64),
}


impl InputSystem {

    pub(crate) fn new() -> Self {
        InputSystem {
            key_states: HashMap::new(),
            mouse_button_states: HashMap::new(),
            mouse_position: (0.0, 0.0)
        }
    }

    pub fn is_key_pressed(&self, key: winit::keyboard::PhysicalKey) -> bool {
        if let Some(state) = self.key_states.get(&key)  {
            *state == KeyState::Pressed
        } else {
            false
        }
    }
    
    pub fn is_btn_pressed(&self, btn: MouseButton) -> bool {
        if let Some(state) = self.mouse_button_states.get(&btn)  {
            *state == KeyState::Pressed
        } else {
            false
        }
    }


    pub(crate) fn on_event(&mut self, event: &mut dyn event::Event) {
        match event.get_event_type() {
            EventType::KeyPressed => {
                let e = try_cast::<KeyPressed>(event).unwrap();
                self.on_key_pressed(e);
                if e.get_key() == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Backslash) {
                    println!("Key States: {:?}, Mouse States: {:?}, Mouse Position: x: {:?}, y: {:?}", self.key_states, self.mouse_button_states, self.mouse_position.0, self.mouse_position.1)
                }
            }
            EventType::KeyReleased => {
                let e = try_cast::<KeyReleased>(event).unwrap();
                self.on_key_realeased(e);
            },
            EventType::MouseButtonPressed => {
                let e = try_cast::<MouseButtonPressed>(event).unwrap();
                self.on_mouse_pressed(e);
            },
            EventType::MouseButtonReleased => {
                let e = try_cast::<MouseButtonReleased>(event).unwrap();
                self.on_mouse_released(e);
            }
            EventType::MouseMoved => {
                let e = try_cast::<MouseMoved>(event).unwrap();
                self.on_mouse_moved(e);
            },
            _ => {}
        }
        event.set_handled(false);
    }

    fn on_mouse_moved (&mut self, event: &MouseMoved) {
        self.mouse_position = (event.get_x(), event.get_y())
    }

    //TODO: Check if key exists and update instead of insert
    fn on_mouse_pressed (&mut self, event: &MouseButtonPressed) {
        self.mouse_button_states.insert(event.get_key(), KeyState::Pressed);
    }

    fn on_mouse_released (&mut self, event: &MouseButtonReleased) {
        self.mouse_button_states.insert(event.get_key(), KeyState::Released);
    }

    //TODO: Check if key exists and update instead of insert
    fn on_key_pressed(&mut self, event: &KeyPressed) {
        self.key_states.insert(event.get_key(), KeyState::Pressed); 
    }

    fn on_key_realeased(&mut self, event: &KeyReleased) {
        self.key_states.insert(event.get_key(), KeyState::Released); 
    }
}