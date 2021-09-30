use home;
use std::fs;
use std::path::PathBuf;
use toml::value::Map;
use toml::Value;

// TODO - deserialize this structure into a HashMap<String, String> with serde

pub fn parse_whoiam() -> Value {
    let path: PathBuf = [
        home::home_dir().expect("Cannot determine home directory."),
        PathBuf::from(".whoiam.toml"),
    ]
    .iter()
    .collect();

    let contents =
        fs::read_to_string(path).expect("Error reading ~/.whoiam.toml file. Check that it exists?");
    let toml = contents.parse::<Value>().unwrap();

    match toml["accounts"].is_table() {
        true => toml["accounts"].clone(), // is there a way to do this without cloning? This shouldn't be needed after the serde part, just curious
        false => Value::Table(Map::new()),
    }
}
