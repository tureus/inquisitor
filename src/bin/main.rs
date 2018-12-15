#![feature(uniform_paths,slice_concat_ext)]
#![recursion_limit="128"]
use futures::prelude::*;
use futures::future::join_all;
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
  inquisitor <url> <payload-bytes> [--host=<host>] [--concurrency=<concurrency>]

Options:
  -h --help                      Show this screen.
  --version                      Show version.
  --host=HOST                    Override the HTTP `Host`.
  --concurrency=CONCURRENCY      Spawn concurrent requests.
";


fn request(client: ESClient, body: Rc<Vec<u8>>) -> impl Future<Item=(), Error=()> {
    client.bulk(body)
}

fn a_bunch_of_requests(client: ESClient, body: Rc<Vec<u8>>, concurrency: Option<usize>) -> impl Future<Item=(), Error=()> {
    let mut builder = vec![request(client.clone(), body.clone())];

    if let Some(c) = concurrency {
        for _ in 0..c {
            builder.push(request(client.clone(), body.clone()));
        }
    }

    join_all(builder).map(|_| ())
}

#[derive(Deserialize)]
struct Args {
    arg_url: String,
    arg_payload_bytes: String,
    flag_host: Option<String>,
    flag_concurrency: Option<usize>,
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

    for _ in 1.. {
        tokio::run(a_bunch_of_requests(es_client.clone(), rc_comp.clone(), arg.flag_concurrency));
    }
}
