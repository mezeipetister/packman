use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
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
  // packman::fs::create_packfile();
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
  let customer = Customer::new(name, age);
  let mut a =
    packman::fs::PackFile::<Customer>::from_path("demo_data").unwrap();
  a.write_data(&customer);
  let mut a =
    packman::fs::PackFile::<Customer>::from_path("demo_data").unwrap();
  println!("{:?}, {:?}", a.inodes[0], a.inodes[1]);
  println!("Data is {:?}", a.try_load_data().unwrap());
}
