extern crate clap;
use bitcoincore_rpc::{Auth, Client};
use clap::App;
use clap::{crate_authors, crate_description, crate_version, load_yaml};

use defi_rust_cli::chainparamsbase::Chain;
use defi_rust_cli::rpc::exec_rpc_command;

const DEFAULT_RPCCONNECT: &str = "127.0.0.1";

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .get_matches();

    let rpcuser = matches.value_of("rpcuser").unwrap();
    let rpcpassword = matches.value_of("rpcpassword").unwrap();

    let params = match matches.value_of("chain") {
        Some("regtest") => Chain::Regtest.get_params(),
        Some("testnet") => Chain::Testnet.get_params(),
        Some("devnet") => Chain::Devnet.get_params(),
        _ => Chain::Main.get_params(),
    };

    let rpc = match Client::new(
        format!("http://{}:{}", DEFAULT_RPCCONNECT, params.rpc_port),
        Auth::UserPass(rpcuser.to_string(), rpcpassword.to_string()),
    ) {
        Ok(client) => client,
        Err(error) => panic!("Error creating RPC client : {:?}", error),
    };

    let cmd = matches.value_of("command").unwrap();
    exec_rpc_command(&rpc, cmd);
}
