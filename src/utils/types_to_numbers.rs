use crate::utils::data_types::SolDataTypes;
use serde_json::{json, Map, Value};


pub fn _types_to_number() -> Map<String, Value> {
    let mut map: Map<String, Value> = Map::new();

    // Adding an entry for all data_types to map
    map.insert(
        SolDataTypes::Bool.to_string_representation().to_string(),
        json!("1"),
    );
    map.insert(
        SolDataTypes::Bytes4.to_string_representation().to_string(),
        json!("1.1"),
    );
    map.insert(
        SolDataTypes::Uint8.to_string_representation().to_string(),
        json!("1.2"),
    );
    map.insert(
        SolDataTypes::Int8.to_string_representation().to_string(),
        json!("1.3"),
    );
    map.insert(
        SolDataTypes::Int16.to_string_representation().to_string(),
        json!("2"),
    );
    map.insert(
        SolDataTypes::Uint16.to_string_representation().to_string(),
        json!("2.1"),
    );
    map.insert(
        SolDataTypes::Address.to_string_representation().to_string(),
        json!("3"),
    );
    map.insert(
        SolDataTypes::Uint160.to_string_representation().to_string(),
        json!("3.1"),
    );
    map.insert(
        SolDataTypes::Bytes20.to_string_representation().to_string(),
        json!("3.2"),
    );
    map.insert(
        SolDataTypes::Uint.to_string_representation().to_string(),
        json!("4"),
    );
    map.insert(
        SolDataTypes::Uint256.to_string_representation().to_string(),
        json!("4.1"),
    );
    map.insert(
        SolDataTypes::Bytes32.to_string_representation().to_string(),
        json!("4.2"),
    );
    map.insert(
        SolDataTypes::Int.to_string_representation().to_string(),
        json!("4.3"),
    );
    map.insert(
        SolDataTypes::Int256.to_string_representation().to_string(),
        json!("4.4"),
    );
    map.insert(
        SolDataTypes::Strings.to_string_representation().to_string(),
        json!("5"),
    );
    map.insert(
        SolDataTypes::Bytes.to_string_representation().to_string(),
        json!("5.1"),
    );
    map
}
