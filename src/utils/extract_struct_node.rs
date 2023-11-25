use crate::utils::{types_to_numbers};
use serde_json::{Value};
use std::fs; 

pub fn extract_struct_node() -> (Vec<String>, String) {
    let ast_json = fs::read_to_string("src/optimizor/ast.json").expect("Failed to read");
    let ast: Value = serde_json::from_str(&ast_json).expect("Failed to deserialize");
    let mut arr = Vec::new();
    let mut name = "";
    if let Some(nodes) = ast.get("nodes").and_then(Value::as_array) {
        for node in nodes {
           
            if let Some(node_type) = node.get("nodeType").and_then(Value::as_str) {
                if node_type == "ContractDefinition" {
                   
                    if let Some(contract_nodes) = node.get("nodes").and_then(Value::as_array) {
                        for contract_node in contract_nodes {
                            
                            if let Some(func_node) = contract_node.as_object() {
                                if func_node.get("nodeType").and_then(Value::as_str)
                                    == Some("StructDefinition")
                                {   
                                    name = func_node.get("name").unwrap().as_str().unwrap();
                                    if let Some(members) =
                                        func_node.get("members").and_then(Value::as_array)
                                    {
                                        for member in members {
                                            if let Some(type_name) = member.get("typeName") {
                                                if let Some(name) =
                                                    type_name.get("name").and_then(Value::as_str)
                                                {
                                                    let t_to_n = types_to_numbers::_types_to_number();
                                                    if let Some(val) =
                                                        t_to_n.get(name).and_then(|v| v.as_str())
                                                    {
                                                        arr.push(val.to_string());
                                                    // Clone and push the value
                                                    } else {
                                                        // Handle the case when the value is not found or not a string
                                                        println!("Warning: Value not found or not a string for name '{}'", name);
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
            }
        }
    }
    (arr, name.to_owned())
}

