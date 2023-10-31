use reqwest;
use reqwest::Error;
use serde_json::{Map, Value};
use tokio;
use std::fs;

fn read_sol_file(file_path: &str) -> Result<String, std::io::Error> {
    // Read the contents of the Solidity file into a string
    let content = fs::read_to_string(file_path)?;

    Ok(content)
}

async fn read_from_github(path: &str) -> Result<String, Error>  {
    // Send an HTTP GET request to fetch the raw content
   
    let client = reqwest::Client::new();
    let response = client.get(path).send().await?;
    let contract_content = response.text().await?;
    println!("{}", contract_content);

     Ok(contract_content)

}

mod optimizor {
    pub mod gas_tricks;
}

#[tokio::main]
async fn main() {
    let contract: &str = &read_sol_file("./src/contract.sol").unwrap();

    //create new JSON Object to store gas inefficiencies
    let mut gas_inefficiencies = Map::new();

    optimizor::gas_tricks::bytes32(contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::openzepplin(contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::safemath(contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::token(contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::uint_incur_overhead(contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::check_constructor_absence(contract);
    optimizor::gas_tricks::use_named_retunrs(contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::uint_incur_overhead(contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::mapping_instead_array(contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::uint256_instead_bool(contract, &mut gas_inefficiencies);

    // Convert the gas inefficiencies to JSON
    let gas_inefficiencies_json =
        serde_json::to_string_pretty(&Value::Object(gas_inefficiencies)).unwrap();

    // Save the JSON to a file
    let json_file_path = "report.json";
    fs::write(json_file_path, gas_inefficiencies_json).expect("Unable to write JSON file");

}
