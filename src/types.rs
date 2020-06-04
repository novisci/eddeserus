use serde::{Serialize, Deserialize};

/// The `Event` type. See []().
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub pid:     String,
    pub start:   Option<String>,
    pub end:     Option<String>,
    pub domain:  Domain,
    pub tags:    Vec<String>,
    pub context: serde_json::Value
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

