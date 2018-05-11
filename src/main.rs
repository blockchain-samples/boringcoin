extern crate sodiumoxide;

use sodiumoxide;
use blockchain::Blockchain;
use block::Block;
use wallet::Wallet;
use transaction::{ Transaction, TransactionOutput };

fn main() {
    sodiumoxide::init();
    let mut boringchain = Blockchain::new();

    let wallet_a = Wallet::new(&boringchain);   
    let wallet_b = Wallet::new(&boringchain);
    let coinbase = Wallet::new(&boringchain);

    let mut genesis_transaction = Transaction::new(&coinbase.public_key, &wallet_a.public_key, 100f, Vec::new());
    genesis_transaction.generate_signature(&coinbase.private_key);
    genesis_transaction.id = String::from("0");
    genesis_transaction.outputs.push(TransactionOutput::new(genesis_transaction.receiver, genesis_transaction.value, genesis_transaction.id));
    boringchain.UTXOs.insert(genesis_transaction.outputs[0].id, genesis_transaction.outputs[0]);

    println!("Generating and Mining genesis block");

    let mut genesis_block = Block::new(String::from("0"));
    
    // TODO -> this is messy
    let mut prev_hash = genesis_block.hash.clone();
    
    genesis_block.add_transaction(genesis_transaction);

    boringchain.add_block(genesis_block);

    let block1 = Block::new(prev_hash);
    println!("\nWalletA's balance is: " + wallet_a.getBalance());
    println!("\nWalletA is Attempting to send funds (40) to WalletB...");
    block1.add_transaction(wallet_a.sendFunds(wallet_b.publicKey, 40_f32));
    prev_hash = block1.hash.clone();
    boringchain.add_block(block1);
    println!("\nWalletA's balance is: " + wallet_a.getBalance());
    println!("WalletB's balance is: " + wallet_b.getBalance());


    let block2 = Block::new(prev_hash);
    println!("\nWalletA Attempting to send more funds (1000) than it has...");
    println!("\nWalletA is Attempting to send funds (40) to WalletB...");
    block2.add_transaction(wallet_a.sendFunds(wallet_b.publicKey, 1_000_f32));
    prev_hash = block2.hash.clone();
    boringchain.add_block(block1);
    println!("\nWalletA's balance is: " + wallet_a.getBalance());
    println!("WalletB's balance is: " + wallet_b.getBalance());

    let block3 = Block::new(prev_hash);
    println!("\nWalletB is Attempting to send funds (20) to WalletA...");
    block3.add_transaction(wallet_a.sendFunds(wallet_b.publicKey, 20_f32));
    prev_hash = block3.hash.clone();
    println!("\nWalletA's balance is: " + wallet_a.getBalance());
    println!("WalletB's balance is: " + wallet_b.getBalance());

    if boringchain.is_valid() {
        println!("CHAIN BE VALID");
    } else {
        println!("WHO BROKE DA CHAIN");
    }
    
}   








