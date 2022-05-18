use web3::types::Address;
use std::str::FromStr;

#[tokio::main]
async fn main() -> web3::Result {
    let transport = web3::transports::Http::new("https://ropsten.infura.io/v3/62e14d23b9c34086bb58866d032e1137").unwrap();
    let web3_obj = web3::Web3::new(transport);
    let src_addr = Address::from_str("0xC403776B7e8df99e8BB98e5cA175cCd4d7f02b65").unwrap();

    let count = web3_obj.eth().transaction_count(src_addr, None).await?;
    println!("transaction_count: {}", count);

    Ok(())
}