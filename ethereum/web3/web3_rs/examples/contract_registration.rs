use web3::types::Address;
use web3::contract::{Contract, Options};
use secp256k1::SecretKey;
use std::str::FromStr;
use std::time;

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    //let transport = web3::transports::Http::new("https://ropsten.infura.io/v3/62e14d23b9c34086bb58866d032e1137").unwrap();
    let transport = web3::transports::Http::new("http://localhost:8545").unwrap();  // [TODO] eth ropsten testnet
    let web3_obj = web3::Web3::new(transport);

    //let src_addr = Address::from_str("0xC403776B7e8df99e8BB98e5cA175cCd4d7f02b65").unwrap();
    let accounts = web3_obj.eth().accounts().await?;
    let src_addr = accounts[0];  // for local testing
    let bytecode = include_str!("../../../examples/out/Faucet.bin").trim_end();

    let contract = Contract::deploy(web3_obj.eth(), include_bytes!("../../../examples/out/Faucet.abi"))?
        .confirmations(1)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, (), src_addr)
        .await?;

    let contract_address = contract.address();
    println!("contract_addr: {}", contract_address);

    Ok(())
}