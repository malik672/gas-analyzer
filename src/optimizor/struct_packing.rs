use crate::utils::{extract_struct_node, number_to_types, update_gas_ineffiency_map};
use regex::Regex;
use serde_json::Map;
use std::fs;

pub fn struct_packing(gas_inefficiencies: &mut Map<String, serde_json::Value>, mut _prev: usize) {
    let mut _name = "";
    let (old_arr, _name) = extract_struct_node::extract_struct_node();
    let mut old_arrs: Vec<f64> = Vec::new();
    let mut new_arr: Vec<f64> = Vec::new();
    let mut new_arr_str: Vec<String> = Vec::new();

    for str_to_f64 in old_arr {
        if let Ok(parsed_number) = str_to_f64.parse::<f64>() {
            new_arr.push(parsed_number);
            old_arrs.push(parsed_number);
            new_arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
    }

    //Update gas_inefficiencies map
    for str_new_arr in new_arr.clone() {
        new_arr_str.push(
            number_to_types::number_to_types()
                .get(&str_new_arr.to_string())
                .unwrap()
                .to_string().trim_matches('"').to_string(),
        );
    }

    if old_arrs != new_arr {
        println!("{:?}", true);
        _prev = get_line_number_zero(_name.as_str(), _prev);
        let mut _inefficiency_id = format!("line_{}", get_line_number_zero(_name.as_str(), _prev));

        get_line_number_zero(_name.as_str(), _prev);
        _inefficiency_id = format!("line_{}", _prev);

        update_gas_ineffiency_map::update_gas_inefficency_map(
            _inefficiency_id,
            gas_inefficiencies,
            format!(
                "struct is arraged wrongly should be arranged in this way {:?}",
                new_arr_str
            ),
        )
    }
}

fn get_line_number_zero(src: &str, mut _prev: usize) -> usize {
    // Read the source file as a string
    let contract = fs::read_to_string("src/contract.sol").expect("Failed to read");

    // Split the contract content into lines
    let lines: Vec<&str> = contract.lines().collect();

    // Format the string with " = 0" at the end
    let strss = format!(r"struct {}", src);

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
