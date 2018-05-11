extern crate sodiumoxide;
extern crate boringcoin_core;

use boringcoin_core::blockchain::Blockchain;
use boringcoin_core::block::Block;
use boringcoin_core::wallet::Wallet;
use boringcoin_core::transaction::{ Transaction, TransactionOutput };

fn main() {
    sodiumoxide::init();
    let mut boringchain = Blockchain::new();

    let wallet_a = Wallet::new(boringchain.clone());   
    let wallet_b = Wallet::new(boringchain.clone());
    let coinbase = Wallet::new(boringchain.clone());

    let mut genesis_transaction = Transaction::new(coinbase.public_key.clone(), wallet_a.public_key.clone(), 100_f32, Vec::new());
    genesis_transaction.generate_signature(&coinbase.private_key);
    genesis_transaction.id = String::from("0");
    genesis_transaction.outputs.push(TransactionOutput::new(genesis_transaction.receiver, genesis_transaction.value, genesis_transaction.id));
    boringchain.UTXOs.insert(genesis_transaction.outputs[0].id, genesis_transaction.outputs[0]);

    println!("Generating and Mining genesis block");

    let mut genesis_block = Block::new(String::from("0"));
    
    // TODO -> this is messy
    let mut prev_hash = genesis_block.hash.clone();
    let genesis_transaction_clone = genesis_transaction.clone();

    genesis_block.add_transaction(genesis_transaction, boringchain.clone(), &wallet_a.private_key);

    boringchain.add_block(genesis_block);

    let block1 = Block::new(prev_hash);
    println!("\nWalletA's balance is: {}", wallet_a.get_balance());
    println!("\nWalletA is Attempting to send funds (40) to WalletB...");
    block1.add_transaction(wallet_a.send_funds(wallet_b.public_key, 40_f32).unwrap(),boringchain.clone(), &wallet_b.private_key);
    prev_hash = block1.hash.clone();
    boringchain.add_block(block1);
    println!("\nWalletA's balance is: {}", wallet_a.get_balance());
    println!("WalletB's balance is: {}", wallet_b.get_balance());


    let block2 = Block::new(prev_hash);
    println!("\nWalletA Attempting to send more funds (1000) than it has...");
    println!("\nWalletA is Attempting to send funds (40) to WalletB...");
    block2.add_transaction(wallet_a.send_funds(wallet_b.public_key, 1_000_f32).unwrap(),boringchain.clone(), &wallet_b.private_key);
    prev_hash = block2.hash.clone();
    boringchain.add_block(block1);
    println!("\nWalletA's balance is: {}", wallet_a.get_balance());
    println!("WalletB's balance is: {}", wallet_b.get_balance());

    let block3 = Block::new(prev_hash);
    println!("\nWalletB is Attempting to send funds (20) to WalletA...");
    block3.add_transaction(wallet_b.send_funds(wallet_a.public_key, 20_f32).unwrap(),boringchain.clone(), &wallet_a.private_key);
    prev_hash = block3.hash.clone();
    println!("\nWalletA's balance is: {}", wallet_a.get_balance());
    println!("WalletB's balance is: {}", wallet_b.get_balance());

    if boringchain.is_valid(&genesis_transaction_clone) {
        println!("CHAIN BE VALID");
    } else {
        println!("WHO BROKE DA CHAIN");
    }
    
}   








