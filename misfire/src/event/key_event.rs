#![allow(dead_code)]

use std::any::Any;
use winit::keyboard::SmolStr;

use super::{Event, EventType, EventCategory};

#[derive(Debug)]
pub struct KeyPressed {
    key: winit::keyboard::PhysicalKey,
    text: Option<SmolStr>,
    repeat_count: i32,
    handled: bool
}

impl KeyPressed {
    pub fn new(key: winit::keyboard::PhysicalKey, text:Option<SmolStr>, repeat_count: i32) -> Self {
        KeyPressed {
            key,
            text,
            repeat_count,
            handled: false
        }
    }

    pub fn get_key(&self) -> winit::keyboard::PhysicalKey {
        self.key
    }
}

impl Event for KeyPressed {
    fn get_event_type(&self) -> EventType {
        EventType::KeyPressed
    }

    fn get_category_flags(&self) -> u8 {
        EventCategory::EventCategoryApplication as u8
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
}


#[derive(Debug)]
pub struct KeyReleased {
    key: winit::keyboard::PhysicalKey,
    text: Option<SmolStr>,
    handled: bool
}

impl KeyReleased {
    pub fn new(key: winit::keyboard::PhysicalKey, text:Option<SmolStr>   ) -> Self {
        KeyReleased {
            key,
            text,
            handled: false
        }
    }

    pub fn get_key(&self) -> winit::keyboard::PhysicalKey {
        self.key
    }
}

impl Event for KeyReleased {
    fn get_event_type(&self) -> EventType {
        EventType::KeyReleased
    }

    fn get_category_flags(&self) -> u8 {
        EventCategory::EventCategoryApplication as u8
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
}

