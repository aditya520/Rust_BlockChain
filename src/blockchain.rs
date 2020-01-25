extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use serde_derive::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Serialize, Debug)]
pub struct Blockheader{
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block{
    header: Blockheader,
    count: u32,
    transaction: Vec<Transaction>,
}