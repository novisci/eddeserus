
use crate::se::serialize_event;
use serde_json::{Deserializer};
use crate::types::Event;
use std::io::{self, Write};

/// TODO
pub fn process_events(events_json: &str, processor: &mut dyn std::ops::Fn(Event) -> Event) -> () {
    
    let stream = Deserializer::from_str(&events_json).into_iter::<Event>();
    let stdout = io::stdout(); 
    let mut handle    = io::BufWriter::new(stdout.lock()); 
    let mut errhandle = io::BufWriter::new(io::stderr()); 

    for event in stream {
        let p = serialize_event(&processor(event.unwrap()));

        match p {
            Ok(v) =>  writeln!(handle, "{}", v).ok(),
            Err(e) => writeln!(errhandle, "{}", e).ok(),
        };        
    }

}
