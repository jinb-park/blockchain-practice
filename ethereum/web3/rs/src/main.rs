extern crate web3;

use web3::futures::Future;

fn main() {
    let (_eloop, transport) = web3::transports::Http::new("https://ropsten.infura.io/v3/62e14d23b9c34086bb58866d032e1137").unwrap();
    let web3_obj = web3::Web3::new(transport);
    let src = Address::from_str("0xXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap();

    let count = web3_obj.eth().transaction_count().wait().unwrap();
    println!("{}", count);
}