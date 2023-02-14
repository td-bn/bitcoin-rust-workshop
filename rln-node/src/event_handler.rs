use lightning::util::events::{Event, EventHandler};

pub struct RLNEventHandler;

impl EventHandler for RLNEventHandler {
    fn handle_event(&self, event: Event) {
        println!("{:#?}", event);
    }
}
