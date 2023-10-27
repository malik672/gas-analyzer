use regex::Regex;

pub fn bytes32(contract: &str) {
  let variable_declaration_regex = Regex::new(r"(bytes32 (public|private|internal) (constant|immutable))\s*\b([a-zA-Z_]\w*)\b\s*=\s*(.*)") .unwrap();

    for capture in variable_declaration_regex.captures_iter(contract) {
        let modifier = capture.get(1).map_or("default", |m| m.as_str());
        let variable_name = capture.get(4).unwrap().as_str();
        println!(
            "Use Uint256 instead of bytes32 to store constant: Modifier: {}, variable_name: {}",
            modifier, variable_name
        );
    }
}

pub fn openzepplin(contract: &str) {
    let variable_declaration_regex = Regex::new(r"openzeppelin") .unwrap();

    for capture in variable_declaration_regex.captures_iter(contract) {
        let modifier = capture.get(0).map_or("default", |m| m.as_str());
        println!(
            "instead of using openzeppelin we can use solady which is way cheaper and way efficient [https://github.com/Vectorized/solady: {}",
            modifier
        );
    }
}

pub fn safemath(contract: &str) {
    let variable_declaration_regex = Regex::new(r"SafeMath") .unwrap();

    for capture in variable_declaration_regex.captures_iter(contract) {
        let modifier = capture.get(0).map_or("default", |m| m.as_str());
        println!(
            "SafeMath is no longer needed since solidity version 0.8.0, use of safeMath can be considered unnessary: {}",
            modifier
        );
    }
}

pub fn token(contract: &str) {
    let variable_declaration_regex = Regex::new(r"(string (public|private|))\s*\b([a-zA-Z_]\w*)\b\s*=\s*(.*)") .unwrap();

    for capture in variable_declaration_regex.captures_iter(contract) {
        let modifier = capture.get(0).map_or("default", |m| m.as_str());
        println!(
            "For string of length less than 33, its better to use uint256 to store them: {}",
            modifier
        );
    }
}

pub fn uint_incur_overhead(contract: &str) {
    let variable_declaration_regex = Regex::new(r#"\((uint24|uint8|uint160|uint16)\s*(public|private|internal)\s*[^}]*bool"#) .unwrap();

    for capture in variable_declaration_regex.captures_iter(contract) {
        let modifier = capture.get(0).map_or("default", |m| m.as_str());
        println!(
            "instead of a uint24, uint16 or any uint and int type apart from uint256 or int256, it's way better to use uint256 or int256: {}",
            modifier
        );
    }
}