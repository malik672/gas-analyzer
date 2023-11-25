#[warn(dead_code)]

pub enum SolDataTypes {
    Strings,       // Variable-length byte array
    Uint8,         // Unsigned integer of 8 bits
    Uint16,        // Unsigned integer of 16 bits
    Uint160,       // Unsigned integer of 160 bits
    Uint256,       // Unsigned integer of 256 bits
    Uint,          // Unsigned integer of 256 bits (assuming this is a placeholder)
    Bytes4,        // Fixed-length byte array of 4 bytes
    Bytes20,       // Fixed-length byte array of 20 bytes
    Bytes32,       // Fixed-length byte array of 32 bytes
    Bytes,         // Dynamic-length byte array
    Bool,          // Boolean value, true or false
    Address,       // 160-bit value that can store an Ethereum address
    Int8,          // Signed integer of 8 bits
    Int16,         // Signed integer of 16 bits
    Int256,        // Signed integer of 256 bits
    Int,           // Signed integer of 256 bits (assuming this is a placeholder)
}

impl SolDataTypes {
    pub fn to_string_representation(&self) -> &'static str {
        match self {
            SolDataTypes::Strings => "string",
            SolDataTypes::Uint8 => "uint8",
            SolDataTypes::Uint16 => "uint16",
            SolDataTypes::Uint160 => "uint160",
            SolDataTypes::Uint256 => "uint256",
            SolDataTypes::Uint => "uint",
            SolDataTypes::Bytes4 => "bytes4",
            SolDataTypes::Bytes20 => "bytes20",
            SolDataTypes::Bytes32 => "bytes32",
            SolDataTypes::Bytes => "bytes",
            SolDataTypes::Bool => "bool",
            SolDataTypes::Address => "address",
            SolDataTypes::Int8 => "int8",
            SolDataTypes::Int16 => "int16",
            SolDataTypes::Int256 => "int256",
            SolDataTypes::Int => "int",
        }
    }
}

