
use serde_json::value::RawValue;
use serde::{Deserialize};

/*----------------------------------------------------------------------------*/
/// Shared types
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Interval {
    IntervalInt { begin : u64,    end : Option<u64> },
    IntervalStr { begin : String, end : Option<String> },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SubjectID<'a> {
    IDstr(&'a str),
    Idint(u64),
}


#[derive(Debug, Deserialize)]
pub struct Event1<'a> {
    patient_id : SubjectID<'a>,
    begin : &'a RawValue,
    end   : &'a RawValue,
    domain : &'a str,
    pub tags : Vec<&'a str>,
    pub context    : Context1<'a>,
}

#[derive(Debug, Deserialize)]
pub struct Context1<'a> {
    patient_id : SubjectID<'a>,
    domain : &'a str,
    facts : serde_json::Value,
    time : Interval,
    source: &'a RawValue,
    misc : &'a RawValue,
}

#[derive(Debug, Deserialize)]
pub struct Event2 {
    pub pid : String,
    pub begin : u64,
    pub end : Option<u64>,
    pub domain : String,
    pub tags : Vec<String>,
    pub context  : Context2,
}

#[derive(Debug, Deserialize)]
pub struct Context2 {
    pub patient_id : String,
    pub domain : String,
    pub time : Interval,
    pub facts : serde_json::Value,
    pub misc : Option<serde_json::Value>
}


#[derive(Debug, Deserialize)]
pub struct Event3<'a> {
    pub p : SubjectID<'a>,
    pub b : &'a RawValue,
    pub e : &'a RawValue,
    pub d : &'a str,
    pub tags : Vec<&'a str>,
    pub context : Context3<'a>,
}




#[cfg(test)]
mod test_event_experiment {
    use crate::experiment::*;

    #[test]
    fn test1() {
        use serde_json::{from_str, Result};

        let json = "[\
        \"xyz\",2,null,\"Demographics\",[],\
        {\"domain\":\"Demographics\",\
         \"patient_id\":\"abc\",\
         \"time\":{\"begin\":0,\"end\":1},\
         \"facts\":{\"field\":\"BirthYear\",\"info\":\"1980\"},\
         \"source\":{\"table\":\"somewhere\",\"db\":\"optum\"},\
         \"misc\":{\"key1\":\"val1\",\"key2\":\"val2\"}\
        }]".to_string();

        let evnt : Result<Event1> = from_str(&json);
        println!("{:?}", &evnt);
        // assert_eq!(json, to_string(&evnt.unwrap()).unwrap());
    }

    #[test]
    fn test2() {
        use serde_json::{from_str, Result};

        let json = "[\
        \"xyz\",2,null,\"Demographics\",[],\
        {\"domain\":\"Demographics\",\
         \"patient_id\":\"abc\",\
         \"time\":{\"begin\":0,\"end\":1},\
         \"facts\":{\"field\":\"BirthYear\",\"info\":\"1980\"},\
         \"source\":{\"table\":\"somewhere\",\"db\":\"optum\"},\
         \"misc\":{\"key1\":\"val1\",\"key2\":\"val2\"}\
        }]".to_string();

        let evnt : Result<Event2> = from_str(&json);
        println!("{:?}", &evnt);
        // assert_eq!(json, to_string(&evnt.unwrap()).unwrap());
    }

    #[test]
    fn test3() {
        use serde_json::{from_str, Result};

        let json = "[\
        \"xyz\",2,null,\"Demographics\",[],\
        {\"domain\":\"Demographics\",\
         \"patient_id\":\"abc\",\
         \"time\":{\"begin\":0,\"end\":1},\
         \"facts\":{\"field\":\"BirthYear\",\"info\":\"1980\"},\
         \"source\":{\"table\":\"somewhere\",\"db\":\"optum\"},\
         \"misc\":{\"key1\":\"val1\",\"key2\":\"val2\"}\
        }]".to_string();

        let evnt : Result<Event3> = from_str(&json);
        println!("{:?}\n", &evnt);
        // assert_eq!(json, to_string(&evnt.unwrap()).unwrap());
    }
}


