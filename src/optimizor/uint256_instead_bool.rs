use regex::Regex;
use serde_json::json;
use serde_json::Map;

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