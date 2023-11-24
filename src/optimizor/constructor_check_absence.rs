use regex::Regex;
use serde_json::json;
use serde_json::Map;

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
