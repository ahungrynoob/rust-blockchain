use core::blockchain;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();

    println!("start mining ...");
    thread::sleep(Duration::from_secs(5));

    bc.add_block("a -> b: 5btc".to_string());
    println!("product a block");

    println!("start mining ...");
    thread::sleep(Duration::from_secs(5));

    bc.add_block("c -> d: 1btc".to_string());
    println!("product a block");

    for b in bc.blocks {
        println!("++++++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }
}
