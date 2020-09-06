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

fn build_big() -> BigStruct {
  let mut objects = Vec::new();
  for i in 0..5_000_000 {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let start_main = Instant::now();

  let start_load = Instant::now();
  let mut data: Pack<BigStruct> =
    Pack::load_or_init(std::path::PathBuf::from("data"), "big").unwrap();
  println!(
    "Finished loading 1M data file into memory in {:?}",
    start_load.elapsed()
  );

  // std::thread::sleep(std::time::Duration::from_secs(20));

  if data.objects.len() == 0 {
    let start_big_creation = Instant::now();
    let d = build_big();
    println!(
      "1M file created in {} ms",
      start_big_creation.elapsed().as_millis()
    );
    let start_fs_save = Instant::now();
    let _ = std::mem::replace(data.as_mut().unpack(), d);
    println!(
      "1M file replaced and saved in {} ms",
      start_fs_save.elapsed().as_millis()
    );
  }

  let start_update_hi = Instant::now();
  (*data.as_mut()).hi = "Wohoo".to_string();
  data.save().expect("Error while saving");
  println!("Updated hi field in {:?}", start_update_hi.elapsed());

  let start_update_id = Instant::now();
  (*data.as_mut()).counter += 1;
  data.save().expect("Error while saving 2");
  println!("Updated id field in {:?}", start_update_id.elapsed());

  println!("Counter is {}", data.counter);

  println!("Main finished in {:?}", start_main.elapsed());

  Ok(())
}
