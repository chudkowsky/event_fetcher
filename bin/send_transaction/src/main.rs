use std::fs;

use clap::Parser;
use shard_core::send_transaction;
use starknet_crypto::Felt;

#[derive(Parser)]
struct Cli {
    #[arg(env, short, long)]
    contract_address: Felt,
    #[arg(env, short, long)]
    rpc_url: String,
    #[arg(env, long)]
    account_address: Felt,
    #[arg(env, short, long)]
    private_key: Felt,
    #[arg(env, short, long)]
    args_file: String,
}

#[tokio::main]
pub async fn main() {
    let args = Cli::parse();
    let rpc_url = args.rpc_url.parse().unwrap();
    let raw_args = fs::read_to_string(args.args_file).unwrap();

    let calldata: Vec<Felt> = raw_args
        .split(',')
        .map(|s| Felt::from_dec_str(s.trim()).unwrap())
        .collect();
    println!("{:?}", calldata);
    send_transaction::send_transaction(
        args.contract_address,
        rpc_url,
        args.private_key,
        args.account_address,
        calldata,
    )
    .await;
}
