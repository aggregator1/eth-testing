use std::env;
use std::str::FromStr;

use ethers::providers::StreamExt;
use hex_literal::hex;

use web3::contract::{Contract, Options};
use web3::futures::future;
use web3::types::{Address, BlockNumber, FilterBuilder, H160, U256, U64};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_SEPOLIA").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(web3::types::H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
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

    let filter = FilterBuilder::default()
        .from_block(BlockNumber::Number(U64::from(16603117)))
        .to_block(BlockNumber::Number(U64::from(16604000)))
        //Use emn178.github,io/online-tools/keccak_256.html to compute the hash of
        //token_supply(uint256) which is the ABI event corresponding to supply adjustments
        .topics(
            Some(vec![hex!(
                "b933665c2e5359d93d51999f2f8a4b3fef96a8500903bf562a96dc54a5d74b18"
            )
            .into()]),
            None,
            None,
            None,
        )
        .build();

    let subscription = web3s.eth_subscribe().subscribe_logs(filter).await?;

    subscription
        .for_each(|log| {
            println!("{:?}", log);
            future::ready(())
        })
        .await;

    Ok(())
}
