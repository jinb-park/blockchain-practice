use web3::types::{Address, TransactionRequest, FilterBuilder, Log};
use web3::ethabi::ethereum_types::U256;
use web3::contract::{Contract, Options};
use web3::api::{Eth, EthFilter, BaseFilter};
use web3::futures::StreamExt;
use web3::transports::Http;
use secp256k1::SecretKey;
use std::str::FromStr;
use std::time;
use hex_literal::hex;
use std::pin::Pin;

// [TODO] 2022.05.20: withdraw() doesn't work, needs to be fixed.

async fn print_balance(prefix: &str, eth: Eth<Http>, address: Address) {
    let balance = eth.balance(address, None).await;
    println!("{} : {}", prefix, balance.unwrap());
}

macro_rules! create_log_filter {
    ($addr:ident, $topic:literal, $eth:ident) => {
        $eth.eth_filter().create_logs_filter(
            FilterBuilder::default()
                .address(vec![$addr])
                .topics(
                    Some(vec![hex!($topic).into()]),
                    None,
                    None,
                    None,
                )
                .build()
            ).await?
    };
}

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    //let transport = web3::transports::Http::new("https://ropsten.infura.io/v3/62e14d23b9c34086bb58866d032e1137").unwrap();
    let transport = web3::transports::Http::new("http://localhost:8545").unwrap();  // [TODO] eth ropsten testnet
    let web3_obj = web3::Web3::new(transport);

    //let src_addr = Address::from_str("0xC403776B7e8df99e8BB98e5cA175cCd4d7f02b65").unwrap();
    let accounts = web3_obj.eth().accounts().await?;

    let src_addr = accounts[0];  // for local testing
    let bytecode = include_str!("../../../examples/out/Faucet.bin").trim_end();

    // Get current balance
    println!("=========== Deploy contract ==============");
    let mut src_balance = web3_obj.eth().balance(src_addr, None).await?;
    println!("src_balance: {}", src_balance);

    // Deploy
    let contract = Contract::deploy(web3_obj.eth(), include_bytes!("../../../examples/out/Faucet.abi"))?
        .confirmations(1)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(bytecode, (), src_addr)
        .await?;

    let contract_address = contract.address();
    println!("contract_addr: {}", contract_address);

    // Send 0.2 ETH to the contract
    println!("=========== Send 0.2 ETH to the contract ==============");

    // Event log stream (event Deposit):  see 'eventSelector' in .ast file
    let filter = create_log_filter!(contract_address, "e1fffcc4923d04b559f4d29a8bfc6cda04eb5b0d3c460751c2402c5c5cc9109c", web3_obj);
    let logs_stream = filter.stream(time::Duration::from_secs(1));
    futures::pin_mut!(logs_stream);

    let tx_object = TransactionRequest {
        from: src_addr,
        to: Some(contract_address),
        value: Some(U256::exp10(17) + U256::exp10(17)), //0.2 eth
        ..Default::default()
    };
    let result = web3_obj.eth().send_transaction(tx_object).await?;
    println!("transaction_result: {}", result); // transaction id

    // Waiting for transactions to be stored on the chain..
    // [TODO] How to do this an async way?
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Print current balance
    print_balance("contract_balance", web3_obj.eth(), contract_address).await;
    print_balance("src_balance", web3_obj.eth(), src_addr).await;

    // Get event log
    let log = logs_stream.next().await.unwrap();
    println!("event Deposit log: {:?}", log);

    // Withdraw 0.1 ETH from the contract
    println!("=========== Withdraw 0.1 ETH from the contract ==============");

    // Event log stream (event withdrawl)
    let filter = create_log_filter!(contract_address, "5afbed92d4941f0d4f191d133da98098254352f0196e7b3834378b4c66480928", web3_obj);
    let logs_stream = filter.stream(time::Duration::from_secs(1));
    futures::pin_mut!(logs_stream);

    let tx_withdraw = contract.call("transfer", (U256::exp10(17),), src_addr, Options::default()).await?;
    println!("TxHash: {}", tx_withdraw);

    // Waiting for transactions to be stored on the chain..
    // [TODO] How to do this an async way?
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Print current balance
    print_balance("contract_balance", web3_obj.eth(), contract_address).await;
    print_balance("src_balance", web3_obj.eth(), src_addr).await;

    let log = logs_stream.next().await.unwrap();
    println!("event withdrawl log: {:?}", log);

    Ok(())
}