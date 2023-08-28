use namada::tendermint::block::Height;
use namada::tendermint_rpc::{Client, HttpClient};
use namada::tendermint::block::Block;
use std::fs;
use serde_json::json;

fn main() {
    let data = fs::read_to_string("./block.json").unwrap();
    let block: Block = serde_json::from_str(&data).unwrap();


    println!("{}", serde_json::to_string(&block).unwrap());
}