use regex::Regex;
use serde_json::json;
use serde_json::Map;

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
