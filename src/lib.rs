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
use codec::{deserialize, serialize};
use env_logger;
use std::collections::HashMap;
use std::env;

use std::error::Error;

const CAPABILITY_ID: &str = "wascc:messaging";

capability_provider!(AmazonSqsMessagingProvider, AmazonSqsMessagingProvider::new);

// Represents a waSCC Amazon SQS messaging provider.
pub struct AmazonSqsMessagingProvider {}

impl Default for AmazonSqsMessagingProvider {
    // Returns the default value for `AmazonSqsMessagingProvider`.
    fn default() -> Self {
        if env_logger::try_init().is_err() {
            info!("Logger already intialized");
        }

        AmazonSqsMessagingProvider {}
    }
}

impl AmazonSqsMessagingProvider {
    // Creates a new, empty `AmazonSqsMessagingProvider`.
    pub fn new() -> Self {
        Self::default()
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

        Ok(())
    }

    // Called by the host runtime when an actor is requesting a command be executed.
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        info!("AmazonSqsMessagingProvider(wascc:messaging) handle_call `{}` from `{}`", op, actor);

        match op {
            _ => return Err(format!("Unsupported operation: {}", op).into()),
        }

        Ok(vec![])
    }

    // Returns the human-readable, friendly name of this capability provider.
    fn name(&self) -> &'static str {
        "Amazon SQS messaging provider"
    }
}
