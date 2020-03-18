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

#[macro_use]
extern crate log;
#[macro_use]
extern crate wascc_codec as codec;

use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::{CapabilityConfiguration, OP_CONFIGURE, OP_REMOVE_ACTOR};
use codec::messaging::{PublishMessage, OP_PUBLISH_MESSAGE};
use codec::deserialize;
use env_logger;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use std::error::Error;

mod sqs;

const CAPABILITY_ID: &str = "wascc:messaging";

capability_provider!(AmazonSqsMessagingProvider, AmazonSqsMessagingProvider::new);

// Represents a waSCC Amazon SQS messaging provider.
pub struct AmazonSqsMessagingProvider {
    dispatcher: Arc<RwLock<Box<dyn Dispatcher>>>,
    clients: Arc<RwLock<HashMap<String, sqs::Client>>>,
}

impl Default for AmazonSqsMessagingProvider {
    // Returns the default value for `AmazonSqsMessagingProvider`.
    fn default() -> Self {
        if env_logger::try_init().is_err() {
            info!("Logger already intialized");
        }

        AmazonSqsMessagingProvider {
            dispatcher: Arc::new(RwLock::new(Box::new(NullDispatcher::new()))),
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl AmazonSqsMessagingProvider {
    // Creates a new, empty `AmazonSqsMessagingProvider`.
    pub fn new() -> Self {
        Self::default()
    }

    // Starts the capability provider.
    fn start(&self, config: CapabilityConfiguration) -> Result<Vec<u8>, Box<dyn Error>> {
        let module_id = &config.module;

        info!("AmazonSqsMessagingProvider(wascc:messaging) start: {}", module_id);

        let client = sqs::Client::new();

        self.clients.write().unwrap().insert(module_id.clone(), client);

        Ok(vec![])
    }

    // Stops the capability provider.
    fn stop(&self, config: CapabilityConfiguration) -> Result<Vec<u8>, Box<dyn Error>> {
        let module_id = &config.module;

        info!("AmazonSqsMessagingProvider(wascc:messaging) stop: {}", module_id);

        self.clients.write().unwrap().remove(module_id);

        Ok(vec![])
    }

    // Publishes a message.
    // Is this the right place for "tokio::main"?
    #[tokio::main(basic_scheduler)]
    async fn publish_message(&self, actor: &str, msg: PublishMessage) -> Result<Vec<u8>, Box<dyn Error>> {
        let lock = self.clients.read().unwrap();
        let client = match lock.get(actor) {
            Some(c) => c,
            None => return Err(format!("Unknown actor: {}", actor).into())
        };

        client.publish(msg).await
    }
}

impl CapabilityProvider for AmazonSqsMessagingProvider {
    // Returns the capability ID in the formated `namespace:id`.
    fn capability_id(&self) -> &'static str {
        CAPABILITY_ID
    }

    // Called when the host runtime is ready and has configured a dispatcher.
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        info!("AmazonSqsMessagingProvider(wascc:messaging) configure_dispatch");

        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    // Called by the host runtime when an actor is requesting a command be executed.
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        info!("AmazonSqsMessagingProvider(wascc:messaging) handle_call `{}` from `{}`", op, actor);

        match op {
            OP_CONFIGURE if actor == "system" => self.start(deserialize(msg)?),
            OP_REMOVE_ACTOR if actor == "system" => self.stop(deserialize(msg)?),
            OP_PUBLISH_MESSAGE => self.publish_message(actor, deserialize(msg)?),
            _ => Err(format!("Unsupported operation: {}", op).into()),
        }
    }

    // Returns the human-readable, friendly name of this capability provider.
    fn name(&self) -> &'static str {
        "Amazon SQS messaging provider"
    }
}
