use packman::fs::PackFile;
use packman::Pack;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Customer {
  name: String,
  age: u32,
}

impl Customer {
  fn new(name: String, age: u32) -> Self {
    Self { name, age }
  }
}

impl packman::TryFrom for Customer {
  type TryFrom = Customer;
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let name: String = match args.len() {
    x if x > 1 => match args[1].parse::<String>() {
      Ok(_name) => _name,
      Err(_) => "Mezei Péter".into(),
    },
    _ => "Mezei Péter".into(),
  };
  let age: u32 = match args.len() {
    3 => match args[2].parse() {
      Ok(_age) => _age,
      Err(_) => 31,
    },
    _ => 31,
  };

  // Create new customer data
  // let customer = Customer::new(name, age);

  let mut c: Pack<Customer> =
    Pack::try_load_or_init(PathBuf::from("data"), "demo4").unwrap();

  println!("Before: {:?}", c.unpack());

  c.update(|c| {
    c.name = name;
    c.age = age;
  })
  .unwrap();

  println!("After: {:?}", c.unpack());
}
