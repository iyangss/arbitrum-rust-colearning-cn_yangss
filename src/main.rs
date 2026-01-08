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
    let rpc_url = "https://ethereum-sepolia.rpc.subquery.network/public".parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    // // Get the latest block number.
    // let latest_block = provider.get_block_number().await?;
    // println!("Latest block number: {latest_block}");


    // Instantiate the contract instance.
    let weth = address!("0x33e42f1f6ec71b27f5b31b750ebf39c733828827");
    let contract = HelloWeb3::new(weth, provider); 
 
    let out = contract.hello_web3().call().await?; 
    println!("Hello web3 out: {out}");

    Ok(())
}
