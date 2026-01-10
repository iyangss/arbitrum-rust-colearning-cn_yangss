use alloy::{
    primitives::{Address, address},
    providers::ProviderBuilder,
    sol,
};
use std::error::Error;

// Generate the contract bindings for the ERC20 interface.
sol! {
   // The `rpc` attribute enables contract interaction via the provider.
   #[sol(rpc)]
   contract ERC20 {
        function name() public view virtual override returns (string memory);
        function symbol() public view virtual override returns (string memory);
        function decimals() public view virtual override returns (uint8);
        function totalSupply() public view virtual override returns (uint256);
   }
}

const RPC_URL: &str = "https://arb-sepolia.g.alchemy.com/v2/5rAhwxvzPaAMyn2_aCsvu";
const CONTRACT_ADDRESS: Address = address!("0x047CD90411B7E601ed30a9A1af91b08c11298eD8");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = RPC_URL.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    let contract = ERC20::new(CONTRACT_ADDRESS, provider);

    let name = contract.name().call().await?;
    let symbol = contract.symbol().call().await?;
    let decimals = contract.decimals().call().await?;
    let total_supply = contract.totalSupply().call().await?;
    println!(
        "Contract name: {name}, symbol: {symbol}, decimals: {decimals}, total supply: {total_supply}, address: {CONTRACT_ADDRESS}"
    );

    Ok(())
}
