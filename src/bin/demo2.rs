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
  let customer = Customer::new(name, age);

  // Open or init demo packfile
  let mut a: PackFile = packman::fs::PackFile::open_or_init(
    Path::new("data/demo_data_customer"),
    0,
    None,
    None,
    None,
  )
  .unwrap();

  // Write new data to PackFile
  a.write_data(&bincode::serialize(&customer).unwrap())
    .unwrap();

  // Open or init demo packfile again
  let mut a: PackFile = packman::fs::PackFile::open_or_init(
    Path::new("data/demo_data_customer"),
    0,
    None,
    None,
    None,
  )
  .unwrap();

  // Print packfile data
  println!("{:?}, {:?}", a.inodes[0], a.inodes[1]);
  println!("---------------");
  println!("Data is {:?}", a.load_data().unwrap());
  println!("Backup is {:?}", a.load_backup().unwrap());

  let c: Pack<Customer> =
    Pack::load_from_path(PathBuf::from("data/demo_data_customer")).unwrap();

  println!("{:?}", c.unpack());

  // Try load a non packfile
  // let mut b: PackFile<Customer> =
  //   packman::fs::PackFile::open("LICENSE").unwrap();
}
