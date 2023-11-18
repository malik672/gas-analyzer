use regex::Regex;
use serde_json::Map;
use serde_json::json;
use serde_json::Value;
use std::fs;


pub fn write_zero_to_storage(gas_inefficiencies: &mut Map<String, serde_json::Value>, mut _prev: usize) {
    let ast_json = fs::read_to_string("src/optimizor/ast.json").expect("Failed to read");
    let ast: Value = serde_json::from_str(&ast_json).expect("Failed to deserialize");
    let mut _name = "";
    if let Some(nodes) = ast.get("nodes").and_then(Value::as_array) {
        for node in nodes {
            if let Some(node_type) = node.get("nodeType").and_then(Value::as_str) {
                if node_type == "ContractDefinition" {
                    if let Some(contract_nodes) = node.get("nodes").and_then(Value::as_array) {
                        for contract_node in contract_nodes {
                            if let Some(func_node) = contract_node.as_object() {
                                if func_node.get("nodeType").and_then(Value::as_str)
                                    == Some("FunctionDefinition")
                                {
                                    if let Some(statements) = func_node
                                        .get("body")
                                        .and_then(|b| b.get("statements"))
                                        .and_then(Value::as_array)
                                    {
                                        for statement in statements {
                                            if let Some(expression) = statement.get("expression") {
                                                if expression
                                                    .get("operator")
                                                    .and_then(Value::as_str)
                                                    == Some("=")
                                                {
                                                    if let Some(left_hand_side) =
                                                        expression.get("leftHandSide")
                                                    {
                                                        _name = left_hand_side
                                                            .get("name")
                                                            .unwrap()
                                                            .as_str()
                                                            .unwrap_or("");
                                                    }
                                                    if let Some(right_hand_side) =
                                                        expression.get("rightHandSide")
                                                    {
                                                        if right_hand_side.get("value")
                                                            == Some(&Value::String("0".to_string()))
                                                        {
                                                            if !right_hand_side
                                                                .get("isConstant")
                                                                .unwrap_or(&Value::Bool(true))
                                                                .as_bool()
                                                                .unwrap_or(true)
                                                            {
                                                                _prev = get_line_number_zero(_name, _prev);
                                                                let mut inefficiency_id = format!("line_{}", get_line_number_zero(_name, _prev));
                                                               
                                                                get_line_number_zero(_name, _prev);
                                                                 inefficiency_id = format!("line_{}",  _prev);
                                                                // Check if the slot exists in the map
                                                                if let Some(existing_value) =
                                                                    gas_inefficiencies
                                                                        .get_mut(&inefficiency_id)
                                                                {
                                                                    // Slot exists, append the new issue to the existing array
                                                                    let mut existing_arr: Vec<
                                                                        String,
                                                                    > = serde_json::from_value(
                                                                        existing_value.clone(),
                                                                    )
                                                                    .unwrap_or_default();

                                                                    existing_arr.push("avoid writing zero to storage slot".to_string());

                                                                    // Update the value in the map
                                                                    gas_inefficiencies.insert(
                                                                        inefficiency_id,
                                                                        json!(existing_arr),
                                                                    );
                                                                } else {
                                                                    // Slot doesn't exist, create a new entry with a new array
                                                                    let new_arr = vec!["avoid writing zero to storage slot"];
                                                                    gas_inefficiencies.insert(
                                                                        inefficiency_id,
                                                                        json!(new_arr),
                                                                    );
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
