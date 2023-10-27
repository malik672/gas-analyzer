use std::fs;

fn read_sol_file(file_path: &str) -> Result<String, std::io::Error> {
    // Read the contents of the Solidity file into a string
    let content = fs::read_to_string(file_path)?;

    Ok(content)
}

mod optimizor {
    pub mod gas_tricks;
}

fn main() {
    let contract: &str = &read_sol_file("/home/malik/Desktop/revm/src/contract.sol").unwrap();
    
    optimizor::gas_tricks::bytes32(contract);
    optimizor::gas_tricks::openzepplin(contract);
    optimizor::gas_tricks::safemath(contract);
    optimizor::gas_tricks::token(contract);
    optimizor::gas_tricks::uint_incur_overhead(contract);
    optimizor::gas_tricks::check_constructor_absence(contract)
 
}
