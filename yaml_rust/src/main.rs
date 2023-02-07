use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    addresses: Vec<String>,
}

fn main() {
    let mut file = File::open("person.yaml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let p: Person = serde_yaml::from_str(&contents).unwrap();

    println!("{:#?}", p.addresses);
}