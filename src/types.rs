
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_tuple::*;

/// The `Event` type. See []().
#[derive(Serialize_tuple, Deserialize, Debug)]
pub struct Event {
    pub pid:      String,
    pub start:    Option<String>,
    pub end:      Option<String>,
    pub domain:   Domain,
    pub concepts: Vec<String>,
    // pub context:  serde_json::Value
    pub context:  Context
}

/// The `Domain` type. See []().
#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum Domain {
    /// The `Claim` type. See []().
    Claim,
    /// The `Demographic` type. See []().
    Demographic,
    /// The `Enrollment` type. See []().
    Enrollment, 
    /// The `Labs` type. See []().
    Labs,
    /// The `Medication` type. See []().
    Medication,
    /// The `Procedure` type. See []().
    Procedure,
}

// type HashString = HashMap<String, String>;

/// TODO
#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    // pub facts:  HashMap<String, HashOrString>,
    pub facts: serde_json::Value,
    pub source: Source
}

/// TODO
type Source = Option<HashMap<String, String>>;
