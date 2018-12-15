#![feature(uniform_paths,slice_concat_ext)]
#![recursion_limit="128"]
use futures::prelude::*;
#[macro_use] extern crate serde_derive;
use docopt::Docopt;
use unbytify::unbytify;

use std::{ rc::Rc };

extern crate inquisitor;
use inquisitor::data_builder::{*};
use inquisitor::client::*;

const USAGE: &'static str = "
Elasticsearch's inquisitor

Usage:
  inquisitor <url> <payload-bytes> [--host=<host>]

Options:
  -h --help     Show this screen.
  --version     Show version.
  --host=HOST   Override the HTTP `Host`.
";


fn request(client: ESClient, body: Rc<Vec<u8>>) -> impl Future<Item=(), Error=()> {
    client.bulk(body)
}

fn a_bunch_of_requests(client: ESClient, body: Rc<Vec<u8>>) -> impl Future<Item=(), Error=()> {
    request(client, body).map(|_| ())
}

#[derive(Deserialize)]
struct Args {
    arg_url: String,
    arg_payload_bytes: String,
    flag_host: Option<String>
}

fn main() {
    let arg: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let payload_bytes = unbytify(&arg.arg_payload_bytes[..]).unwrap();
    let data = inquisitor::data_builder::MetricDataBuilder::new(payload_bytes as usize);
    let compressed = data.jsonnld_compressed();
    let rc_comp = Rc::new(compressed);

    let es_client = ESClient::new(arg.arg_url, arg.flag_host );

    for _ in 1..2 {
        tokio::run(a_bunch_of_requests(es_client.clone(), rc_comp.clone()));
    }
}
