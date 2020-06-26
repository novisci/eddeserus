
use eddeserus::types::*;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};


fn deserialize(x: &String) -> () {
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


fn deserialize_demographics(c: &mut Criterion) {

    let mut group = c.benchmark_group("demographics");

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
            BenchmarkId::new("de", format!("{} demographics", &n)),
             &json,
            |b, j| b.iter(|| deserialize(j) ));


    }

}


fn deserialize_diagnosis(c: &mut Criterion) {

    let mut group = c.benchmark_group("diagnosis");

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
            BenchmarkId::new("de", format!("{} diagnosis", &n)),
             &json, 
             |b, j| b.iter(|| deserialize(j) ));

    }
}



fn deserialize_procedure(c: &mut Criterion) {

    let mut group = c.benchmark_group("procedure");

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
            BenchmarkId::new("de", format!("{} procedure", &n)),
             &json, 
            |b, j| b.iter(|| deserialize(j) ));

    }
}


criterion_group!(benches, 
    deserialize_demographics, 
    deserialize_diagnosis,
    deserialize_procedure);
criterion_main!(benches);