use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>, // ここを String から変更
    pub nonce: u64,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,    // 送信者（アドレスや名前）
    pub receiver: String,  // 受信者
    pub amount: f64,       // 金額
}

impl Block {
    // 1. 引数を transactions: Vec<Transaction> に変更
    pub fn new(id: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            id,
            timestamp,
            previous_hash,
            transactions, // ここにそのまま入れる
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }
    
    pub fn calculate_hash(&self) -> String {
        let tx_data = format!("{:?}", self.transactions); 
        let data = format!("{}{}{}{}{}", self.id, self.timestamp, self.previous_hash, tx_data, self.nonce);
        
        // 省略されていたハッシュ計算の実体を書き戻します
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        // 最初のハッシュが条件を満たしていない可能性があるので計算
        self.hash = self.calculate_hash();

        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Mined! Hash: {}", self.hash);
    }
}