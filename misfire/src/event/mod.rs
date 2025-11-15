use std::any::Any;
pub mod window_event;
pub mod key_event;
pub mod mouse_event;


#[derive(Debug)]
pub enum EventType {
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    AppTick, AppUpdate, AppRender,
    KeyPressed, KeyReleased,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

#[derive(Debug)]
#[repr(u8)]
pub enum EventCategory {
    EventCategoryApplication = 1,
    EventCategoryInput = 2,
    EventCategoryKeyboard = 4,
    EventCategoryMouse = 8,
    EventCategoryMouseButton = 16
}

pub trait Event {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_event_type(&self) -> EventType; //NOTE: Could be ref ig?
    fn get_category_flags(&self) -> u8;
    fn is_in_category(&self, category: EventCategory) -> bool;
    fn set_handled(&mut self, handled: bool);
    fn is_handled(&self) -> &bool;
}

impl dyn Event + '_ {

    pub fn downcast_ref<T: Event + 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    pub fn downcast_mut<T: Event + 'static>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut::<T>()
    }

}

pub fn try_cast<E: Event + 'static>(event: &mut dyn Event) -> Option<&E> {
    event.downcast_ref()
}