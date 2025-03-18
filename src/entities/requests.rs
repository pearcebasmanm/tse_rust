use serde::{Deserialize, Serialize};

use crate::services::service::FulfillmentStrategy;

use super::block::Block;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
pub struct Config {
    pub algorithm: FulfillmentStrategy,
}

#[derive(Serialize, Deserialize)]
pub struct BlockList {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Pack {
    pub name: String,
    pub price: f64,
    pub blocks: Vec<Block>,
}
