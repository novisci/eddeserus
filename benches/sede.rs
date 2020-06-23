
use eddeserus::types::*;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use serde_json::{Result};
use eddeserus::sede::*;
// use std::fs::File;
// use std::io::{Read};
// use std::io::{BufReader, BufRead};
// use serde_json::{Deserializer};


pub fn single_event_deserialize(c: &mut Criterion) {

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

    c.bench_with_input(BenchmarkId::new("single event", "claim"), &json, 
        |b, j| b.iter(|| {
            let ev : Result<Event> = deserialize_event(&j.to_string());
            ev
        }));

    let json = "\
         {\"domain\":\"Claim\",\
            \"patient_id\":\"xyz\",\
            \"time\":{\"begin\":0,\"end\":1},\
            \"facts\":{\
                \"claim\":{\"id\":\"claim1\"}\
             }\
         }";

    c.bench_with_input(BenchmarkId::new("single context", "claim"), &json, 
        |b, j| b.iter(|| {
            let ev : Result<ContextClaim> = serde_json::from_str(&j.to_string());
            ev
        }));
}


// pub fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("deserialize50_iter", {
//         |b| {


//           b.iter(|| {
//             let mut file = File::open("resources/50events.json").unwrap();
//             let mut contents = String::new();
//             file.read_to_string(&mut contents).unwrap();

//             let stream = Deserializer::from_str(&contents).into_iter::<Event>();
//             for event in stream {
//                 event.unwrap();
//             }

//           })
//         } 
//     });

//     c.bench_function("sede50_iter", {
//         |b| {


//           b.iter(|| {
//             let mut file = File::open("resources/50events.json").unwrap();
//             let mut contents = String::new();
//             file.read_to_string(&mut contents).unwrap();

//             let stream = Deserializer::from_str(&contents).into_iter::<Event>();

//             for event in stream {
//                 serialize_event(&event.unwrap()).ok();
//             }

//           })
//         } 
//     });

//     c.bench_function("deserialize50_buffer", {
//         |b| {

//           b.iter(|| {

//             let file = File::open("resources/50events.json").unwrap();
//             let data = Box::new(BufReader::new(file));
//             for line in data.lines() {
//                 deserialize_event(&line.unwrap()).ok();
//             }

//           })
//         } 
//     });

//     c.bench_function("sede50_buffer", {
//         |b| {

//           b.iter(|| {

//             let file = File::open("resources/50events.json").unwrap();
//             let data = Box::new(BufReader::new(file));
//             for line in data.lines() {
//                 let event = deserialize_event(&line.unwrap());
//                 serialize_event(&event.unwrap()).ok();
//             }

//           })
//         } 
//     });
// }

criterion_group!(benches, single_event_deserialize);
criterion_main!(benches);
