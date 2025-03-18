use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

use crate::entities::{
    block::{Block, Color, Shape},
    order::Order,
    requests::{BlockList, Pack},
};

#[derive(Debug, Clone, Default)]
pub struct Service {
    inventory: Arc<RwLock<Inventory>>,
    orders: Arc<RwLock<Vec<Order>>>,
    strategy: Arc<RwLock<FulfillmentStrategy>>,
    packs: Arc<RwLock<Vec<Pack>>>,
}

impl Service {
    pub fn get_blocks(&self) -> BlockList {
        self.inventory.read().unwrap().block_list()
    }

    pub fn add_blocks(&self, blocks: Vec<Block>) {
        for block in blocks {
            *self
                .inventory
                .write()
                .unwrap()
                .0
                .entry((block.shape, block.color))
                .or_default() += block.count;
        }

        self.try_fulfill();
    }

    pub fn get_strategy(&self) -> FulfillmentStrategy {
        self.strategy.read().unwrap().to_owned()
    }

    pub fn set_strategy(&self, strategy: FulfillmentStrategy) {
        *self.strategy.write().unwrap() = strategy;
    }

    pub fn get_orders(&self) -> Vec<Order> {
        self.orders.read().unwrap().to_owned()
    }

    pub fn submit_order(&self, blocks: Vec<Block>) {
        let mut orders = self.orders.write().unwrap();
        let number = orders.len() as u32 + 1;
        orders.push(Order::new(number, blocks));
        drop(orders);
        self.try_fulfill();
    }

    pub fn add_pack(&self, pack: Pack) {
        self.packs.write().unwrap().push(pack);
    }

    fn try_fulfill(&self) {
        match *self.strategy.read().unwrap() {
            FulfillmentStrategy::FirstOrder => {
                let mut orders = self.orders.write().unwrap();
                let mut inventory = self.inventory.write().unwrap();
                for order in orders.iter_mut() {
                    let fulfillable = order.blocks().iter().all(|&block| {
                        inventory
                            .0
                            .get(&(block.shape, block.color))
                            .is_some_and(|&count| count >= block.count)
                    });
                    if fulfillable {
                        for &block in order.blocks() {
                            *inventory.0.get_mut(&(block.shape, block.color)).unwrap() -=
                                block.count;
                        }
                        order.mark_fulfilled();
                    }
                }
            }
            FulfillmentStrategy::MaxOrder => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Inventory(HashMap<(Shape, Color), u32>);

impl Inventory {
    pub fn block_list(&self) -> BlockList {
        let blocks = self
            .0
            .iter()
            .map(|(&(shape, color), &count)| Block {
                shape,
                color,
                count,
            })
            .collect();
        BlockList { blocks }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FulfillmentStrategy {
    #[default]
    FirstOrder,
    MaxOrder,
}
