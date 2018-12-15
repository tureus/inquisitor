#![feature(uniform_paths,slice_concat_ext)]
#![recursion_limit="128"]
use reqwest::r#async::*;
use futures::prelude::*;
#[macro_use] extern crate serde_derive;

use std::mem;

mod data_builder;
use data_builder::JSONDataBuilder;

use std::slice::SliceConcatExt;

fn request() -> impl Future<Item=(), Error=()> {
    let data = data_builder::MetricDataBuilder::new(30*1024*1024);
//    println!("data: {}", unsafe { String::from_utf8_unchecked(data.get_blob()) });
    let json : Vec<data_builder::MetricData> = data.json();
    let lines: Vec<String> = json
        .iter()
        .map(|x| {
            serde_json::to_string(x).unwrap()
        })
        .collect();

    let mut with_cmds : Vec<String> = Vec::with_capacity(lines.len() * 2);

    for line in lines {
        with_cmds.push(r#"{ "index" : { "_index" : "xavier-bomb", "_type" : "_doc" } }"#.to_owned());
        with_cmds.push(line);
    }

    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build().unwrap();

    client
        .post("https://nginx-ingress-nlb.nginx-ingress/_bulk")
//        .post("https://metrics-dev.interactivedatastore.viasat.io/xavier-bomb/_bulk")
        .body(with_cmds.join("\n") + "\n")
        .basic_auth("elastic", Some("Dumb1234"))
        .header("HOST", "metrics-dev.interactivedatastore.viasat.io")
        .header("Content-Type","application/x-ndjson")
        .send()
        .and_then(|mut res| {
            let body = mem::replace(res.body_mut(), Decoder::empty());
            body.concat2()
        })
        .map_err(|err| println!("bad: {:#?}", err))
        .map(|body| println!("good {:#?}", body))
}

fn a_bunch_of_requests() -> impl Future<Item=(), Error=()> {
        request().join(request()).join(request()).join(request()).join(request()).join(request()).join(request()).join(request()).join(request()).join(request()).join(request()).map(|_| ())
}

fn main() {
    for _ in 1..100 {
        tokio::run(a_bunch_of_requests());
    }
}
