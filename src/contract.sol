// SPDX-License-Identifier: MIT
pragma solidity 0.8.12;

contract Counter1 {
    uint256 public number;
    string private ionize;
    bool private localize;

    function increment() public {
        require(number < 10);
        number = number + 1;
    }
}

