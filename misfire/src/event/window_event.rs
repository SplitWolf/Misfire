use std::any::Any;
use super::{Event, EventType, EventCategory};

#[derive(Debug)]
pub struct WindowResizeEvent {
    width: u32,
    height: u32,
    handled: bool
}

impl WindowResizeEvent {
    pub fn new(width: u32, height: u32) -> Self {
        WindowResizeEvent {
            width,
            height,
            handled: false
        }
    }
    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn get_height(&self) -> u32 {
        self.height
    }
}

impl Event for WindowResizeEvent {
    fn get_event_type(&self) -> EventType {
        EventType::WindowResize
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
pub struct WindowCloseEvent {
    handled: bool
}

impl WindowCloseEvent {
    pub fn new() -> Self {
        WindowCloseEvent {
            handled: false
        }
    }
}

impl Event for WindowCloseEvent {
    fn get_event_type(&self) -> EventType {
        EventType::WindowClose
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
pub struct WindowFocus {
    handled: bool
}

impl WindowFocus {
    pub fn new() -> Self {
        WindowFocus {
            handled: false
        }
    }
}


impl Event for WindowFocus {
    fn get_event_type(&self) -> EventType {
        EventType::WindowFocus
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
pub struct WindowLostFocus {
    handled: bool
}

impl WindowLostFocus {
    pub fn new() -> Self {
        WindowLostFocus {
            handled: false
        }
    }
}


impl Event for WindowLostFocus {
    fn get_event_type(&self) -> EventType {
        EventType::WindowLostFocus
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
pub struct WindowMoved {
    x: i32,
    y: i32,
    handled: bool
}

impl WindowMoved {
    pub fn new(x: i32, y: i32) -> Self {
        WindowMoved {
            x,
            y,
            handled: false
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl Event for WindowMoved {
    fn get_event_type(&self) -> EventType {
        EventType::WindowMoved
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





