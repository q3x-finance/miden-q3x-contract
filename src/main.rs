use masm_project_template::common::{create_no_auth_faucet, instantiate_client};
use miden_client::{
    account::{AccountId, AccountStorageMode, Address, AddressInterface},
    address::{AddressId, NetworkId},
    asset::FungibleAsset,
    note::NoteType,
    rpc::Endpoint,
    transaction::TransactionRequestBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut client, _) = instantiate_client(Endpoint::testnet()).await?;

    client.sync_state().await?;

    let sync_height = client.get_sync_height().await?;
    println!("sync_height: {}", sync_height);

    // deploy fungible assets without the need of auth
    let account = create_no_auth_faucet(
        &mut client,
        "QWAP",
        1000000000000000000,
        8,
        AccountStorageMode::Public,
    )
    .await?;

    // let addr: AddressId = AddressId::from(account.id());

    let addr_id = AccountId::from_bech32("mtst1qzx905defy842yr7sgnh7pxpqpcqq86aypm")
        .unwrap()
        .1;

    // mint qash to
    let transaction_request = TransactionRequestBuilder::new()
        .build_mint_fungible_asset(
            FungibleAsset::new(account.id(), 100000000000000).unwrap(),
            addr_id,
            NoteType::Public,
            client.rng(),
        )
        .unwrap();

    client
        .submit_new_transaction(account.id(), transaction_request)
        .await?;
    println!("Minted 100 tokens for mtst1qzx905defy842yr7sgnh7pxpqpcqq86aypm.",);
    Ok(())
}
