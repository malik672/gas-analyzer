use regex::Regex;
use serde_json::json;
use serde_json::Map;

pub fn bytes32(contract: &str, gas_inefficiencies: &mut Map<String, serde_json::Value>) {
    let variable_declaration_regex = Regex::new(
        r"(bytes32 (public|private|internal) (constant|immutable))\s*\b([a-zA-Z_]\w*)\b\s*=\s*(.*)",
    )
    .unwrap();

    // Split the contract content into lines
    let lines: Vec<&str> = contract.lines().collect();

    for (line_number, line) in lines.iter().enumerate() {
        if let Some(capture) = variable_declaration_regex.captures(line) {
            let modifier = capture.get(1).map_or("default", |m| m.as_str());
            let variable_name = capture.get(4).unwrap().as_str();
            let inefficiency_id = format!("line_{}", line_number + 1); // Add 1 to adjust for 0-based indexing

            // Check if the slot exists in the map
            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push("Use Uint256 instead of bytes32 to store constant".to_string());

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr = vec!["Use Uint256 instead of bytes32 to store constant"];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }

            println!(
                "Use Uint256 instead of bytes32 to store constant: Modifier: {}, variable_name: {}, Line: {}",
                modifier,
                variable_name,
                line_number + 1 // Add 1 to adjust for 0-based indexing
            );
        }
    }
}

pub fn openzepplin(contract: &str, gas_inefficiencies: &mut Map<String, serde_json::Value>) {
    let variable_declaration_regex = Regex::new(r"openzeppelin").unwrap();

    // Split the contract content into lines
    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if let Some(capture) = variable_declaration_regex.captures(line) {
            let inefficiency_id = format!("line_{}", line_number + 1);

            // Check if the slot exists in the map
            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push("instead of using openzeppelin we can use solady which is way cheaper and way efficient [https://github.com/Vectorized/solady".to_string());

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr = vec!["instead of using openzeppelin we can use solady which is way cheaper and way efficient [https://github.com/Vectorized/solady"];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }

            let modifier = capture.get(0).map_or("default", |m| m.as_str());

            println!(
            "instead of using openzeppelin we can use solady which is way cheaper and way efficient [https://github.com/Vectorized/solady: {}",
             modifier
            );
        }
    }
}

pub fn safemath(contract: &str, gas_inefficiencies: &mut Map<String, serde_json::Value>) {
    let variable_declaration_regex = Regex::new(r"SafeMath").unwrap();

    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if let Some(capture) = variable_declaration_regex.captures(line) {
            let inefficiency_id = format!("line_{}", line_number + 1);
            // Check if the slot exists in the map
            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push("SafeMath is no longer needed since solidity version 0.8.0, use of safeMath can be considered unnessary".to_string());

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr = vec!["SafeMath is no longer needed since solidity version 0.8.0, use of safeMath can be considered unnessary"];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }

            let modifier = capture.get(0).map_or("default", |m| m.as_str());
            println!(
            "SafeMath is no longer needed since solidity version 0.8.0, use of safeMath can be considered unnessary: {}",
            modifier
        );
        }
    }
}

pub fn token(contract: &str, gas_inefficiencies: &mut Map<String, serde_json::Value>) {
    let variable_declaration_regex =
        Regex::new(r"(string (public|private|))\s*\b([a-zA-Z_]\w*)\b\s*=\s*(.*)").unwrap();

    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if variable_declaration_regex.captures(line).is_some() {
            let inefficiency_id = format!("line_{}", line_number + 1);
            // Check if the slot exists in the map
            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push(
                    "For string of length less than 33, its better to use uint256 to store them"
                        .to_string(),
                );

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr = vec![
                    "For string of length less than 33, its better to use uint256 to store them",
                ];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }
            println!("For string of length less than 33, its better to use uint256 to store them");
        }
    }
}

pub fn uint_incur_overhead(
    contract: &str,
    gas_inefficiencies: &mut Map<String, serde_json::Value>,
) {
    let variable_declaration_regex =
        Regex::new(r#"\((uint24|uint8|uint160|uint16)\s*(public|private|internal)\s*[^}]*bool"#)
            .unwrap();

    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if variable_declaration_regex.captures(line).is_some() {
            let inefficiency_id = format!("line_{}", line_number + 1);

            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push(
                    "instead of a uint24, uint16 or any uint and int type apart from uint256 or int256, it's way better to use uint256 or int256"
                        .to_string(),
                );

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr = vec![
                    "instead of a uint24, uint16 or any uint and int type apart from uint256 or int256, it's way better to use uint256 or int256",
                ];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }

            println!(
            "instead of a uint24, uint16 or any uint and int type apart from uint256 or int256, it's way better to use uint256 or int256:"
        );
        }
    }
}

pub fn check_constructor_absence(contract: &str) {
    let constructor_regex = Regex::new(r"constructor\s*\(.*\)").unwrap();

    if constructor_regex.is_match(contract) {
        let constructor_regex_payable = Regex::new(r"constructor\s*\(.*\)").unwrap();

        if constructor_regex_payable.is_match(contract) {
            println!("making the constructor payable saves gas in deployment cost");
        }
    } else {
        println!(
            "Adding constructor to the code and making it payable saves gas in deployment cost"
        );
    }
}

