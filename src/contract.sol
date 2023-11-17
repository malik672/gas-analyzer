/// SPDX-License-Identifier: BUSL-1.1

/// Copyright (C) 2023 Brahma.fi

pragma solidity 0.8.12;

/**
 * @title TypeHashHelper
 * @author Brahma.fi
 * @notice Helper library containing functions to build EIP712 struct and type hashes
 */
 
contract TypeHashHelper {
   uint public reds;
   function red() public {
      uint x = 3 +2;
      reds = 0;
   }

   function destroy() public {
      reds = 0;
   }
}