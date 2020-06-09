/// TODO describe crate
pub mod types;
pub mod de {
    use crate::types::Event;

    /// TODO
    pub fn deserialize_event(x: &std::string::String) -> Event {
        serde_json::from_str(&x).unwrap()
    }
}

pub mod se {
    use crate::types::Event;

    /// TODO
    pub fn serialize_event(x: &Event) -> serde_json::Result<String> {
        serde_json::to_string(x)
    }
}

pub mod process;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
