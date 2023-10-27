
mod optimizor {
    pub mod gas_tricks;
}

fn main() {
    let contract = "
      string name
      constructor() {
        
      }
    ";
    
    optimizor::gas_tricks::bytes32(contract);
    optimizor::gas_tricks::openzepplin(contract);
    optimizor::gas_tricks::safemath(contract);
    optimizor::gas_tricks::token(contract);
    optimizor::gas_tricks::uint_incur_overhead(contract);
    optimizor::gas_tricks::check_constructor_absence(contract)
 
}
