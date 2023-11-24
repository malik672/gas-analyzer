use crate::utils;
use regex::Regex;
// use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use std::fs;

pub fn cache_state_variables(
    gas_inefficiencies: &mut Map<String, serde_json::Value>,
    mut _prev: usize,
) {
    let ast_json = fs::read_to_string("src/optimizor/ast.json").expect("Failed to read");
    let ast: Value = serde_json::from_str(&ast_json).expect("Failed to deserialize");

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
                                                    if !expression
                                                        .get("leftHandSide")
                                                        .unwrap()
                                                        .get("name")
                                                        .is_none()
                                                    {
                                                        let name = expression
                                                            .get("leftHandSide")
                                                            .unwrap()
                                                            .get("name")
                                                            .and_then(Value::as_str)
                                                            .unwrap()
                                                            .to_owned();

                                                        let maps = utils::get_all_state_var::get_all_state_vars();
                                                        let _name_bool: Option<bool> = maps
                                                            .get(&name)
                                                            .and_then(Value::as_bool);

                                                        _prev = get_line_number_zero(&name, _prev);
                                                        let inefficiency_id =
                                                            format!("line_{}", _prev);

                                                        utils::update_gas_ineffiency_map::update_gas_inefficency_map(inefficiency_id, gas_inefficiencies, "cache state variables".to_owned());
                                                    } else if let Some(name) = expression
                                                        .get("rightHandSide")
                                                        .unwrap()
                                                        .get("name")
                                                        .and_then(Value::as_str)
                                                        .map(String::from)
                                                    {
                                                        let maps = utils::get_all_state_var::get_all_state_vars();
                                                        let _name_bool: Option<bool> = maps
                                                            .get(&name)
                                                            .and_then(Value::as_bool);

                                                        println!("{}", expression);

                                                        _prev = get_line_number_zero(&name, _prev);
                                                        let inefficiency_id =
                                                            format!("line_{}", _prev);

                                                        utils::update_gas_ineffiency_map::update_gas_inefficency_map(inefficiency_id, gas_inefficiencies,"cache state variables".to_owned());
                                                    }
                                                } else if expression
                                                    .get("operator")
                                                    .and_then(Value::as_str)
                                                    != Some("=")
                                                {
                                                    if let Some(arguments) = expression
                                                        .get("body")
                                                        .and_then(|b| b.get("arguments"))
                                                        .and_then(Value::as_array)
                                                    {
                                                        for argument in arguments {
                                                            if let Some(left_E) =
                                                                argument.get("leftExpression")
                                                            {
                                                                let name = argument
                                                                    .get("leftExpression")
                                                                    .unwrap()
                                                                    .get("name")
                                                                    .and_then(Value::as_str)
                                                                    .unwrap()
                                                                    .to_owned();

                                                                let maps = utils::get_all_state_var::get_all_state_vars();
                                                                let _name_bool: Option<bool> = maps
                                                                    .get(&name)
                                                                    .and_then(Value::as_bool);

                                                                _prev = get_line_number_zero(
                                                                    &name, _prev,
                                                                );
                                                                let inefficiency_id =
                                                                    format!("line_{}", _prev);

                                                                utils::update_gas_ineffiency_map::update_gas_inefficency_map(inefficiency_id, gas_inefficiencies, "cache state variables".to_owned());
                                                            } else if let Some(name) = argument
                                                                .get("rightExpression")
                                                                .unwrap()
                                                                .get("name")
                                                                .and_then(Value::as_str)
                                                                .map(String::from)
                                                            {
                                                                let maps = utils::get_all_state_var::get_all_state_vars();
                                                                let _name_bool: Option<bool> = maps
                                                                    .get(&name)
                                                                    .and_then(Value::as_bool);

                                                                _prev = get_line_number_zero(
                                                                    &name, _prev,
                                                                );
                                                                let inefficiency_id =
                                                                    format!("line_{}", _prev);

                                                                utils::update_gas_ineffiency_map::update_gas_inefficency_map(inefficiency_id, gas_inefficiencies, "cache state variables".to_owned());
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
    //get number of times
    let mut times = 0;
    println!("{}", "erere");
    // Read the source file as a string
    let contract = fs::read_to_string("src/contract.sol").expect("Failed to read");

    // Split the contract content into lines
    let lines: Vec<&str> = contract.lines().collect();

    // Format the string with " = 0" at the end
    let strss = format!(r"{}", src);

    // Compile the regex pattern for any function declaration
    let any_function_declaration_regex = Regex::new(r"function\s+\w*\s*\(").unwrap();

    // Compile the regex pattern
    let variable_declaration_regex = Regex::new(&strss).unwrap();

    for (line_number, line) in lines.iter().enumerate() {
        if let Some(_captures) = any_function_declaration_regex.captures(line) {
            if let Some(_capture) = variable_declaration_regex.captures(line) {
                if line_number > _prev && times > 1 {
                    _prev = line_number + 1;
                    break;
                }
                times = times + 1;
            }
        }
    }
    _prev
}
