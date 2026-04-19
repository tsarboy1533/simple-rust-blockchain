use crate::block::{Block, Transaction}; // Transactionもインポートする

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        // ジェネシスブロックも Vec<Transaction> を受け取るように修正
        // 最初は空のリスト vec![] を渡すのが一般的です
        let genesis_block = Block::new(
            0, 
            String::from("0"), 
            vec![] // 空のトランザクションリスト
        );
        
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut new_block = Block::new(
            self.chain.len() as u64, 
            previous_hash, 
            transactions
        );
        
        new_block.mine(4); 
        self.chain.push(new_block);
    }
}