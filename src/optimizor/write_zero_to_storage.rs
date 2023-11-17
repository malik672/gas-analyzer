use regex::Regex;
use serde_json::Value;
use std::fs;

pub fn write_zero_to_storage() {
    let ast_json = fs::read_to_string("src/optimizor/ast.json").expect("Failed to read");
    let ast: Value = serde_json::from_str(&ast_json).expect("Failed to deserialize");
    let mut _name = "";
    let mut _prev: usize = 0;
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
                                                                println!(
                                                                        "Avoid zero to one storage writes where possible. Line: {:?}",
                                                                        get_line_number_zero(_name, _prev)
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

fn get_line_number_zero(src: &str, mut _prev: usize) -> String {
    // Read the source file as a string
    let contract = fs::read_to_string("src/contract.sol").expect("Failed to read");

    // Split the contract content into lines
    let lines: Vec<&str> = contract.lines().collect();

    // Format the string with " = 0" at the end
    let strss = format!("{} = 0", src);

    // Compile the regex pattern
    let variable_declaration_regex = Regex::new(&strss).unwrap();

    let mut _line_numbers = 0;
    for (mut line_number, line) in lines.iter().enumerate() {
        line_number = _prev;
        _line_numbers = line_number + 1;
        if let Some(_capture) = variable_declaration_regex.captures(line) {}
        _prev = _line_numbers;
    }
    _prev.to_string()
}