pub fn mapping_instead_array(
    contract: &str,
    gas_inefficiencies: &mut Map<String, serde_json::Value>,
) {
    let regexe = Regex::new(
        r"(\w+)\s*\[\]\s+(public|external|internal|private)?\s+(\w+)\s*;
    ",
    )
    .unwrap();
    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if regexe.captures(line).is_some() {
            let inefficiency_id = format!("line_{}", line_number + 1);

            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push(
                    "Use mapping instead of array: Modifier:"
                        .to_string(),
                );

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr = vec![
                    "Use mapping instead of array: Modifier:",
                ];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }

            println!("Use mapping instead of array: Modifier:");
        }
    }
}

pub fn uint256_instead_bool(
    contract: &str,
    gas_inefficiencies: &mut Map<String, serde_json::Value>,
) {
    let regexe = Regex::new(
        r"bool\s*\(.*\);
    ",
    )
    .unwrap();
    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if regexe.captures(line).is_some() {
            let inefficiency_id = format!("line_{}", line_number + 1);

            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push(
                    "Use uint256 type to store boolean value instead of bool"
                        .to_string(),
                );

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr = vec![
                    "Use uint256 type to store boolean value instead of bool",
                ];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }

            println!("Use uint256 type to store boolean value instead of bool");
        }
    }
}

pub fn use_named_retunrs(contract: &str, gas_inefficiencies: &mut Map<String, serde_json::Value>) {
    let regexe = Regex::new(
        r"returns\s+\(([^)]+)\) ;
    ",
    )
    .unwrap();
    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if regexe.captures(line).is_some() {
            let inefficiency_id = format!("line_{}", line_number + 1);

            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push(
                    "Use named returns".to_string(),
                );

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr =
                    vec!["Use named returns"];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }
            println!("Use named returns");
        }
    }
}

pub fn require_double_logic(
    contract: &str,
    gas_inefficiencies: &mut Map<String, serde_json::Value>,
) {
    let regexe = Regex::new(r"require\(.*&&.*\);").unwrap();
    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if regexe.captures(line).is_some() {
            let inefficiency_id = format!("line_{}", line_number + 1);

            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push(
                    "split require statements that use && into two seperate parts to save gas "
                        .to_string(),
                );

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr = vec![
                    "split require statements that use && into two seperate parts to save gas ",
                ];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }
            println!("split require statements ");
        }
    }
}

pub fn revert_32(contract: &str, gas_inefficiencies: &mut Map<String, serde_json::Value>) {
    let regexe = Regex::new(r"revert\(.*\'.{33,}\'.*\);").unwrap();
    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if regexe.captures(line).is_some() {
            let inefficiency_id = format!("line_{}", line_number + 1);

            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push(
                    "revert statement that has it's string longer than 32 length is always more expensive ".to_string(),
                );

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr =
                    vec!["revert statement that has it's string longer than 32 length is always more expensive "];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }
            println!("string length more thaan 32 ");
        }
    }
}

pub fn do_while(contract: &str, gas_inefficiencies: &mut Map<String, serde_json::Value>) {
    let regexe = Regex::new(r"\bfor\s*\(").unwrap();
    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if regexe.captures(line).is_some() {
            let inefficiency_id = format!("line_{}", line_number + 1);

            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr
                    .push("do while loops are cheaper than loops and consume less gas".to_string());

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr = vec!["do while loops are cheaper than loops and consume less gas"];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }
            println!("use do while loops instead of for loops ");
        }
    }
}

pub fn priv_constants_immut(
    contract: &str,
    gas_inefficiencies: &mut Map<String, serde_json::Value>,
) {
    let regexe = Regex::new(r"\bpublic\b.*(constant | immutable)").unwrap();
    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if regexe.captures(line).is_some() {
            let inefficiency_id = format!("line_{}", line_number + 1);
            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push(
                    "variables that are constant should have a visibility of private".to_string(),
                );

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr =
                    vec!["variables that are constant should have a visibility of private"];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }
            println!("variables that are constant should have a visibility of private");
        }
    }
}

pub fn emit_loops(contract: &str, gas_inefficiencies: &mut Map<String, serde_json::Value>) {
    let regexe = Regex::new(r"\bfor\s*\(\s*.*emit").unwrap();
    let lines: Vec<&str> = contract.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        if regexe.captures(line).is_some() {
            let inefficiency_id = format!("line_{}", line_number + 1);
            if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
                // Slot exists, append the new issue to the existing array
                let mut existing_arr: Vec<String> =
                    serde_json::from_value(existing_value.clone()).unwrap_or_default();

                existing_arr.push(
                    "emiting events in loops cost more, and should be done using other means"
                        .to_string(),
                );

                // Update the value in the map
                gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
            } else {
                // Slot doesn't exist, create a new entry with a new array
                let new_arr =
                    vec!["emiting events in loops cost more, and should be done using other means"];
                gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
            }
            println!("emiting events in loops cost more, and should be done using other means");
        }
    }
}
