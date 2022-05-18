pragma solidity ^0.8.13;

contract Faucet {
    function withdraw(uint withdraw_amount) public {
        require(withdraw_amount <= 100000000000000000);

        payable(msg.sender).transfer(withdraw_amount);
    }

    // 1. where is the code handling a coming ETH?
    //fallback () external payable {}
}