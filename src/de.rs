use crate::types::Event;


/// TODO
pub fn deserialize_event(x: &std::string::String) -> Event {
    serde_json::from_str(&x).unwrap()
}