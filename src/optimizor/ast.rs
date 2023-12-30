use ethers_solc;
use ethers_solc::Project;
use std::fs;

pub fn ast() {
    // create a project from a configuration file
    let project = Project::builder().build().unwrap();
    let output = project.compile_files(vec!["src/contract.sol"]).unwrap();
    let ast = output
        .output()
        .sources
        .0
        .values()
        .next()
        .iter_mut()
        .next()
        .as_mut()
        .unwrap()
        .get(0)
        .unwrap()
        .source_file
        .ast
        .clone()
        .unwrap();
    fs::write("src/optimizor/ast.json", ast_json).expect("failed to write ast");
}
