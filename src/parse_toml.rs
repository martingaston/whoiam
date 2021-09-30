use home;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
struct WhoIAMToml {
    accounts: HashMap<String, String>,
}

pub fn parse_whoiam() -> HashMap<String, String> {
    let path: PathBuf = [
        home::home_dir().expect("Cannot determine home directory."),
        PathBuf::from(".whoiam.toml"),
    ]
    .iter()
    .collect();

    let contents =
        fs::read_to_string(path).expect("Error reading ~/.whoiam.toml file. Check that it exists?");

    let toml: WhoIAMToml = toml::from_str(&contents)
        .expect("Could not parse ~/.whoiam.toml file. Does it have an accounts table?");
    return toml.accounts;
}
