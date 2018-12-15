use std::{ mem, rc::Rc };

use reqwest::r#async::{ Client, ClientBuilder, Decoder };
use futures::prelude::*;

mod responses;
use crate::client::responses::bulk_response::*;

#[derive(Clone)]
pub struct ESClient {
    client: Client,
    base_url: String,
    host: Option<String>
}

impl ESClient {
    pub fn new(base_url: String, host: Option<String>) -> Self {
        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build().unwrap();

        ESClient {
            client,
            base_url,
            host
        }
    }

    pub fn bulk(&self, body: Rc<Vec<u8>>) -> impl Future<Item=(), Error=()> {
        let url = format!("{}/_bulk", self.base_url);

        let most_of_the_request = self.client
            // .post("https://nginx-ingress-nlb.nginx-ingress/_bulk")
            .post(&url[..])
            .body(body.as_ref().to_owned())
            .basic_auth("elastic", Some("Dumb1234"))
            .header("Content-Type","application/x-ndjson")
            .header("Content-Encoding", "gzip");

        let request = if let Some(ref host) = self.host {
            most_of_the_request.header("Host", host.to_owned())
        } else {
            most_of_the_request
        };

        request.send()
            .and_then(|mut res| {
                let body = mem::replace(res.body_mut(), Decoder::empty());
                body.concat2()
            })
            .map_err(|err| println!("no bueno: {:#?}", err))
            .map(|body| {
                let response : BulkErrorsResponse = serde_json::from_slice(&body[..]).unwrap();

                println!("took: {}, was successful: {}", response.took(), response.is_ok());
                if response.is_err(){
                    let errs : Vec<&ErrorItem<String,String,String>>= response.iter().collect();
                    println!("errs: {:#?}", errs);
                }
            })
    }
}