// config.rs
use clap::{App, Arg};

pub fn parse_args() -> (String, Option<String>) {
    let matches = App::new("Solidity Contract Analyzer")
        .version("1.0")
        .author("Malik")
        .about("Analyze Solidity contracts for gas inefficiencies")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Path to the Solidity contract file")
                .required(true),
        )
        .arg(
            Arg::with_name("github")
                .short('g')
                .long("github")
                .value_name("URL")
                .help("GitHub URL to a Solidity contract")
                .conflicts_with("input"),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap().to_string();
    let github_url = matches.value_of("github").map(|url| url.to_string());

    (input_file, github_url)
}
