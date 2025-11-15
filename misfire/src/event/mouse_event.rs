#![allow(dead_code)]

use std::any::Any;
use super::{Event, EventType, EventCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

#[derive(Debug)]
pub struct MouseButtonPressed {
    button: MouseButton,
    handled: bool
}

impl MouseButtonPressed {
    pub fn new(button: MouseButton) -> Self {
        MouseButtonPressed {
            button,
            handled: false
        }
    }
    
    pub fn get_key(&self) -> MouseButton {
        self.button
    }
}

impl Event for MouseButtonPressed {
    fn get_event_type(&self) -> EventType {
        EventType::MouseButtonPressed
    }

    fn get_category_flags(&self) -> u8 {
        EventCategory::EventCategoryMouseButton as u8 | EventCategory::EventCategoryMouse as u8 | EventCategory::EventCategoryInput as u8
    }

    fn is_in_category(&self, category: EventCategory) -> bool {
        self.get_category_flags() & category as u8 != 0 
    }

    fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    fn is_handled(&self) -> &bool {
        &self.handled
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


#[derive(Debug)]
pub struct MouseButtonReleased {
    button: MouseButton,
    handled: bool
}

impl MouseButtonReleased {
    pub fn new(button: MouseButton) -> Self {
        MouseButtonReleased {
            button,
            handled: false
        }
    }
    
    pub fn get_key(&self) -> MouseButton {
        self.button
    }
}

impl Event for MouseButtonReleased {
    fn get_event_type(&self) -> EventType {
        EventType::MouseButtonReleased
    }

    fn get_category_flags(&self) -> u8 {
        EventCategory::EventCategoryMouseButton as u8 | EventCategory::EventCategoryMouse as u8 | EventCategory::EventCategoryInput as u8
    }

    fn is_in_category(&self, category: EventCategory) -> bool {
        self.get_category_flags() & category as u8 != 0 
    }

    fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    fn is_handled(&self) -> &bool {
        &self.handled
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}



#[derive(Debug)]
pub struct MouseMoved {
    x: f64,
    y: f64,
    handled: bool
}

impl MouseMoved {
    pub fn new(x: f64, y: f64) -> Self {
        MouseMoved {
            x,
            y,
            handled: false
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }
    pub fn get_y(&self) -> f64 {
        self.y
    }
}

impl Event for MouseMoved {
    fn get_event_type(&self) -> EventType {
        EventType::MouseMoved
    }

    fn get_category_flags(&self) -> u8 {
        EventCategory::EventCategoryMouse as u8 | EventCategory::EventCategoryInput as u8
    }

    fn is_in_category(&self, category: EventCategory) -> bool {
        self.get_category_flags() & category as u8 != 0 
    }

    fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    fn is_handled(&self) -> &bool {
        &self.handled
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


#[derive(Debug)]
pub struct MouseScrolled {
    x_offset: f32,
    y_offset: f32,
    handled: bool
}

impl MouseScrolled {
    pub fn new(x_offset: f32,y_offset: f32) -> Self {
        MouseScrolled {
            x_offset,
            y_offset,
            handled: false
        }
    }

    pub fn get_x_offset(&self) -> f32 {
        self.x_offset
    }
    pub fn get_y_offset(&self) -> f32 {
        self.y_offset
    }
}


impl Event for MouseScrolled {
    fn get_event_type(&self) -> EventType {
        EventType::MouseScrolled
    }

    fn get_category_flags(&self) -> u8 {
        EventCategory::EventCategoryMouse as u8 | EventCategory::EventCategoryInput as u8
    }

    fn is_in_category(&self, category: EventCategory) -> bool {
        self.get_category_flags() & category as u8 != 0 
    }

    fn set_handled(&mut self, handled: bool) {
        self.handled = handled;
    }

    fn is_handled(&self) -> &bool {
        &self.handled
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
