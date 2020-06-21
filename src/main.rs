extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Attribute {
    title: String,
    #[serde(skip_serializing_if="Option::is_none")]
    mysql_scheme: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    command: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Table {
    object_type: String,
    attributes: Vec<Attribute>,
}

fn main() {
    let f = File::open("objects.yaml").unwrap();
    let person: Table = serde_yaml::from_reader(f).unwrap();
    println!("{:?}", person)
}
