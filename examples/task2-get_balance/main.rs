use ethers::prelude::*;
use ethers::utils::format_units;
use std::error::Error;
use std::str::FromStr;

const RPC_URL: &str = "https://arb-sepolia.g.alchemy.com/v2/5rAhwxvzPaAMyn2_aCsvu";
const ADDRESS: &str = "0x5554956ca200e34C38B24E15B794c7f173e7Dfe1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = Address::from_str(ADDRESS).unwrap();
    let provider: Provider<Http> = Provider::<Http>::try_from(RPC_URL)?;
    let balance = provider.get_balance(address, None).await?;
    let balance_eth = format_units(balance, "ether")?;

    println!("Get balance: {balance:?} / {balance_eth}(eth)");

    Ok(())
}
