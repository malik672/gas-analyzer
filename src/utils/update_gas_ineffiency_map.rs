use serde_json::json;
use serde_json::Map;

pub fn update_gas_inefficency_map(
    inefficiency_id: String,
    gas_inefficiencies: &mut Map<String, serde_json::Value>,
    err: String,
) {
    // Check if the slot exists in the map
    if let Some(existing_value) = gas_inefficiencies.get_mut(&inefficiency_id) {
        // Slot exists, append the new issue to the existing array
        let mut existing_arr: Vec<String> =
            serde_json::from_value(existing_value.clone()).unwrap_or_default();

        existing_arr.push(err);

        // Update the value in the map
        gas_inefficiencies.insert(inefficiency_id, json!(existing_arr));
    } else {
        // Slot doesn't exist, create a new entry with a new array
        let new_arr = vec![err];
        gas_inefficiencies.insert(inefficiency_id, json!(new_arr));
    }
}
