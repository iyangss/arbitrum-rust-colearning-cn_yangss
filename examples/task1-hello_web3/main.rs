use alloy::{primitives::address, providers::ProviderBuilder, sol};
use std::error::Error;

// Generate the contract bindings for the ERC20 interface.
sol! {
   // The `rpc` attribute enables contract interaction via the provider.
   #[sol(rpc)]
   contract HelloWeb3 {
        function hello_web3() pure public returns(string memory);
   }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the HTTP transport which is consumed by the RPC client.
    // let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    let rpc_url = "https://arb-sepolia.g.alchemy.com/v2/5rAhwxvzPaAMyn2_aCsvu".parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // // Get the latest block number.
    // let latest_block = provider.get_block_number().await?;
    // println!("Latest block number: {latest_block}");

    // Instantiate the contract instance.
    let weth = address!("0x92591dd496E4233f690DD501FC1bc6bF71B401F6");
    let contract = HelloWeb3::new(weth, provider);

    let out = contract.hello_web3().call().await?;
    println!("Hello web3 out: {out}");

    Ok(())
}
