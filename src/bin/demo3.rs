use nanoid::nanoid;
use packman::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
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
    if i % 100_000 == 0 {
      println!("{}", i);
    }
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

fn main() {
  // let big_struct_data = build_big(3_000_000);

  // println!("Build done!");

  // let mut big_struct: Pack<BigStruct> =
  //   Pack::try_load_or_init(PathBuf::from("data"), "big_struct_demo3").unwrap();

  // let _ = std::mem::replace(big_struct.as_mut().unpack(), big_struct_data);
  // println!("Save start");
  // let start = std::time::Instant::now();
  // big_struct.save().unwrap();
  // println!("Save finished in {:?}", start.elapsed());

  let start = std::time::Instant::now();
  let big_struct: Pack<BigStruct> =
    Pack::try_load_or_init(PathBuf::from("data"), "big_struct_demo3").unwrap();

  println!("Object len is {}", big_struct.unpack().objects.len());
  println!("Time elapsed {:?}", start.elapsed());
}
