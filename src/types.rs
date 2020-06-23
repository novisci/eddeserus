//! The Rust internal representations of NoviSci EDM data types.

use serde::{Serialize, Deserialize};
use serde_tuple::*;
use serde::de::{self, Deserializer};
use std::fmt;
/*----------------------------------------------------------------------------*/
// [`Event`](https://docs.novisci.com/schema/event-data-model/1.0/#event-schema)

#[derive(Serialize_tuple, Debug)]
pub struct Event  {
    pub pid: serde_json::Value,
    pub begin: serde_json::Value,
    pub end: serde_json::Value,
    pub domain: serde_json::Value,
    // pub pid:      SubjectId,
    // pub begin:    TimeValue, 
    // pub end:      Option<TimeValue>,
    // pub domain:   String,
    pub concepts: Vec<String>,
    pub context:  Context
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(a {} event)", self.domain)
    }

}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {  
        use serde_json::{Value, from_value};

        #[derive(Deserialize, Debug)]
        struct EventHolder {
            pid: serde_json::Value,
            begin: serde_json::Value,
            end: serde_json::Value,
            domain: serde_json::Value,
            // pid :     SubjectId,
            // begin:    TimeValue, 
            // end:      Option<TimeValue>,
            // domain:   String,
            concepts: Vec<String>,
            ctxt:     Value,
        }

        let v = Value::deserialize(deserializer)?;
        let m = EventHolder::deserialize(&v).map_err(de::Error::custom)?;
        let d = Domain::deserialize(&m.ctxt).map_err(de::Error::custom)?;
        let x = m.ctxt;

        // Parse the context based on the domain
        let context = 
        match d {
            Domain::Claim => 
            Context::Claim(
                from_value::<ContextClaim>(x).map_err(de::Error::custom)?
            ),

            Domain::Demographics => 
            Context::Demographics(
                from_value::<ContextDemographics>(x).map_err(de::Error::custom)?
            ),

            Domain::Diagnosis => 
            Context::Diagnosis(
                from_value::<ContextDiagnosis>(x).map_err(de::Error::custom)?
            ),

            Domain::Eligibility => 
            Context::Eligibility(
                from_value::<ContextEligibilty>(x).map_err(de::Error::custom)?
            ),

            Domain::Enrollment => 
            Context::Enrollment(
                from_value::<ContextEnrollment>(x).map_err(de::Error::custom)?
            ),

            Domain::Labs => 
            Context::Labs(
                from_value::<ContextLabs>(x).map_err(de::Error::custom)?
            ),

            Domain::Medication => 
            Context::Medication(
                from_value::<ContextMedication>(x).map_err(de::Error::custom)?
            ),

            Domain::Procedure => 
            Context::Procedure(
                from_value::<ContextProcedure>(x).map_err(de::Error::custom)?
            ),
        };

        Ok(Event {
            pid      : m.pid,
            begin    : m.begin,
            end      : m.end,
            domain   : m.domain,
            concepts : m.concepts,
            context  : context,
        })

    }
}

#[cfg(test)]
mod test_event_context {
    use crate::types::Event;

    #[test]
    fn test1() {
        use serde_json::{from_str, to_string, Result};
        let json = "\
            [\"xyz\",\"2010-01-01\",null,\"Claim\",[],\
             {\"domain\":\"Claim\",\
                \"patient_id\":\"xyz\",\
                \"time\":{\"begin\":0,\"end\":1},\
                \"facts\":{\
                    \"claim\":{\"id\":\"claim1\"}\
                 }\
             }\
            ]";
        let evnt : Result<Event> = from_str(&json.to_string());
        println!("{:?}", &evnt);
        assert_eq!(json, to_string(&evnt.unwrap()).unwrap());
    }
}

/*----------------------------------------------------------------------------*/
// Domains
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "domain")]
pub enum Domain {
    Claim,
    Demographics,
    Diagnosis,
    Eligibility,
    Enrollment,
    Labs,
    Medication,
    Procedure,
}

/*----------------------------------------------------------------------------*/
// Contexts
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "domain")]
pub enum Context {
    Claim(ContextClaim),
    Demographics(ContextDemographics),
    Diagnosis(ContextDiagnosis),
    Eligibility(ContextEligibilty),
    Enrollment(ContextEnrollment),
    Labs(ContextLabs),
    Medication(ContextMedication),
    Procedure(ContextProcedure),
}

