extern crate sodiumoxide;
extern crate boringcoin_core;

use boringcoin_core::blockchain::Blockchain;
use boringcoin_core::block::Block;
use boringcoin_core::wallet::Wallet;
use boringcoin_core::transaction::{ Transaction, TransactionOutput };

fn main() {
    sodiumoxide::init();
    let mut boringchain = Blockchain::new(1, 2.5);

    let mut wallet_a = Wallet::new();   
    let mut wallet_b = Wallet::new();
    let mut coinbase = Wallet::new();

    let mut genesis_transaction = Transaction::new(coinbase.public_key.clone(), wallet_a.public_key.clone(), 100_f32, Vec::new());
    genesis_transaction.generate_signature(&coinbase.private_key);
    genesis_transaction.id = String::from("0");
    genesis_transaction.outputs.push(TransactionOutput::new(genesis_transaction.receiver.clone(), genesis_transaction.value.clone(), genesis_transaction.id.clone()));
    boringchain.UTXOs.insert(genesis_transaction.outputs[0].id.clone(), genesis_transaction.outputs[0].clone());
    println!("Generating and Mining genesis block");

    let mut genesis_block = Block::new(vec![0_u8]);
   
    println!("gened gen block");
    // TODO -> this is messy
    let mut prev_hash = genesis_block.hash.clone();
    let genesis_transaction_clone = genesis_transaction.clone();
    println!("making transaction");
    genesis_block.add_transaction(genesis_transaction, boringchain.clone(), &wallet_a.private_key);

    println!("added transaction");
    boringchain.add_block(genesis_block);
    println!("added block");
    let mut block1 = Block::new(prev_hash);
    println!("\nWalletA's balance is: {}", wallet_a.get_balance(&boringchain));
    println!("\nWalletA is Attempting to send funds (40) to WalletB...");
    block1.add_transaction(wallet_a.send_funds(wallet_b.public_key, 40_f32, &boringchain).unwrap(),boringchain.clone(), &wallet_b.private_key);
    prev_hash = block1.hash.clone();
    boringchain.add_block(block1);
    println!("\nWalletA's balance is: {}", wallet_a.get_balance(&boringchain));
    println!("WalletB's balance is: {}", wallet_b.get_balance(&boringchain));


    let mut block2 = Block::new(prev_hash);
    println!("\nWalletA Attempting to send more funds (1000) than it has...");
    let send_funds2 = wallet_a.send_funds(wallet_b.public_key, 1_000_f32, &boringchain);
    match send_funds2 {
        Ok(transaction) => { block2.add_transaction(transaction, boringchain.clone(), &wallet_b.private_key); () },
        Err(e) => println!("Err while sending funds: {}", e),
    }
    prev_hash = block2.hash.clone();
    boringchain.add_block(block2);
    println!("\nWalletA's balance is: {}", wallet_a.get_balance(&boringchain));
    println!("WalletB's balance is: {}", wallet_b.get_balance(&boringchain));

    let mut block3 = Block::new(prev_hash);
    println!("\nWalletB is Attempting to send funds (20) to WalletA...");
    let send_funds3 = wallet_b.send_funds(wallet_a.public_key, 20_f32, &boringchain);
    match send_funds3 {
        Ok(transaction) => { block3.add_transaction(transaction, boringchain.clone(), &wallet_a.private_key); () },
        Err(e) => println!("Err while sending funds: {}", e),
    }
    prev_hash = block3.hash.clone();
    println!("\nWalletA's balance is: {}", wallet_a.get_balance(&boringchain));
    println!("WalletB's balance is: {}", wallet_b.get_balance(&boringchain));

    if boringchain.is_valid(&genesis_transaction_clone) {
        println!("CHAIN BE VALID");
    } else {
        println!("WHO BROKE DA CHAIN");
    }
    
}   








