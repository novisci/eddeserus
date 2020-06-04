use crate::types::Event;


/// TODO
pub fn serialize_event(x: &Event) -> String {
    serde_json::to_string(x).unwrap()
}