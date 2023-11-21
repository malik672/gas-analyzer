use crate::utils;
use regex::Regex;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use std::fs;

pub fn cache_state_variables(
    gas_inefficiencies: &mut Map<String, serde_json::Value>,
    mut _prev: usize,
) {
    let arr_str: Vec<&str>;
    let ast_json = fs::read_to_string("src/optimizor/ast.json").expect("Failed to read");
    let ast: Value = serde_json::from_str(&ast_json).expect("Failed to deserialize");
    let mut _name = "";
    utils::get_all_state_var::get_all_state_vars();
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
                                                    != Some("=")
                                                {
                                                    if let Some(left_hand_side) =
                                                        expression.get("leftHandSide")
                                                    {
                                                        _name = left_hand_side
                                                            .get("name")
                                                            .unwrap()
                                                            .as_str()
                                                            .unwrap_or("");
                                                        let maps = utils::get_all_state_var::get_all_state_vars();
                                                        let name_bool = maps.get(_name).unwrap_or(
                                                            &serde_json::to_value(false).unwrap(),
                                                        );

                                                        if let Some(right_hand_side) =
                                                            expression.get("rightHandSide")
                                                        {
                                                            if let Some(right_hand_side_name) =
                                                                right_hand_side.get("name")
                                                            {
                                                                if let Some(is_pure) =
                                                                    right_hand_side.get("isPure")
                                                                {
                                                                    if is_pure == false {
                                                                        if _name
                                                                            == right_hand_side_name
                                                                        {
                                                                            /////**CACHE VARIABLE */
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
