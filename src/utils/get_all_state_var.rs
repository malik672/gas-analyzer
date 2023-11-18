use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use std::fs;

pub fn get_all_state_vars() -> Vec<str> {
    let arr_str: Vec<str>;
    let ast_json = fs::read_to_string("src/optimizor/ast.json").expect("Failed to read");
    let ast: Value = serde_json::from_str(&ast_json).expect("Failed to deserialize");
    let mut _name = "";
    if let Some(nodes) = ast.get("nodes").and_then(Value::as_array) {
        for node in nodes {
            if let Some(node_type) = node.get("nodeType").and_then(Value::as_str) {
                if node_type == "ContractDefinition" {
                    if let Some(contract_nodes) = node.get("nodes").and_then(Value::as_array) {
                        for contract_node in contract_nodes {
                            if let Some(state_node) = contract_node.as_object() {
                                if state_node.get("nodeType").and_then(Value::as_str)
                                    == Some("VariableDeclaration")
                                {
                                    if state_node.get("constant").and_then(Value::as_str)
                                        == Some("false")
                                    {
                                        if state_node.get("stateVariable").and_then(Value::as_str)
                                            == Some("true")
                                        {
                                            arr_str.push(state_node.get("name").unwrap());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    arr_str
}
