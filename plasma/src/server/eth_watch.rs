use std::sync::mpsc::Sender;
use crate::models::plasma_models::{Block};
use super::state_keeper::BlockProcessingRequest;

pub struct EthWatch {

}

impl EthWatch {

    pub fn new() -> Self {
        Self{}
    }

    pub fn run(&mut self, tx_for_blocks: Sender<BlockProcessingRequest>) {
        // TODO: watch chain events
        // on new deposit or exit blocks => pass them via tx_for_blocks
        // on new tx blocks do nothing for now; later we can use them to sync multiple 
        // servers (in which case we only use them to update current state)
    }

}