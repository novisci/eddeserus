
use eddeserus::experiment::*;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn deserialize_orig(x: &String) -> () {
    let stream = serde_json::Deserializer::from_str(&x)
                 .into_iter::<eddeserus::types::Event>();

    for event in stream {
        event.unwrap();
    }
}

fn deserialize_new(x: &String) -> () {
    let stream = serde_json::Deserializer::from_str(&x)
                 .into_iter::<Event>();

    for event in stream {
        event.unwrap();
    }
}

fn replicate_event(x: &String, n: u32) -> String {
    let mut out = String::new();
    for _i in 0..n {
        out = format!("{}{}", &out, &x)
    }
    out
}


fn deserialize_demographic_experiments(c: &mut Criterion) {

    let mut group = c.benchmark_group("demographics_experiments");

    let val = "[\
    \"xyz\",2,null,\"Demographics\",[],\
    {\"domain\":\"Demographics\",\
     \"patient_id\":\"abc\",\
     \"time\":{\"begin\":0,\"end\":1},\
     \"facts\":{\"field\":\"BirthYear\",\"info\":\"1980\"},\
     \"source\":{\"table\":\"somewhere\",\"db\":\"optum\"},\
     \"misc\":{\"key1\":\"val1\",\"key2\":\"val2\"}}\
    ]\n".to_string();

    for n in [4, 16, 256].iter() {

        let json = replicate_event(&val, *n);

        // Demographics
        group.bench_with_input(
            BenchmarkId::new("orig", format!("{} demographics", &n)),
             &json,
            |b, j| b.iter(|| deserialize_orig(j) ));

        group.bench_with_input(
            BenchmarkId::new("new", format!("{} demographics", &n)),
             &json,
            |b, j| b.iter(|| deserialize_new(j) ));

    }

}


fn deserialize_diagnosis_experiments(c: &mut Criterion) {

    let mut group = c.benchmark_group("diagnosis_experiments");

    let val = "[\
    \"abc\",2,null,\"Diagnosis\",[],\
    {\"domain\":\"Diagnosis\",\
     \"patient_id\":\"abc\",\
     \"time\":{\"begin\":0,\"end\":1},\
     \"facts\":{\"code\":{\"code\":\"99.01\",\"codebook\":\"ICD\"},\
               \"location\":\"Inpatient\",\
               \"claim\":{\"id\":\"98918\",\"index\":900}},\
     \"source\":{\"table\":\"somewhere\",\"db\":\"optum\"},\
     \"misc\":{\"key1\":\"val1\",\"key2\":\"val2\",\"key3\":\"val3\",\
               \"key4\":\"val4\",\"key5\":\"val5\",\"key5\":\"val5\"}}\
    ]\n".to_string();


    for n in [4, 16, 256].iter() {

        let json = replicate_event(&val, *n);
        // Diagnosis
        group.bench_with_input(
            BenchmarkId::new("orig", format!("{} diagnosis", &n)),
             &json, 
             |b, j| b.iter(|| deserialize_orig(j) ));

        group.bench_with_input(
            BenchmarkId::new("new", format!("{} diagnosis", &n)),
             &json, 
            |b, j| b.iter(|| deserialize_new(j) ));

    }
}



fn deserialize_procedure_experiments(c: &mut Criterion) {

    let mut group = c.benchmark_group("procedure_experiments");

    let val = "[\
    \"abc\",2,null,\"Procedure\",[],\
    {\"domain\":\"Procedure\",\
     \"patient_id\":\"abc\",\
     \"time\":{\"begin\":0,\"end\":1},\
     \"facts\":{\"code\":{\"code\":\"99.01\",\"codebook\":\"CPT\"},\
               \"location\":\"Outpatient\"}}\
    ]\n".to_string();


    for n in [4, 16, 256].iter() {

        let json = replicate_event(&val, *n);

        // Procedure
        group.bench_with_input(
            BenchmarkId::new("orig", format!("{} procedure", &n)),
             &json, 
            |b, j| b.iter(|| deserialize_orig(j) ));


        group.bench_with_input(
            BenchmarkId::new("new", format!("{} procedure", &n)),
             &json, 
            |b, j| b.iter(|| deserialize_new(j) ));

    }
}


criterion_group!(benches, 
    deserialize_demographic_experiments, 
    deserialize_diagnosis_experiments,
    deserialize_procedure_experiments);
criterion_main!(benches);