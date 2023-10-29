use regex::Regex;
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

            gas_inefficiencies.insert(
                inefficiency_id,
                "Use Uint256 instead of bytes32 to store constant".into(),
            );
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

            let modifier = capture.get(0).map_or("default", |m| m.as_str());

            gas_inefficiencies.insert(inefficiency_id, "instead of using openzeppelin we can use solady which is way cheaper and way efficient [https://github.com/Vectorized/solady".into());
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
            let modifier = capture.get(0).map_or("default", |m| m.as_str());
            println!(
            "SafeMath is no longer needed since solidity version 0.8.0, use of safeMath can be considered unnessary: {}",
            modifier
        );
            gas_inefficiencies.insert(inefficiency_id, "SafeMath is no longer needed since solidity version 0.8.0, use of safeMath can be considered unnessary".into());
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
            println!("For string of length less than 33, its better to use uint256 to store them");

            gas_inefficiencies.insert(
                inefficiency_id,
                "For string of length less than 33, its better to use uint256 to store them".into(),
            );
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
            println!(
            "instead of a uint24, uint16 or any uint and int type apart from uint256 or int256, it's way better to use uint256 or int256:"
        );
            gas_inefficiencies.insert(
            inefficiency_id,
            "instead of a uint24, uint16 or any uint and int type apart from uint256 or int256, it's way better to use uint256 or int256".into(),
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
            println!("Use mapping instead of array: Modifier:");

            gas_inefficiencies.insert(
            inefficiency_id,
            "instead of a uint24, uint16 or any uint and int type apart from uint256 or int256, it's way better to use uint256 or int256".into(),
        );
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
            println!("Use uint256 type to store boolean value instead of bool");

            gas_inefficiencies.insert(
            inefficiency_id,
            "instead of a uint24, uint16 or any uint and int type apart from uint256 or int256, it's way better to use uint256 or int256".into(),
        );
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
            println!("Use named returns");

            gas_inefficiencies.insert(
            inefficiency_id,
            "instead of a uint24, uint16 or any uint and int type apart from uint256 or int256, it's way better to use uint256 or int256".into(),
        );
        }
    }
}
