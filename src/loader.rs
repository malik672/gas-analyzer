use reqwest;
use reqwest::Error;
use serde_json::{Map, Value};
use std::fs;
use crate::config;
use crate::optimizor;

fn read_sol_file(file_path: &str) -> Result<String, std::io::Error> {
    // Read the contents of the Solidity file into a string
    let content = fs::read_to_string(file_path)?;

    Ok(content)
}

async fn read_from_github(path: &str) -> Result<String, Error>  {
    // Send an HTTP GET request to fetch the raw content
    //Path should be in this format: https://raw.githubusercontent.com/user/repo/master/contract.sol
    let client = reqwest::Client::new();
    let response = client.get(path).send().await?;
    let contract_content = response.text().await?;
    println!("{}", contract_content);

     Ok(contract_content)

}


pub async fn loader() {
   
    //  // Parse command-line arguments using the config module
    //  let (input_file, github_url) = config::parse_args();

    //  // Initialize the contract variable
    //  let  contract: String;
     
    //  if let Some(url) = github_url {
    //     // Read from a GitHub URL
    //     contract = read_from_github(&url).await.unwrap();
    // } else {
    //     // Read from a local file
       let contract = read_sol_file("src/contract.sol").unwrap();
    // }


    //create new JSON Object to store gas inefficiencies
    let mut gas_inefficiencies = Map::new();

    optimizor::gas_tricks::bytes32(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::openzepplin(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::safemath(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::token(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::uint_incur_overhead(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::check_constructor_absence(&contract);
    optimizor::gas_tricks::use_named_retunrs(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::uint_incur_overhead(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::mapping_instead_array(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::uint256_instead_bool(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::require_double_logic(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::revert_32(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::do_while(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::priv_constants_immut(&contract, &mut gas_inefficiencies);
    optimizor::gas_tricks::emit_loops(&contract, &mut gas_inefficiencies);

    // Convert the gas inefficiencies to JSON
    let gas_inefficiencies_json =
        serde_json::to_string_pretty(&Value::Object(gas_inefficiencies)).unwrap();

    // Save the JSON to a file
    let json_file_path = "report.json";
    fs::write(json_file_path, gas_inefficiencies_json).expect("Unable to write JSON file");

}
