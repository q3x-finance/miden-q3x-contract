use masm_project_template::common::{create_no_auth_faucet, instantiate_client};
use miden_client::{
    account::{AccountId, AccountStorageMode, Address, AddressInterface},
    address::{AddressId, NetworkId, RoutingParameters},
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
    let faucet = create_no_auth_faucet(
        &mut client,
        "QWAP",
        1000000000000000000,
        8,
        AccountStorageMode::Public,
    )
    .await?;

    let faucet_address_id = AddressId::from(faucet.id());
    let faucet_address = Address::new(faucet_address_id);
    let faucet_address = faucet_address
        .with_routing_parameters(RoutingParameters::new(AddressInterface::BasicWallet))
        .unwrap();

    // print out the faucet bech32 address
    println!(
        "Faucet bech32 address: {}",
        faucet_address.encode(NetworkId::Testnet)
    );

    let addr = Address::decode("mtst1apr3e492put8ayrsrz7wklh30sp0048t_qruqqypuyph")
        .unwrap()
        .1;
    let addr_id = addr.id();

    let account_id = match addr_id {
        AddressId::AccountId(account_id) => account_id,
        _ => panic!("Invalid address ID"),
    };

    // mint qash to
    let transaction_request = TransactionRequestBuilder::new()
        .build_mint_fungible_asset(
            FungibleAsset::new(faucet.id(), 100000000000000).unwrap(),
            account_id,
            NoteType::Public,
            client.rng(),
        )
        .unwrap();

    client
        .submit_new_transaction(faucet.id(), transaction_request)
        .await?;
    println!("Minted 100 tokens for mtst1apr3e492put8ayrsrz7wklh30sp0048t_qruqqypuyph.",);
    Ok(())
}
