#!/bin/sh

solc -o out/ --optimize --bin --abi --ast-compact-json --overwrite --asm Faucet2.sol