/*----------------------------------------------------------------------------*/
// Common types used in multiple domains and contexts

// type Source = Option<HashMap<String, String>>;
type Source = Option<serde_json::Value>;
type Misc = Option<serde_json::Value>;

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum Location {
    Unknown,
    Inpatient,
    Outpatient,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Code {
    pub code : String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub codebook : Option<Codebook>
}

type Codebook = String;

/*----------------------------------------------------------------------------*/
// Fact Types
type SubjectId = serde_json::Value;

// #[derive(PartialEq, Serialize, Deserialize, Debug)]
// #[serde(untagged)]
// pub enum SubjectId {
//     PidInt(u64),
//     IDString(String),
// }

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Claim {
    pub id:     String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub index:  Option<u32>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Cost {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,

    pub cost: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction:  Option<String>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Fill {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_supply: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength:  Option<String>,
}


type Time = serde_json::Value;
// #[derive(PartialEq, Serialize, Deserialize, Debug)]
// #[serde(untagged)]
// pub enum TimeValue {
//     TimeValueInt(u64),
//     TimeValueString(String),
// }

// #[derive(PartialEq, Serialize, Deserialize, Debug)]
// pub struct Time {
//     pub begin : TimeValue,

//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub end   : Option<TimeValue>,
// }
// #[cfg(test)]
// mod test_time {
//     use serde_json::{from_str, to_string, Result};
//     use crate::types::Time;

//     #[test]
//     fn test1() {
//         use serde_json::{from_str, to_string, Result};
//         let json = r#"{"begin":0,"end":10}"#;
//         let ctxt : Result<Time> = from_str(&json.to_string());
//         println!("The time is: {:?}", &ctxt);
//         assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
//     }

//     #[test]
//     fn test2() { 
//         let json = r#"{"begin":0}"#;
//         let ctxt : Result<Time> = from_str(&json.to_string());
//         println!("The time is: {:?}", &ctxt);
//         assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
//     }

//     #[test]
//     fn test3() { 
//         let json = r#"{"begin":"2010-01-01"}"#;
//         let ctxt : Result<Time> = from_str(&json.to_string());
//         println!("The time is: {:?}", &ctxt);
//         assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
//     }
// }

/*----------------------------------------------------------------------------*/
// Claim

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ContextClaim {

    pub patient_id : SubjectId,
    pub time :  Time,
    pub facts:  ClaimFacts,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<serde_json::Value>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ClaimFacts {

    pub claim : Claim,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location : Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost : Option<Cost>,

}

#[cfg(test)]
mod test_claim_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\"domain\":\"Claim\",\
                    \"patient_id\":123,\
                    \"time\":{\"begin\":0,\"end\":1},\
                    \"facts\":{\
                        \"claim\":{\"id\":\"claim1\"}\
                      }\
                    }";
        let ctxt : Result<Context> = from_str(&json.to_string());
        println!("{:?}", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}
/*----------------------------------------------------------------------------*/
// Demographics

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ContextDemographics {

    pub patient_id : SubjectId,
    pub time :  Time,
    pub facts:  DemographicFacts,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<serde_json::Value>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum DemographicField {
    BirthYear,
    BirthDate,
    Race,
    RaceCodes,
    Gender,
    Zipcode,
    County,
    State,
    Ethnicity,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct DemographicFacts {
    pub field:  DemographicField,
    pub info:   Option<serde_json::Value>,
}

#[cfg(test)]
mod test_demographic_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        use serde_json::{from_str, to_string, Result};
        let json = "{\"domain\":\"Demographics\",\
                    \"patient_id\":123,\
                    \"time\":{\"begin\":0,\"end\":1},\
                    \"facts\":{\"field\":\"BirthYear\",\"info\":\"1980\"}\
                    }";
        let ctxt : Result<Context> = from_str(&json.to_string());
        println!("{:?}", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }

    #[test]
    fn test2() {
        let json = "{\"domain\":\"Demographics\",\
                    \"patient_id\":123,\
                    \"time\":{\"begin\":0,\"end\":1},\
                    \"facts\":{\"field\":\"RaceCodes\",\"info\":[\"some\",\"info\"]}\
                    }";
        let ctxt : Result<Context> = from_str(&json.to_string());
        println!("{:?}", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }

    #[test]
    fn test3() {
        let json = "{\"domain\":\"Demographics\",\
                    \"patient_id\":123,\
                    \"time\":{\"begin\":0,\"end\":1},\
                    \"facts\":{\"field\":\"RaceCodes\",\"info\":null}\
                    }";
        let ctxt : Result<Context> = from_str(&json.to_string());
        println!("{:?}", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}

/*----------------------------------------------------------------------------*/
// Diagnosis

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ContextDiagnosis {

    // pub patient_id : SubjectId, 
    pub patient_id : serde_json::Value,
    pub time : serde_json::Value,
    // pub time :  Time,
    pub facts: DiagnosisFacts,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Source,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Misc,
}


#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct DiagnosisFacts {
    pub code    : Code,
    // pub code   : serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    // pub claim   : Option<Claim>,
    pub claim   : Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}


#[cfg(test)]
mod test_diagnosis_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\
            \"domain\":\"Diagnosis\",\
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"facts\":{\"code\":{\"code\":\"Z21\"},\
            \"location\":\"Outpatient\"}\
            }";
        let ctxt : Result<Context> = from_str(&json.to_string());
        println!("{:?}", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}

/*----------------------------------------------------------------------------*/
// Eligibility

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ContextEligibilty {

    pub patient_id : SubjectId,
    pub time :  Time,
    pub facts:  EligibilityFacts,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<serde_json::Value>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct EligibilityFacts {}

#[cfg(test)]
mod test_eligibility_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\
            \"domain\":\"Eligibility\",\
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"facts\":{}\
            }";
        let ctxt : Result<Context> = from_str(&json.to_string());
        println!("{:?}", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}

/*----------------------------------------------------------------------------*/
// Enrollment

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ContextEnrollment {

    pub patient_id : SubjectId,
    pub time :  Time,
    pub facts:  EnrollmentFacts,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<serde_json::Value>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct EnrollmentFacts {}

#[cfg(test)]
mod test_enrollment_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\
            \"domain\":\"Enrollment\",\
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"facts\":{}\
            }";
        let ctxt : Result<Context> = from_str(&json.to_string());
        println!("{:?}", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}

/*----------------------------------------------------------------------------*/
// Labs

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ContextLabs {

    pub patient_id : SubjectId,
    pub time :  Time,
    pub facts: LabsFacts,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<serde_json::Value>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct LabValue {

  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub number: Option<f64>,
  
  pub units: String
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct LabsFacts {
    pub code    : Code,
    pub value   : LabValue,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim   : Option<Claim>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

#[cfg(test)]
mod test_labs_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\
            \"domain\":\"Labs\",\
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"facts\":{\"code\":{\"code\":\"L21\"},\
                       \"value\":{\"number\":0.1,\"units\":\"mg\"}}\
            }";
        let ctxt : Result<Context> = from_str(&json.to_string());
        println!("{:?}", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}

/*----------------------------------------------------------------------------*/
// Medication

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ContextMedication {

    pub patient_id : SubjectId,
    pub time :  Time,
    pub facts: MedicationFacts,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<serde_json::Value>,
}


#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct MedicationFacts {
    pub code    : Code,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fill>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim : Option<Claim>,
}

#[cfg(test)]
mod test_medication_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\
            \"domain\":\"Medication\",\
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"facts\":{\"code\":{\"code\":\"A21\"},\
                       \"location\":\"Inpatient\"}\
            }";
        let ctxt : Result<Context> = from_str(&json.to_string());
        println!("{:?}", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}
/*----------------------------------------------------------------------------*/
// Procedure

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ContextProcedure {

    pub patient_id : SubjectId,
    pub time :  Time,
    pub facts: ProcedureFacts,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<serde_json::Value>,
}


#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct ProcedureFacts {
    pub code    : Code,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim   : Option<Claim>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

#[cfg(test)]
mod test_procedure_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\
            \"domain\":\"Procedure\",\
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"facts\":{\"code\":{\"code\":\"A21\"},\
                       \"location\":\"Inpatient\"}\
            }";
        let ctxt : Result<Context> = from_str(&json.to_string());
        println!("{:?}", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}