/*----------------------------------------------------------------------------*/
// Contexts
// #[derive(Deserialize, Debug)]
// #[serde(tag = "domain")]
// pub enum Context<'a> {
//     #[serde(bound(deserialize = "ContextClaim<'a>: Deserialize<'de>"))]
//     Claim(ContextClaim<'a>),
//     #[serde(bound(deserialize = "ContextDemographics<'a>: Deserialize<'de>"))]
//     Demographics(ContextDemographics<'a>),
//     // Diagnosis(ContextDiagnosis),
//     // Eligibility(ContextEligibilty),
//     // Enrollment(ContextEnrollment),
//     // Labs(ContextLabs),
//     // Medication(ContextMedication),
//     // Procedure(ContextProcedure),
// }

#[derive(Deserialize, Debug)]

pub struct Context3<'a> {
    #[serde(bound(deserialize = "SubjectID<'a>: Deserialize<'de>"))]
    patient_id : SubjectID<'a>,
    time : Interval,
    
    #[serde(bound(deserialize = "&'a RawValue: Deserialize<'de>"))]
    source: &'a RawValue,

    #[serde(bound(deserialize = "&'a RawValue: Deserialize<'de>"))]
    misc:   &'a RawValue,

    #[serde(flatten)]
    facts : Facts,
}


#[derive(Deserialize, Debug)]
#[serde(tag = "domain", content = "facts")]
// #[serde(untagged)]
pub enum Facts {
    Claim(ClaimFacts),
    Demographics(DemographicFacts),
    // Diagnosis(ContextDiagnosis),
    // Eligibility(ContextEligibilty),
    // Enrollment(ContextEnrollment),
    // Labs(ContextLabs),
    // Medication(ContextMedication),
    // Procedure(ContextProcedure),
}


#[derive(Debug, Deserialize)]
pub struct ContextClaim<'a> {
    field: &'a str,
}

// #[derive(Debug, Deserialize)]
// pub struct ContextDemographics<'a> {
//     field: &'a str,
// }
#[derive(Debug, Deserialize)]
pub struct ClaimFacts {
    field: String,
}

/*----------------------------------------------------------------------------*/
// Demographics

#[derive(Deserialize, Debug)]
pub struct ContextDemographics<'a> {
    pub patient_id : &'a str,
    pub time :  Interval,
    pub facts:  DemographicFacts,

    // #[serde(skip_serializing_if = "Option::is_none")]
    //pub source: &'a RawValue,
    pub source: serde_json::Value,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub misc: &'a RawValue,
    pub misc: serde_json::Value,
}

#[derive(PartialEq, Deserialize, Debug)]
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

pub enum StringOrVecString {
    SingleString(String),
    VecString(Vec<String>),
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct DemographicFacts {
    pub field:  DemographicField,
    pub info:   Option<serde_json::Value>,
}

// #[cfg(test)]
// mod test_demographic_context {
//     use serde_json::{from_str, Result};
//     // use crate::types::Context;
//     use crate::experiment::Context;

//     #[test]
//     fn test1() {
//         use serde_json::{from_str, Result};
//         let json = "{\"domain\":\"Demographics\",\
//                     \"patient_id\":\"abc\",\
//                     \"time\":{\"begin\":0,\"end\":1},\
//                     \"facts\":{\"field\":\"BirthYear\",\"info\":\"1980\"},\
//                     \"source\":{\"table\":\"somewhere\",\"db\":\"optum\"},\
//                      \"misc\":{\"key1\":\"val1\",\"key2\":\"val2\"}\
//                     }".to_string();
//         let ctxt : Result<Context> = from_str(&json);
//         println!("{:?}\n", &ctxt);
//         // assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
//     }

//     #[test]
//     fn test2() {
//         let json = "{\"domain\":\"Demographics\",\
//                     \"patient_id\":\"abc\",\
//                     \"time\":{\"begin\":0,\"end\":1},\
//                     \"facts\":{\"field\":\"RaceCodes\",\"info\":[\"some\",\"info\"]},\
//                     \"source\":{\"table\":\"somewhere\",\"db\":\"optum\"},\
//                     \"misc\":{\"key1\":\"val1\",\"key2\":\"val2\"}\
//                     }".to_string();
//         let ctxt : Result<Context> = from_str(&json);
//         println!("{:?}\n", &ctxt);
//         // assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
//     }

// }