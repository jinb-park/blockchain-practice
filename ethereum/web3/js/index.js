let Web3 = require('web3')
let EtheriumTx = require('ethereumjs-tx').Transaction

// web3 1.0.0
let web3 = new Web3(new Web3.providers.HttpProvider('https://ropsten.infura.io/v3/62e14d23b9c34086bb58866d032e1137'));

let srcAddr = "0xC403776B7e8df99e8BB98e5cA175cCd4d7f02b65";  // MetaMask account
let dstAddr = "0xe353548b33fB88b66B1Dd8E56AA7F996E9a4e25b";  // test contract
let privKey = "0"

// 1. getTransactionCount()
let nonce = web3.eth.getTransactionCount(srcAddr, 'pending')

// 2. send 0.1 ETH
//let egas = web3.eth.estimateGas({
//    to: dstAddr
//}).then(web3.utils.toHex)

let rawTx = {
    nonce: web3.utils.toHex(nonce) + 0x1,
    from: srcAddr,
    to: dstAddr,
    gasPrice: web3.utils.toHex('21040'),
    gasLimit: web3.utils.toHex('100000'),
    value: web3.utils.toHex(web3.utils.toWei("0.1", "ether"))
}

const tx = new EtheriumTx(rawTx, {'chain':'ropsten'});
tx.sign(Buffer.from(privKey, 'hex'));

let serializedTx = '0x' + tx.serialize().toString('hex');
web3.eth.sendSignedTransaction(serializedTx).on('receipt', console.log);
