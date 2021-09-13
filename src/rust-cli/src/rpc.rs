use bitcoincore_rpc::{Client, RpcApi};

pub fn exec_rpc_command(client: &Client, cmd: &str) {
    match cmd {
        "getbestblockhash" => println!("{:?}", client.get_best_block_hash().unwrap()),
        "getblockcount" => println!("{:?}", client.get_block_count().unwrap()),
        _ => {
            eprintln!("Error: Unsupported command {}", cmd);
            std::process::exit(1);
        }
    }
}
