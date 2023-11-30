#[cfg(test)]
mod tests {
    use gas_analyzer::{optimizor, utils};
    use serde_json::{Map, Value};
    use std::fs;

    fn read_sol_file(file_path: &str) -> Result<String, std::io::Error> {
        // Read the contents of the Solidity file into a string
        let content = fs::read_to_string(file_path)?;

        Ok(content)
    }
    #[test]
    fn test_singe_bytes32() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::bytes32::bytes32(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("Use Uint256 instead of bytes32 to store constant")
        );
    }

    #[test]
    fn test_openzepplin() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::openzeppelin::openzepplin(&contract, &mut gas_inefficiencies);
        assert_eq!(gas_inefficiencies.get("line_0").and_then(Value::as_str), Some("instead of using openzeppelin we can use solady which is way cheaper and way efficient [https://github.com/Vectorized/solady"));
    }

    #[test]
    fn test_constrcutor_check() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::openzeppelin::openzepplin(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("making the constructor payable saves gas in deployment cost")
        );
    }

    #[test]
    fn test_do_while() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::do_while::do_while(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("do while loops are cheaper than loops and consume less gas")
        );
    }

    #[test]
    fn test_emit_loops() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::do_while::do_while(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("emiting events in loops cost more, and should be done using other means")
        );
    }

    #[test]
    fn test_mapping_array() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::do_while::do_while(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("Use mapping instead of array")
        );
    }

    #[test]
    fn test_priv_constants_immut() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::priv_constants_immut::priv_constants_immut(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("variables that are constant should have a visibility of private")
        );
    }

    #[test]
    fn test_require_double_logic() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::require_double_logic::require_double_logic(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("split require statements that use && into two seperate parts to save gas")
        );
    }

    #[test]
    fn test_revert32() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::revert_32::revert_32(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("revert statement that has it's string longer than 32 length is always more expensive")
        );
    }

    #[test]
    fn test_safemath() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::safemath::safemath(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("SafeMath is no longer needed since solidity version 0.8.0, use of safeMath can be considered unnessary")
        );
    }

    #[test]
    fn test_token() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::token::token(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("For string of length less than 33, its better to use uint256 to store them")
        );
    }

    #[test]
    fn test_uint_incur_overhead() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::uint_incur_overhead::uint_incur_overhead(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("instead of a uint24, uint16 or any uint and int type apart from uint256 or int256, it's way better to use uint256 or int256")
        );
    }

    #[test]
    fn test_uint_instead_bool() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::uint256_instead_bool::uint256_instead_bool(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("Use uint256 type to store boolean value instead of bool")
        );
    }

    #[test]
    fn test_named_returns() {
        //Generate the ast
        optimizor::ast::ast();

        //create new JSON Object to store gas
        let mut gas_inefficiencies = Map::new();
        optimizor::struct_packing::struct_packing(&mut gas_inefficiencies, 0);
        let contract = read_sol_file("src/contract.sol").unwrap();
        optimizor::use_named_returns::use_named_retunrs(&contract, &mut gas_inefficiencies);
        assert_eq!(
            gas_inefficiencies.get("line_0").and_then(Value::as_str),
            Some("Use named returns")
        );
    }

}
