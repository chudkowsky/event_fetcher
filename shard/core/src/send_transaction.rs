use starknet_crypto::Felt;
use std::sync::Arc;
use url::Url;
use starknet::{
    accounts::{Account, SingleOwnerAccount},
    core::{codec::Encode, types::Call},
    macros::selector,
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
    signers::{LocalWallet, SigningKey},
};

#[derive(Debug, Encode)]
struct UpdateStateCalldata {
    snos_output: Vec<Felt>,
    shard_id: Felt,
}

pub async fn send_transaction(
    contract_address: Felt,
    rpc_url: Url,
    private_key: Felt,
    account_address: Felt,
    snos_output: Vec<Felt>,
) {
    let provider: Arc<JsonRpcClient<HttpTransport>> =
        Arc::new(JsonRpcClient::new(HttpTransport::new(rpc_url)));
    let chain_id = provider.chain_id().await.unwrap();
    let signer = LocalWallet::from_signing_key(SigningKey::from_secret_scalar(private_key));
    let account = SingleOwnerAccount::new(
        provider,
        signer,
        account_address,
        chain_id,
        starknet::accounts::ExecutionEncoding::New,
    );

    let selector = selector!("update_state");
    let call = Call {
        to: contract_address,
        selector,
        calldata: {
            let calldata = UpdateStateCalldata {
                snos_output,
                shard_id: Felt::ONE,
            };
            
            let mut raw_calldata = vec![]; 
            calldata.encode(&mut raw_calldata).unwrap();
            raw_calldata
        },
    };
    let tx = account
        .execute_v3(vec![call])
        .send()
        .await
        .unwrap()
        .transaction_hash;
    println!("{}", tx);
}
