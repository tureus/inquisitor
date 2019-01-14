/*
   Copyright 2019 Ashley Mannix

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
*/

#![allow(unused_variables, dead_code)]

/** A default type for allocated fields in responses. */
pub(crate) type DefaultAllocatedField = String;

/** Returned hits metadata. */
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Shards {
    total: u32,
    successful: u32,
    failed: u32,
}

impl Shards {
    /** The total number of shards that participated in this request. */
    pub fn total(&self) -> u32 {
        self.total
    }

    /** The total number of shards that successfully processed the request. */
    pub fn successful(&self) -> u32 {
        self.successful
    }

    /** The total number of shards that failed to process the request. */
    pub fn failed(&self) -> u32 {
        self.failed
    }
}

#[derive(Deserialize, Debug)]
pub(crate) enum DocumentResult {
    #[serde(rename = "deleted")] Deleted,
    #[serde(rename = "updated")] Updated,
    #[serde(rename = "not_found")] NotFound,
    #[serde(rename = "noop")] NoOp,
}