#![cfg(test)]
use cosmwasm_std::CustomMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_storage_plus::Item;

pub mod contracts;
pub mod mocks;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyMsg {}

/// This is just a demo place so we can test custom message handling
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
#[serde(rename = "snake_case")]
pub enum CustomHelperMsg {
    SetName {
        name: String,
    },
    SetAge {
        age: u32,
    },
    #[default]
    NoOp,
}

impl CustomMsg for CustomHelperMsg {}

const COUNT: Item<u32> = Item::new("count");
