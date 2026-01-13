use alloy::{
    primitives::{Address, address},
    providers::ProviderBuilder,
    sol,
};
use eyre::Result;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IERC20,
    "examples/task5-call_contract/abi/IERC20.json"
);

const RPC_URL: &str = "https://arbitrum-sepolia-testnet.api.pocket.network";
const CONTRACT_ADDRESS: Address = address!("0x047CD90411B7E601ed30a9A1af91b08c11298eD8");

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = RPC_URL.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // Create a contract instance.
    let contract = IERC20::new(CONTRACT_ADDRESS, provider);

    let name = contract.name().call().await?;
    let symbol = contract.symbol().call().await?;
    let decimals = contract.decimals().call().await?;
    let total_supply = contract.totalSupply().call().await?;
    println!(
        "Contract name: {name}, symbol: {symbol}, decimals: {decimals}, total supply: {total_supply}, address: {CONTRACT_ADDRESS}"
    );

    Ok(())
}
