use regex::Regex;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use std::fs;

pub fn cache_state_variables(
    gas_inefficiencies: &mut Map<String, serde_json::Value>,
    mut _prev: usize,
) {
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
                                          arr_str.push()
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

fn get_line_number_zero(src: &str, mut _prev: usize) -> usize {
    // Read the source file as a string
    let contract = fs::read_to_string("src/contract.sol").expect("Failed to read");

    // Split the contract content into lines
    let lines: Vec<&str> = contract.lines().collect();

    // Format the string with " = 0" at the end
    let strss = format!(r"{} = 0", src);

    // Compile the regex pattern
    let variable_declaration_regex = Regex::new(&strss).unwrap();

    for (line_number, line) in lines.iter().enumerate() {
        if let Some(_capture) = variable_declaration_regex.captures(line) {
            if line_number > _prev {
                _prev = line_number + 1;
                break;
            }
        }
    }
    _prev
}
