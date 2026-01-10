use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, Bytes, U256, address},
    providers::{Provider, ProviderBuilder, WalletProvider},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};

use eyre::{Ok, Result};

const RPC_URL: &str = "https://arb-sepolia.g.alchemy.com/v2/5rAhwxvzPaAMyn2_aCsvu";
const FROM: Address = address!("0x5554956ca200e34C38B24E15B794c7f173e7Dfe1");
const TO: Address = address!("0xf4D0398aA609CA7fc914409c8D737e72156dAD34");

#[tokio::main]
async fn main() -> Result<()> {
    let private_key = std::env::var("WALLET_HELLO_PRIVATE_KEY_ETHEREUM")
        .expect("WALLET_HELLO_PRIVATE_KEY_ETHEREUM must be set");
    let signer: PrivateKeySigner = private_key.parse().expect("should parse private key");
    let wallet = EthereumWallet::from(signer);

    let rpc_url = RPC_URL.parse()?;
    let provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);

    // The `from` field is automatically filled to the first signer's address (Alice).
    let tx = tx_with(&provider, TransactionRequest::default().with_to(TO)).await?;
    println!("Transaction = {tx:?}");

    // Build and sign the transaction using the `EthereumWallet` with the provided wallet.
    let tx_envelope = tx.build(&provider.wallet()).await?;

    // Send the raw transaction and retrieve the transaction receipt.
    // [Provider::send_tx_envelope] is a convenience method that encodes the transaction using
    // EIP-2718 encoding and broadcasts it to the network using [Provider::send_raw_transaction].
    let receipt = provider
        .send_tx_envelope(tx_envelope)
        .await?
        .get_receipt()
        .await?;

    println!("Sent transaction: {}", receipt.transaction_hash);

    assert_eq!(receipt.from, FROM);
    assert_eq!(receipt.to, Some(TO));

    Ok(())
}

async fn tx_with(provider: &impl Provider, tx: TransactionRequest) -> Result<TransactionRequest> {
    // Estimate EIP-1559 fees.
    let fees = provider.estimate_eip1559_fees().await?;

    // Estimate gas limit for the transaction.
    let gas_limit = provider.estimate_gas(tx.clone()).await?;
    // Add a buffer to the estimates for safety.
    let safe_gas_limit = (gas_limit * 120) / 100;

    let nonce = provider.get_transaction_count(FROM).await?;

    let note = "Transfer 100 Gwei for testing";
    let input_data = Bytes::from(note.as_bytes().to_vec());

    Ok(tx
        .with_chain_id(provider.get_chain_id().await?)
        .with_value(U256::from(100))
        .with_gas_limit(safe_gas_limit)
        .with_max_fee_per_gas(fees.max_fee_per_gas)
        .with_max_priority_fee_per_gas(fees.max_priority_fee_per_gas)
        .with_nonce(nonce)
        .with_input(input_data))
}
