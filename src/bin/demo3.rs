use nanoid::nanoid;
use packman::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

pub fn generate_alphanumeric(length: usize) -> String {
  nanoid!(
    length,
    &[
      'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
      'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
      '2', '3', '4', '5', '6', '7', '8', '9',
    ]
  )
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct Obj {
  id: String,
  name: String,
  age: u32,
  address: Address,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct Address {
  zip: String,
  location: String,
  street: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct BigStruct {
  id: String,
  name: String,
  comment: String,
  objects: Vec<Obj>,
  hi: String,
  counter: u32,
}

impl TryFrom for BigStruct {
  type TryFrom = BigStruct;
}

fn build_big(n: usize) -> BigStruct {
  let mut objects = Vec::new();
  for i in 0..n {
    let o = Obj {
      id: generate_alphanumeric(10),
      name: generate_alphanumeric(20),
      age: 9,
      address: Address {
        zip: generate_alphanumeric(4),
        location: generate_alphanumeric(25),
        street: generate_alphanumeric(20),
      },
    };
    objects.push(o);
  }
  BigStruct {
    id: "HelloBello".to_string(),
    name: generate_alphanumeric(100),
    comment: generate_alphanumeric(200),
    objects: objects,
    hi: "Hi".to_string(),
    counter: 0,
  }
}

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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct Superblock {
  magic: [u8; 7],
  version: u32,
  id: u64,
}

const PACKMAN_MAGIC: &'static [u8; 7] = b"packman";

impl Superblock {
  fn new() -> Self {
    Self {
      magic: *PACKMAN_MAGIC,
      version: 1,
      id: 19,
    }
  }
}

fn print_bytes(b: &[u8]) {
  let s = bincode::serialized_size(b).unwrap();
  println!("str size is {}", &s);
}

fn main() {
  print_bytes("hellobello".as_bytes());
  println!("{:?}", bincode::serialize(b"hellobello").unwrap());
  println!("{:?}", bincode::serialize("hellobello".as_bytes()).unwrap());
  // let in_u64: u64 = bincode::deserialize(&s).unwrap();
  // println!("{:x}", &in_u64);
  // println!("b/packman in u64 {}", in_u64);
  // println!("{:?}", bincode::serialize(&Superblock::new()).unwrap());
  // // packman::fs::create_packfile();
  // // let data = build_big(1_000_000);
  // // println!("data build done!");
  // // let mut a =
  // //   packman::fs::PackFile::<BigStruct>::from_path("demo_data").unwrap();
  // // a.write_data(&data);
  // let mut a =
  //   packman::fs::PackFile::<BigStruct>::from_path("demo_data").unwrap();
  // // 0123456789abcdef
  // println!(
  //   "{:?}, size is {}",
  //   bincode::serialize(&magic).unwrap(),
  //   bincode::serialized_size(&magic).unwrap()
  // );
  // println!("{:?}, {:?}", a.inodes[0], a.inodes[1]);
  // println!("Data is {:?}", a.try_load_data().unwrap().objects.first());
}
