pragma solidity ^0.8.13;

contract owned {
    address payable owner;

    constructor() {
        owner = payable(msg.sender);  // msg.sender refers to who creates this contract first.
    }

    modifier onlyOwner {
        require(msg.sender == owner, "Only the contract owner can call this");
        _;
    }
}

contract mortal is owned {
    function destroy() public onlyOwner {
        selfdestruct(owner);
    }
}

contract Faucet is mortal {
    event withdrawl(address indexed to, uint amount);
    event Deposit(address indexed from, uint amount);

    function transfer(uint withdraw_amount) public payable {
        require(withdraw_amount <= 0.1 ether);
        require(address(this).balance >= withdraw_amount, "Insufficient balance");

        owner.transfer(msg.value);
        emit withdrawl(owner, withdraw_amount);
    }

    receive() external payable {
        emit Deposit(owner, msg.value);
    }
}
