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
  let customer = Customer::new("Peti".into(), 31);
  let mut a =
    packman::fs::PackFile::<Customer>::from_path("demo_data").unwrap();
  a.write_data(&customer);
  println!("{:?}, {:?}", a.inodes[0], a.inodes[1]);
  println!("Data is {:?}", a.try_load_data().unwrap());
}
