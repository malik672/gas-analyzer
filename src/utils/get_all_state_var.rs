use serde_json::Value;
use std::fs;
use serde_json;

pub fn get_all_state_vars() -> serde_json::Map<std::string::String, Value> {
    let ast_json = fs::read_to_string("src/optimizor/ast.json").expect("Failed to read");
    let ast: Value = serde_json::from_str(&ast_json).expect("Failed to deserialize");
    let mut obj = serde_json::Map::new();

    if let Some(nodes) = ast.get("nodes").and_then(Value::as_array) {
        for node in nodes {
            if let Some(node_type) = node.get("nodeType").and_then(Value::as_str) {
                if node_type == "ContractDefinition" {
                    if let Some(contract_nodes) = node.get("nodes").and_then(Value::as_array) {
                        for contract_node in contract_nodes {
                            if let Some(state_node) = contract_node.as_object() {
                                if state_node.get("nodeType").and_then(Value::as_str)
                                    == Some("VariableDeclaration")
                                    && state_node.get("constant").and_then(Value::as_bool)
                                        == Some(false)
                                    && state_node.get("stateVariable").and_then(Value::as_bool)
                                        == Some(true)
                                {
                                    obj.insert(state_node.get("name").unwrap().as_str().unwrap().to_owned(), 
                                    serde_json::to_value(true).unwrap());

                                }
                            }
                        }
                    }
                }
            }
        }
    }
    obj
}
