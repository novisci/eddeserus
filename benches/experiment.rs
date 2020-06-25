use eddeserus::experiment::*;
use serde_json::{from_str, Result};
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use std::fs::{File};
use std::io::{Read};

// use std::io::{BufReader};

pub fn event_experiments(c: &mut Criterion) {

    let json = "[\"xyz\",2,null,\"Claim\",[],{\"patient_id\":\"xyz\",\"time\":{\"begin\":0}}]".to_string();

    let mut group = c.benchmark_group("event experiments");

    group.bench_with_input(
        BenchmarkId::new("event1 experiment", "claim"), 
        &json, 
        |b, j| b.iter(|| {

            let ev : Result<Event1> = from_str(black_box(&j));
            ev
        }));

    group.bench_with_input(
        BenchmarkId::new("event2 experiment", "claim"), 
        &json, 
        |b, j| b.iter(|| {
            let ev : Result<Event2> = from_str(&j);
            ev
        }));


    let mut f = File::open("benches/10demo_events.jsonl").unwrap(); 
    let mut demo_json = String::new();
    f.read_to_string(&mut demo_json).unwrap();

    // let demo_json = read_to_string(File::open(filename).unwrap()).unwrap();
    // let demo_json = "\.
    // [\"xyz\",2,null,\"Claim\",[],\
    // {\"domain\":\"Demographics\",\
    //  \"patient_id\":\"abc\",\
    //  \"time\":{\"begin\":0,\"end\":1},\
    //  \"facts\":{\"field\":\"BirthYear\",\"info\":\"1980\"}\
    // }]".to_string();

    group.bench_with_input(
        BenchmarkId::new("event_orig experiment", "demographics"),
         &demo_json, 
        |b, j| b.iter(|| {

            let stream = serde_json::Deserializer::from_str(j).into_iter::<eddeserus::types::Event>();

            for event in stream {
                event.unwrap();
            }
            // let ev : Result<eddeserus::types::Event> = from_str(&j);
            // ev
        }));

    group.bench_with_input(
        BenchmarkId::new("event1 experiment", "demographics"),
         &demo_json, 
        |b, j| b.iter(|| {
            let stream = serde_json::Deserializer::from_str(j).into_iter::<Event1>();
            for event in stream {
                event.unwrap();
            }
            // let ev : Result<Event1> = from_str(&j);
            // ev
        }));

    group.bench_with_input(
        BenchmarkId::new("event2 experiment", "demographics"), 
        &demo_json, 
        |b, j| b.iter(|| {
            let stream = serde_json::Deserializer::from_str(j).into_iter::<Event2>();
            for event in stream {
                // println!("{:?}", event);
                event.unwrap();
            }
            // let ev : Result<Event2> = from_str(&j);
            // ev
        }));

    group.bench_with_input(
        BenchmarkId::new("event3 experiment", "demographics"),
         &demo_json, 
        |b, j| b.iter(|| {
            let stream = serde_json::Deserializer::from_str(j).into_iter::<Event3>();
            for event in stream {
                // println!("{:?}", event);
                event.unwrap();
            }

            // let ev : Result<Event3> = from_str(&j);
            // ev
        }));



}


pub fn event_contexts(c: &mut Criterion) {

    let demographics_json = "\
    {\"domain\":\"Demographics\",\
     \"patient_id\":\"abc\",\
     \"time\":{\"begin\":0,\"end\":1},\
     \"facts\":{\"field\":\"BirthYear\",\"info\":\"1980\"}\
    }".to_string();

    let mut group = c.benchmark_group("context experiments");

    // group.bench_with_input(
    //     BenchmarkId::new("context experiment", "Demographics"),
    //     &demographics_json, 
    //     |b, j| b.iter(|| {

    //         let ev : Result<eddeserus::experiment::Context> = from_str(&j);
    //         ev
    //     }));

    // group.bench_with_input(
    //     BenchmarkId::new("context orginal", "Demographics"),
    //     &demographics_json, 
    //     |b, j| b.iter(|| {

    //         let ev : Result<eddeserus::types::Context> = from_str(&j);
    //         ev
    //     }));


}

criterion_group!(benches, event_experiments, event_contexts);
criterion_main!(benches);