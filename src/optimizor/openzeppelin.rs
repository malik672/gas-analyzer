use regex::Regex;
use serde_json::json;
use serde_json::Map;

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
