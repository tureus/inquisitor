use serde::Serialize;
use serde_json;

use hashbrown::HashMap;

pub trait JSONDataBuilder<T: Serialize + Sized> {
    fn new(target_size: usize) -> Self;
    fn json(&self) -> Vec<T>;
}

pub struct MetricDataBuilder {
    target_size: usize,
}

#[derive(Serialize)]
pub struct MetricData {
    metric: String,
    tags: hashbrown::HashMap<String,f64>
}

impl MetricData {
    fn random() -> Self {
        let tag_vec = vec![
            ("nice.metric".to_owned(), 1.0f64),
            ("how.about.pi".to_owned(), 3.14f64),
            ("yes.please.i.like.nesting.hmm".to_owned(), 1000000f64),
            ("yes.please.i.like.nesting.more".to_owned(), 1000000f64),
            ("yes.please.i.like.nesting.more2".to_owned(), 1000000f64),
            ("yes.please.i.like.nesting.more100".to_owned(), 1000000f64),
        ];

        let mut tags : HashMap<String,f64> = HashMap::new();

        for (k,v) in tag_vec.into_iter() {
            tags.insert(k,v);
        }

        MetricData {
            metric: "something.random.and.cool".into(),
            tags
        }
    }
}

impl JSONDataBuilder<MetricData> for MetricDataBuilder where {
    fn new(target_size: usize) -> Self {
        Self{ target_size }
    }

    fn json(&self) -> Vec<MetricData> {
        let sample_metric = MetricData::random();
        let sample_blob = serde_json::to_vec(&sample_metric).unwrap();

        if sample_blob.len() > self.target_size {
            vec![sample_metric]
        } else {
            let num_samples_need = self.target_size / sample_blob.len();
            let samples : Vec<_>= (0..num_samples_need).map(|_| MetricData::random()).collect();
            samples
        }
    }
}