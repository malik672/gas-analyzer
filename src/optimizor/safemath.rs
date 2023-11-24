use regex::Regex;
use serde_json::json;
use serde_json::Map;


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
