//! The Rust internal representations of NoviSci EDM data types.

use serde_json::value::RawValue;
use serde::{Deserialize, Serialize};
use serde_tuple::*;

/*----------------------------------------------------------------------------*/
/// Shared types
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Interval {
    IntervalInt { begin : u64,    end : Option<u64> },
    IntervalStr { begin : String, end : Option<String> },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SubjectID<'a> {
    IDstr(&'a str),
    Idint(u64),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Location {
    Unknown,
    Inpatient,
    Outpatient,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Code<'a> {
    pub code : &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    // pub codebook : Option<&'a str>
    pub codebook : Option<Codebook>
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Codebook {
    CDT,
    CPT,
    HCPCS,
    ICD9,
    ICD9PC,
    ICD10,
    ICD10PC,
    LOINC,
    #[serde(rename(serialize = "medicaid_cat", deserialize = "medicaid_cat"))]
    MedicaidCat,
    NABSP,
    NDC,
    NDC9,
    UB92,
    #[serde(rename(serialize = "US_STATE", deserialize = "US_STATE"))]
    USSTATE
}

#[cfg(test)]
mod test_code {
    use crate::types::*;

    #[test]
    fn test_medicaid_cat() {
        use serde_json::{from_str, to_string, Result};

        let json = "\
        {\
         \"code\":\"abc\",\
         \"codebook\":\"medicaid_cat\"\
        }".to_string();

        let code : Result<Code> = from_str(&json);
        println!("Code\n{:?}\n", &code);
        assert_eq!(json, to_string(&code.unwrap()).unwrap());
    }

    #[test]
    fn test_us_state() {
        use serde_json::{from_str, to_string, Result};

        let json = "\
        {\
         \"code\":\"abc\",\
         \"codebook\":\"US_STATE\"\
        }".to_string();

        let code : Result<Code> = from_str(&json);
        println!("Code\n{:?}\n", &code);
        assert_eq!(json, to_string(&code.unwrap()).unwrap());
    }

    #[test]
    fn test_icd10pc() {
        use serde_json::{from_str, to_string, Result};

        let json = "\
        {\
         \"code\":\"abc\",\
         \"codebook\":\"ICD10PC\"\
        }".to_string();

        let code : Result<Code> = from_str(&json);
        println!("Code\n{:?}\n", &code);
        assert_eq!(json, to_string(&code.unwrap()).unwrap());
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claim<'a> {
    pub id:  &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub index:  Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure:  Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Cost<'a> {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<&'a str>,

    pub cost: &'a str, //TODO: EDM type is <Text | Double>

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed:  Option<&'a str>, //TODO: EDM type is Optional <Text | Double>

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction:  Option<&'a str>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Fill<'a> {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_supply: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength:  Option<&'a str>,
}


/*----------------------------------------------------------------------------*/
// [`Event`](https://docs.novisci.com/schema/event-data-model/1.0/#event-schema)
#[derive(Debug, Deserialize, Serialize_tuple)]
pub struct Event<'a> {
    pub p : SubjectID<'a>,
    pub b : &'a RawValue,
    pub e : &'a RawValue,
    pub d : &'a str,
    pub concepts : Vec<String>,
    pub context : Context<'a>,
}


#[cfg(test)]
mod test_events {
    use crate::types::*;

    #[test]
    fn test1() {
        use serde_json::{from_str, to_string, Result};

        let json = "[\
        \"xyz\",2,null,\"Demographics\",[],\
        {\
         \"patient_id\":\"abc\",\
         \"time\":{\"begin\":0,\"end\":1},\
         \"domain\":\"Demographics\",\
         \"facts\":{\"field\":\"BirthYear\",\"info\":\"1980\"},\
         \"source\":{\"table\":\"somewhere\",\"db\":\"optum\"},\
         \"misc\":{\"key1\":\"val1\",\"key2\":\"val2\"}\
        }]".to_string();

        let evnt : Result<Event> = from_str(&json);
        println!("Demographics event\n{:?}\n", &evnt);
        assert_eq!(json, to_string(&evnt.unwrap()).unwrap());
    }
}


/*----------------------------------------------------------------------------*/
// Contexts

#[derive(Debug, Deserialize, Serialize)]
pub struct Context<'a> {
    #[serde(bound(deserialize = "SubjectID<'a>: Deserialize<'de>"))]
    pub patient_id : SubjectID<'a>,
    pub time : Interval,

    #[serde(bound(deserialize = "Facts<'a>: Deserialize<'de>"))]
    #[serde(flatten)]
    pub facts : Facts<'a>,

    #[serde(bound(deserialize = "&'a RawValue: Deserialize<'de>"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a RawValue>,

    #[serde(bound(deserialize = "&'a RawValue: Deserialize<'de>"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<&'a RawValue>,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "domain", content = "facts")]
pub enum Facts<'a> {
    #[serde(bound(deserialize = "Claim<'a>: Deserialize<'de>, Cost<'a>: Deserialize<'de>"))]
    Claim(ClaimFacts<'a>),

    Death(DeathFacts),
    Demographics(DemographicFacts),

    #[serde(bound(deserialize = "Code<'a>: Deserialize<'de>"))]
    Diagnosis(DiagnosisFacts<'a>),
    
    Eligibility(EligibilityFacts),
    Enrollment(EnrollmentFacts),

    #[serde(bound(deserialize = 
        "LabValue<'a>: Deserialize<'de>,
         Code<'a>: Deserialize<'de>,
         Claim<'a>: Deserialize<'de>"))]
    Labs(LabsFacts<'a>),

    #[serde(bound(deserialize = 
        "Fill<'a>: Deserialize<'de>,
         Code<'a>: Deserialize<'de>,
         Claim<'a>: Deserialize<'de>"))]
    Medication(MedicationFacts<'a>),
    
    #[serde(bound(deserialize = 
         "Code<'a>: Deserialize<'de>,
          Claim<'a>: Deserialize<'de>"))]
    Procedure(ProcedureFacts<'a>),

    // NOTE: Undefined domain disallowed after final transformation to
    //       events, thus this domain is not included. 
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimFacts<'a> {
    #[serde(bound(deserialize = "Claim<'a>: Deserialize<'de>"))]
    pub claim : Claim<'a>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location : Option<Location>,

    #[serde(bound(deserialize = "Cost<'a>: Deserialize<'de>"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost : Option<Cost<'a>>,

}

#[cfg(test)]
mod test_claim_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\
                    \"patient_id\":123,\
                    \"time\":{\"begin\":0,\"end\":1},\
                    \"domain\":\"Claim\",\
                    \"facts\":{\
                        \"claim\":{\"id\":\"claim1\"}\
                      }\
                    }".to_string();
        let ctxt : Result<Context> = from_str(&json);
        println!("Claim context:\n{:?}\n", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }


    #[test]
    fn test2() {
        let json = "{\
                    \"patient_id\":123,\
                    \"time\":{\"begin\":0,\"end\":1},\
                    \"domain\":\"Claim\",\
                    \"facts\":{\
                        \"claim\":{\"id\":\"claim1\"},\
                        \"cost\":{\"charge\":\"uiui\",\"cost\":\"99\"}\
                      }\
                    }".to_string();
        let ctxt : Result<Context> = from_str(&json);
        println!("Claim context:\n{:?}\n", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}


/*----------------------------------------------------------------------------*/
// Death

#[derive(Debug, Deserialize, Serialize)]
pub struct DeathFacts {}

#[cfg(test)]
mod test_death_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"domain\":\"Death\",\
            \"facts\":{}\
            }".to_string();
        let ctxt : Result<Context> = from_str(&json);
        println!("Death context\n{:?}\n", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}


/*----------------------------------------------------------------------------*/
// Demographics

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub enum DemographicField {
    BirthYear,
    BirthDate,
    Race,
    RaceCodes,
    Gender,
    Zipcode,
    County,
    CountyFIPS,
    State,
    Ethnicity,
    Region,
    UrbanRural
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
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
        use serde_json::{from_str, Result};
        let json = "{\
                    \"patient_id\":\"abc\",\
                    \"time\":{\"begin\":0,\"end\":1},\
                    \"domain\":\"Demographics\",\
                    \"facts\":{\"field\":\"BirthYear\",\"info\":\"1980\"},\
                    \"source\":{\"table\":\"somewhere\",\"db\":\"optum\"},\
                    \"misc\":{\"key1\":\"val1\",\"key2\":\"val2\"}\
                    }".to_string();
        let ctxt : Result<Context> = from_str(&json);
        println!("Demographic context:\n{:?}\n", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }

    #[test]
    fn test2() {
        let json = "{\
                    \"patient_id\":\"abc\",\
                    \"time\":{\"begin\":0,\"end\":1},\
                    \"domain\":\"Demographics\",\
                    \"facts\":{\"field\":\"RaceCodes\",\"info\":[\"some\",\"info\"]},\
                    \"source\":{\"table\":\"somewhere\",\"db\":\"optum\"},\
                    \"misc\":{\"key1\":\"val1\",\"key2\":\"val2\"}\
                    }".to_string();
        let ctxt : Result<Context> = from_str(&json);
        println!("Demographic context:\n{:?}\n", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}

/*----------------------------------------------------------------------------*/
// Diagnosis

#[derive(Debug, Deserialize, Serialize)]
pub struct DiagnosisFacts<'a> {
    #[serde(bound(deserialize = "Code<'a>: Deserialize<'de>"))]
    pub code    : Code<'a>,
    
    #[serde(bound(deserialize = "Claim<'a>: Deserialize<'de>"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim   : Option<Claim<'a>>,

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
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"domain\":\"Diagnosis\",\
            \"facts\":{\
                \"code\":{\"code\":\"Z21\"},\
                \"location\":\"Outpatient\"}\
            }".to_string();
        let ctxt : Result<Context> = from_str(&json);
        println!("Diagnosis context\n{:?}\n", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}

/*----------------------------------------------------------------------------*/
// Eligibility

#[derive(Debug, Deserialize, Serialize)]
pub struct EligibilityFacts {}

#[cfg(test)]
mod test_eligibility_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"domain\":\"Eligibility\",\
            \"facts\":{}\
            }".to_string();
        let ctxt : Result<Context> = from_str(&json);
        println!("Eligibility context\n{:?}\n", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}

/*----------------------------------------------------------------------------*/
// Enrollment

#[derive(Debug, Deserialize, Serialize)]
pub struct EnrollmentFacts {}

#[cfg(test)]
mod test_enrollment_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"domain\":\"Enrollment\",\
            \"facts\":{}\
            }".to_string();
        let ctxt : Result<Context> = from_str(&json);
        println!("Enrollment context:\n{:?}\n", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}


/*----------------------------------------------------------------------------*/
// Labs

#[derive(Debug, Deserialize, Serialize)]
pub struct LabValue<'a> {

  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub number: Option<f64>,
  
  pub units: &'a str
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LabsFacts<'a> {

    #[serde(bound(deserialize = "Code<'a>: Deserialize<'de>"))]
    pub code    : Code<'a>,

    #[serde(bound(deserialize = "LabValue<'a>: Deserialize<'de>"))]
    pub value   : LabValue<'a>,

    #[serde(bound(deserialize = "Claim<'a>: Deserialize<'de>"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim   : Option<Claim<'a>>,

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
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"domain\":\"Labs\",\
            \"facts\":{\"code\":{\"code\":\"L21\"},\
                       \"value\":{\"number\":0.1,\"units\":\"mg\"}}\
            }".to_string();
        let ctxt : Result<Context> = from_str(&json);
        println!("Lab context\n{:?}\n", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}

/*----------------------------------------------------------------------------*/
// Medication

#[derive(Debug, Deserialize, Serialize)]
pub struct MedicationFacts<'a> {

    #[serde(bound(deserialize = "Code<'a>: Deserialize<'de>"))]
    pub code    : Code<'a>,

    #[serde(bound(deserialize = "Fill<'a>: Deserialize<'de>"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Fill<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    #[serde(bound(deserialize = "Claim<'a>: Deserialize<'de>"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim : Option<Claim<'a>>,
}

#[cfg(test)]
mod test_medication_context {
    use serde_json::{from_str, to_string, Result};
    use crate::types::Context;

    #[test]
    fn test1() {
        let json = "{\
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"domain\":\"Medication\",\
            \"facts\":{\"code\":{\"code\":\"A21\"},\
                       \"location\":\"Inpatient\"}\
            }".to_string();
        let ctxt : Result<Context> = from_str(&json);
        println!("Medication context:\n{:?}\n", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}


/*----------------------------------------------------------------------------*/
// Procedure

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcedureFacts<'a> {
    #[serde(bound(deserialize = "Code<'a>: Deserialize<'de>"))]
    pub code    : Code<'a>,

    #[serde(bound(deserialize = "Claim<'a>: Deserialize<'de>"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim   : Option<Claim<'a>>,

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
            \"patient_id\":123,\
            \"time\":{\"begin\":0,\"end\":1},\
            \"domain\":\"Procedure\",\
            \"facts\":{\"code\":{\"code\":\"A21\"},\
                       \"location\":\"Inpatient\"}\
            }".to_string();
        let ctxt : Result<Context> = from_str(&json);
        println!("{:?}", &ctxt);
        assert_eq!(json, to_string(&ctxt.unwrap()).unwrap());
    }
}