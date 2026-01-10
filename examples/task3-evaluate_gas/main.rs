use alloy::{
    network::TransactionBuilder,
    primitives::{Address, address, utils::format_units},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use eyre::Result;

const RPC_URL: &str = "https://arb-sepolia.g.alchemy.com/v2/5rAhwxvzPaAMyn2_aCsvu";
const ADDRESS: Address = address!("0x5554956ca200e34C38B24E15B794c7f173e7Dfe1");

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = RPC_URL.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // Get the gas price of the network.
    let wei_per_gas = provider.get_gas_price().await?;

    let tx = TransactionRequest::default()
        .with_to(ADDRESS)
        .with_value(alloy::primitives::uint!(100_U256));
    let gas_limit = provider.estimate_gas(tx).await?; // estimate_gas

    // 手动增加 10% 的缓冲量
    let safe_gas_limit = (gas_limit * 110) / 100;
    let total_gas_fee_wei = wei_per_gas * safe_gas_limit as u128; // evaluate
    let total_gas_fee_eth = format_units(total_gas_fee_wei, "ether")?;
    println!(
        "Estimate_gas={gas_limit}, Gas fee: {} / {}(eth)",
        safe_gas_limit, total_gas_fee_eth
    );

    Ok(())
}
