use std::env;
use std::str::FromStr;

use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_SEPOLIA").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);

    let pool_address = Address::from_str("0xD51a44d3FaE010294C616388b506AcdA1bfAAE46").unwrap();
    let pool_contract =
        Contract::from_json(web3s.eth(), pool_address, include_bytes!("erc20_abi.json")).unwrap();

    //Getting the supply of USDT in the pool
    let argument: U256 = U256::exp10(0);
    let supply: U256 = pool_contract
        .query("last_prices", argument, None, Options::default(), None)
        .await
        .unwrap();
    println!("Supply: {:?}", supply);

    Ok(())
}
