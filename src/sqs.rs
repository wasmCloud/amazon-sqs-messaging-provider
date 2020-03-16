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
use rusoto_sqs::{Sqs, SqsClient, SendMessageRequest};
use std::error::Error;
use wascc_codec::messaging::PublishMessage;

// Represents an Amazon SQS client.
pub(crate) struct Client {
    sqs_client: SqsClient,
}

impl Default for Client {
    // Returns the default value for `Client`.
    fn default() -> Self {
        Client {
            sqs_client: SqsClient::new(Region::default()),
        }
    }
}

impl Client {
    // Creates a new, default `Client`.
    pub fn new() -> Self {
        Self::default()
    }

    // Publishes a message.
    pub async fn publish(&self, msg: PublishMessage) -> Result<Vec<u8>, Box<dyn Error>> {
        info!("Publishing message ({} bytes) to {}", msg.message.body.len(), msg.message.subject);

        let req = SendMessageRequest{
            queue_url: msg.message.subject,
            message_body: String::from_utf8(msg.message.body)?,
            ..Default::default()
        };
        let resp = self.sqs_client.send_message(req).await?;
        if let Some(message_id) = resp.message_id {
            info!("Message sent with ID: {}", message_id);
        }

        Ok(vec![])
    }
}
