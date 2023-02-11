use lightning::util::events::Event;

pub fn handle_ldk_event(event: Event) {
    println!("{:#?}", event);
}
