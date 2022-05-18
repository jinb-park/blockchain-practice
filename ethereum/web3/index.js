let Web3 = require('web3')
let web3 = new Web3(new Web3.providers.HttpProvider('https://ropsten.infura.io/v3/62e14d23b9c34086bb58866d032e1137'));

web3.eth.getTransactionCount("0xC403776B7e8df99e8BB98e5cA175cCd4d7f02b65").then(console.log)