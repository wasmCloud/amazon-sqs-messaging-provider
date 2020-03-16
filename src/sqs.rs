// Copyright 2015-2019 Capital One Services, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//
// Amazon SQS Messaging Provider
//

use rusoto_core::Region;
use rusoto_sqs::{SqsClient};

// Represents an Amazon SQS client.
pub struct Client {
    sqs_client: SqsClient,
    queue_url: String,
}

impl Client {
    // Creates a new `Client`.
    pub fn new(queue_url: &str) -> Self {
        Client {
            sqs_client: SqsClient::new(Region::default()),
            queue_url: queue_url.into(),
        }
    }
}
