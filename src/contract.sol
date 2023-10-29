/// SPDX-License-Identifier: BUSL-1.1

/// Copyright (C) 2023 Brahma.fi

pragma solidity 0.8.19;

/**
 * @title TypeHashHelper
 * @author Brahma.fi
 * @notice Helper library containing functions to build EIP712 struct and type hashes
 */
 import "@openzeppelin/contracts/utils/math/SafeCast.sol";
 import "@openzeppelin/contracts/utils/math/SafeMath.sol";
library TypeHashHelper {
    /**
     * @notice Structural representation of transaction details
     * @param operation type of operation
     * @param to address to send tx to
     * @param account address of safe
     * @param executor address of executor if executed via executor plugin, address(0) if executed via execTransaction
     * @param value txn value
     * @param nonce txn nonce
     * @param data txn callData
     */
    struct Transaction {
        uint8 operation;
        address to;
        address account;
        address executor;
        uint256 value;
        uint256 nonce;
        bytes data;
    }

    /**
     * @notice Type of validation struct to hash
     * @param expiryEpoch max time till validity of the signature
     * @param transactionStructHash txn digest generated using TypeHashHelper._buildTransactionStructHash()
     * @param policyHash policy commit hash of the safe account
     */
    struct Validation {
        uint32 expiryEpoch;
        bytes32 transactionStructHash;
        bytes32 policyHash;
    }

    /**
     * @notice EIP712 typehash for transaction data
     * @dev keccak256("ExecutionParams(address to,uint256 value,bytes data,uint8 operation,address account,address executor,uint256 nonce)");
     */
    bytes32 public constant TRANSACTION_PARAMS_TYPEHASH =
        0xfd4628b53a91b366f1977138e2eda53b93c8f5cc74bda8440f108d7da1e99290;

    /**
     * @notice EIP712 typehash for validation data
     * @dev keccak256("ValidationParams(ExecutionParams executionParams,bytes32 policyHash,uint32 expiryEpoch)ExecutionParams(address to,uint256 value,bytes data,uint8 operation,address account,address executor,uint256 nonce)")
     */
    bytes32 public constant VALIDATION_PARAMS_TYPEHASH =
        0x0c7f653e0f641e41fbb4ed1440c7d0b08b8d2a19e1c35cfc98de2d47519e15b1;

    /**
     * @notice Builds EIP712 transaction struct hash
     * @param txn transaction params struct
     * @return transactionStructHash
     */
    function _buildTransactionStructHash(Transaction memory txn) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                TRANSACTION_PARAMS_TYPEHASH,
                txn.to,
                txn.value,
                keccak256(txn.data),
                txn.operation,
                txn.account,
                txn.executor,
                txn.nonce
            )
        );
    }

    /**
     * @notice Builds EIP712 validation struct hash
     * @param validation validation params struct
     * @return validationStructHash
     */
    function _buildValidationStructHash(Validation memory validation) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                VALIDATION_PARAMS_TYPEHASH,
                validation.transactionStructHash,
                validation.policyHash,
                validation.expiryEpoch
            )
        );
    }
}