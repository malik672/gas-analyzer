// use std::fs;

// fn read_sol_file(file_path: &str) -> Result<String, std::io::Error> {
//     // Read the contents of the Solidity file into a string
//     let content = fs::read_to_string(file_path)?;

//     Ok(content)
// }

// mod optimizor {
//     pub mod gas_tricks;
// }

// #[cfg(test)]
// mod tests {

//     use super::*;
    
//     mod src {
       
//     }

//     #[test]
//     fn test_bytes32() {
//         let contract: &str = &read_sol_file("./revm/example.t.sol").unwrap();
//         optimizor::gas_tricks::bytes32(contract);
//     }

//     #[test]
//     fn test_openzeppelin() {
//         let contract: &str = &read_sol_file("./revm/example.t.sol").unwrap();
//         optimizor::gas_tricks::openzepplin(contract);
//     }

//     #[test]
//     fn safemath() {
//         let contract: &str = &read_sol_file("./revm/example.t.sol").unwrap();
//         optimizor::gas_tricks::safemath(contract);
//     }

//     #[test]
//     fn token() {
//         let contract: &str = &read_sol_file("./revm/example.t.sol").unwrap();
//         optimizor::gas_tricks::token(contract);
//     }

//     #[test]
//     fn uint_incur_overhead() {
//         let contract: &str = &read_sol_file("/home/malik/Desktop/revm/example.t.sol").unwrap();
//         optimizor::gas_tricks::uint_incur_overhead(contract);
//     }

//     #[test]
//     fn constructor() {
//         let contract: &str = &read_sol_file("/home/malik/Desktop/revm/example.t.sol").unwrap();
//         optimizor::gas_tricks::check_constructor_absence(contract);
//     }

    
// }
