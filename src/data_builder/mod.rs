use serde::Serialize;
use serde_json;
use hashbrown::HashMap;
use flate2::{ bufread::GzEncoder, Compression};
use pretty_bytes::converter::convert as bytes_convert;

use std::io::Read;


// ES has its own "new line delimited" flavor of encoding
pub trait JSONNLDDataBuilder<T: Serialize + Sized> {
    fn new(target_size: usize) -> Self;
    fn lines(&self) -> Vec<T>;
    fn index_command(&self) -> String;
    fn jsonnld(&self) -> String;
    fn jsonnld_compressed(&self) -> Vec<u8>;
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
            ("nice.metric".to_owned(), rand::random()),
            ("how.about.pi".to_owned(), rand::random()),
            ("yes.please.i.like.nesting.hmm".to_owned(), rand::random()),
            ("yes.please.i.like.nesting.more".to_owned(), rand::random()),
            ("yes.please.i.like.nesting.more2".to_owned(), rand::random()),
            ("yes.please.i.like.nesting.more100".to_owned(), rand::random()),
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

impl JSONNLDDataBuilder<MetricData> for MetricDataBuilder where {
    fn new(target_size: usize) -> Self {
        Self{ target_size }
    }

    fn lines(&self) -> Vec<MetricData> {
        let sample_metric = MetricData::random();
        let sample_blob = serde_json::to_string(&sample_metric).unwrap();
        let sample_cmd = self.index_command();

        let single_entry_bytes = sample_blob.len() + sample_cmd.len();

        if single_entry_bytes > self.target_size {
            vec![sample_metric]
        } else {
            let num_samples_need = self.target_size / single_entry_bytes;
            let samples : Vec<_>= (0..num_samples_need).map(|_| MetricData::random()).collect();
            samples
        }
    }

    fn index_command(&self) -> String {
        r#"{ "index" : { "_index" : "xavier-bomb", "_type" : "_doc" } }"#.to_owned()
    }

    fn jsonnld(&self) -> String {
        let lines: Vec<String> = self.lines()
            .iter()
            .map(|x| {
                serde_json::to_string(x).unwrap()
            })
            .collect();

        let mut with_cmds : Vec<String> = Vec::with_capacity(lines.len() * 2);

        for line in lines {
            with_cmds.push(self.index_command());
            with_cmds.push(line);
        }

        let full_body = with_cmds.join("\n") + "\n";

        full_body
    }

    fn jsonnld_compressed(&self) -> Vec<u8> {
        let full_body = self.jsonnld();

        let full_body_len = full_body.len();
        let reader = std::io::BufReader::new(std::io::Cursor::new(full_body));
        let mut gz = GzEncoder::new(reader, Compression::best());
        let mut compressed : Vec<u8> = Vec::with_capacity(full_body_len);
        gz.read_to_end(&mut compressed).unwrap();

        let ratio : f64 = (compressed.len() as f64) / ( full_body_len as f64 );
        println!("uncompressed: {}, compressed: {}, ratio: {:.2}", bytes_convert(full_body_len as f64), bytes_convert(compressed.len() as f64), ratio);

        compressed
    }
}