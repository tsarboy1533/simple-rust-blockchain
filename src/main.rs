mod block;
mod blockchain;

use block::Transaction; // Transactionを使えるようにする
use blockchain::Blockchain;

fn main() {
    let mut my_blockchain = Blockchain::new();

    println!("Adding block 1...");
    // 1. トランザクションのリストを作る
    let txs1 = vec![
        Transaction {
            sender: String::from("Yuki"),
            receiver: String::from("Sato"),
            amount: 50.0,
        },
        Transaction {
            sender: String::from("Sato"),
            receiver: String::from("Tanaka"),
            amount: 10.0,
        },
    ];
    // 2. 文字列ではなく Vec<Transaction> を渡す
    my_blockchain.add_block(txs1);

    println!("Adding block 2...");
    let txs2 = vec![
        Transaction {
            sender: String::from("Tanaka"),
            receiver: String::from("Yuki"),
            amount: 25.0,
        },
    ];
    my_blockchain.add_block(txs2);

    // チェーンの内容を表示
    for block in my_blockchain.chain {
        println!("{:#?}", block); // {:#?} にすると綺麗に改行されて表示されます
    }
}