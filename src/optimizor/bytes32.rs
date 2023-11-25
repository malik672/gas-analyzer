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
