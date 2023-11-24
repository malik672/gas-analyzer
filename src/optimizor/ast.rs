use std::fs;
use ethers_solc;
use ethers_solc::{Project};

pub fn ast() {
       // create a project from a configuration file
       let project = Project::builder().build().unwrap();
       let output = project
           .compile_files(vec!["src/contract.sol"])
           .unwrap();
       let artifacts = output.output().sources.0; 
       let mut s_ast = artifacts.values();
       let mut binding = s_ast.next();
       let mut binding = binding.iter_mut().next();
       let s_asts = binding.as_mut().unwrap();
       let ast = s_asts.get(0).unwrap().source_file.ast.clone().unwrap();
       let ast_json = serde_json::to_string(&ast).unwrap();
       fs::write("src/optimizor/ast.json", ast_json).expect("failed to write ast");
}