#![feature(uniform_paths,slice_concat_ext)]
use reqwest::r#async::*;
use futures::prelude::*;
#[macro_use] extern crate serde_derive;

use std::mem;

mod data_builder;
use data_builder::JSONDataBuilder;

use std::slice::SliceConcatExt;

fn request() -> impl Future<Item=(), Error=()> {
    let data = data_builder::MetricDataBuilder::new(1*1024*1024);
//    println!("data: {}", unsafe { String::from_utf8_unchecked(data.get_blob()) });
    let json : Vec<data_builder::MetricData> = data.json();
    let lines: Vec<String> = json
        .iter()
        .map(|x| {
            serde_json::to_string(x).unwrap()
        })
        .collect();
    let body = lines.join("\n");

    Client::new()
        .post("https://metrics-dev.interactivedatastore.viasat.io")
        .danger_accept_invalid_certs(true)
        .body(body)
        .basic_auth("elastic", Some("Dumb1234"))
        .send()
        .and_then(|mut res| {
            let body = mem::replace(res.body_mut(), Decoder::empty());
            body.concat2()
        })
        .map_err(|err| println!("bad: {:#?}", err))
        .map(|body| println!("good {:#?}", body))
}

fn a_bunch_of_requests() -> impl Future<Item=(), Error=()> {
    let a = request();
    let b = request();
    let c = request();
    a.join(b).join(c).map(|_| ())
}

fn main() {
    tokio::run(a_bunch_of_requests());
}
