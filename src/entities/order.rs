use serde::{Deserialize, Serialize};

use super::block::Block;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    number: u32,
    state: OrderState,
    blocks: Vec<Block>,
}

impl Order {
    pub fn new(number: u32, blocks: Vec<Block>) -> Self {
        Self {
            number,
            state: OrderState::Pending,
            blocks,
        }
    }

    pub fn mark_fulfilled(&mut self) {
        self.state = OrderState::Fulfilled;
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderState {
    Pending,
    Fulfilled,
}
