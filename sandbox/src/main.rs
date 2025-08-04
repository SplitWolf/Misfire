use misfire::{event::{self, EventType}, layer::Layer, Application};


struct ExampleLayer {

}

impl Layer for ExampleLayer {
    fn on_attach(&self) -> () {

    }

    fn on_detach(&self) -> () {

    }

    fn on_update(&self) -> () {

    }

    fn on_event(&self, event: &mut dyn misfire::event::Event) {
        // println!("{:?}", event.get_event_type());
        match event.get_event_type() {
            EventType::KeyPressed => {
                let key = event.as_any().downcast_ref::<event::key_event::KeyPressed>().unwrap();
                println!("Key: {:?}",key.get_key());
                event.set_handled(true);
            },
            _ => {}
        }
    }
}


fn main() {

    let mut app = Application::new();

    let layer = Box::new(ExampleLayer {});

    app.push_layer(layer);
    
    Application::run(&mut app);
}
