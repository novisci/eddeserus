//! eddeserus
//!
//! `eddeserus` is an Event Data DE/SErialization library written in Rust. 
//! For information about  NoviSci's event data model see 
//! [EDM](https://docs.novisci.com/schema/event-data-model/0.1/) schema.
//!
//! Currently, `eddeserus` can de/serialize JSON formatted events. 
//
#![doc(html_root_url = "https://docs.novisci.com/eddeserus/")]
#![doc(issue_tracker_base_url = "https://gitlab.novisci.com/nsStat/eddeserus/issues")]
//

/// Rust types corresponding to events and elements thereof.
pub mod types;

/// Provides functions for deserialization from JSON to an `Event` and 
/// serialization from an `Event` to JSON.
pub mod sede{
    use crate::types::Event;
    use serde_json::Result;

    /// Deserialize a string reference to a `serde_json::Result<Event>`
    /// 
    /// Example:
    /// ```
    /// use eddeserus::sede::*;
    /// let json = r#"["A", "2010-01-01", null, "Enrollment", [], {"facts":{}, "source":null}]"#;
    /// println!("{:?}", deserialize_event(&json.to_string()));
    /// ```
    ///
    pub fn deserialize_event(x: &std::string::String) -> Result<Event> {
        serde_json::from_str(&x)
    }

    /// Serialize an `Event` to a `serde_json::Result<String>`
    /// 
    /// Example:
    /// ```
    /// use eddeserus::sede::*;
    /// let json = r#"["A","2010-01-01",null,"Enrollment",[],{"facts":{},"source":{"table":"a"}}]"#;
    /// let event = deserialize_event(&json.to_string()).unwrap();
    /// assert_eq!(json, serialize_event(&event).unwrap());
    /// ```
    ///
    pub fn serialize_event(x: &Event) -> Result<String> {
        serde_json::to_string(x)
    }
}

/// Provides functions for processing events (`&str` -> process -> `io::stdput`).
pub mod process{
    use crate::sede::serialize_event;
    use serde_json::{Deserializer};
    use crate::types::Event;
    use std::io::{self, Write};

    /// Process a string of events
    /// 
    /// For each `Event` in `events_json`, this function transforms each event
    /// by the `processor` function, outputting successfully processed events
    /// to `stdout` and unsucessful events to `stderr`.
    pub fn process_events(events_json: &str, 
                          processor: &mut dyn std::ops::Fn(Event) -> Event) 
                          -> () {
        
        let stream = Deserializer::from_str(&events_json).into_iter::<Event>();
        let stdout = io::stdout(); 
        let mut handle    = io::BufWriter::new(stdout.lock()); 
        let mut errhandle = io::BufWriter::new(io::stderr()); 

        for event in stream {
            let p = serialize_event(&processor(event.unwrap()));

            match p {
                Ok(v) =>  writeln!(handle, "{}", v).ok(), 
                // (.ok() added to hide warning: unused std::result:Result)
                Err(e) => writeln!(errhandle, "{}", e).ok(),
            };        
        }

    }
}

