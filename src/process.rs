use crate::se::serialize_event;
use serde_json::{Deserializer};
use crate::types::Event;

/// TODO
pub fn process_events(events_json: &str , processor: &dyn Fn(Event) -> Event, handler: &dyn Fn(String) -> ()) -> (){
    let stream = Deserializer::from_str(&events_json).into_iter::<Event>();
    for event in stream {
        handler(serialize_event(&processor(event.unwrap())));
    }

}
