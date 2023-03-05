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

    //Making a connection to the mainnet or some testnet using the address in INFURA_SEPOLIA(right
    //now the code connects to the mainnet!)
    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_SEPOLIA").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    //First code snippet: getting the balance of one or multiple accounts.
    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(web3::types::H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);

    //Second code snippet: using a contract's ABI.
    //IMPORTANT: first the ABI must be pasted into the erc20_abi.json file.
    let contract_address = Address::from_str("0xdAC17F958D2ee523a2206206994597C13D831ec7").unwrap();
    let contract = Contract::from_json(
        web3s.eth(),
        contract_address,
        include_bytes!("erc20_abi.json"),
    )
    .unwrap();

    //Getting all transfers of USDT. This code can also easily be adapted to register other events
    //from other contracts.

    //Here a function is called from the contract's ABI.
    //If "input" is empty in the ABI, "argument" should be set equal to an empty tuple
    let argument = ();
    let supply: U256 = contract
        .query("totalSupply", argument, None, Options::default(), None)
        .await
        .unwrap();
    println!("Supply: {:?}", supply);

    //Here we have an event listener. from_block has to be greater than the current block to listen
    //live.
    let filter = FilterBuilder::default()
        .from_block(BlockNumber::Number(U64::from(16707030)))
        .to_block(BlockNumber::Number(U64::from(16900000)))
        .address(vec![Address::from_str(
            "0xdAC17F958D2ee523a2206206994597C13D831ec7",
        )
        .unwrap()])
        //Use emn178.github,io/online-tools/keccak_256.html to compute the hash of
        //Transfer(address,address,uint256) which is the ABI event corresponding to token transfers.
        //To listen for another event, its name followed by the Solidity types of its input
        //parameters must be hashed using the link above. The hash is case sensitive.
        .topics(
            Some(vec![hex!(
                "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
            )
            .into()]),
            None,
            None,
            None,
        )
        .build();

    let subscription = web3s.eth_subscribe().subscribe_logs(filter).await?;

    //for_each requires as an argument a closure that returns a type implementing Future.
    //In this case, future::ready() returns type Ready which indeed implements Future.
    subscription
        .for_each(|log| {
            println!(
                "Block, sender, receiver: {:?}",
                //Prints the block, the sending address and the receiving address of each
                //transaction.
                (
                    log.as_ref().unwrap().block_number.unwrap(),
                    log.as_ref().unwrap().topics[1],
                    log.as_ref().unwrap().topics[2],
                )
            );
            future::ready(())
        })
        .await;

    Ok(())
}
