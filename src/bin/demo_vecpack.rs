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
struct Customer {
  id: u32,
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

impl TryFrom for Customer {
  type TryFrom = Customer;
}

impl VecPackMember for Customer {
  type Out = u32;
  fn get_id(&self) -> &Self::Out {
    &self.id
  }
}

impl Customer {
  fn new(id: u32, name: String, age: u32, address: Address) -> Self {
    Self {
      id,
      name,
      age,
      address,
    }
  }
}

fn main() {
  let mut data: VecPack<Customer> =
    VecPack::try_load_or_init(PathBuf::from("data/demo_vecpack")).unwrap();

  for i in 0..100u32 {
    let customer =
      Customer::new(i, generate_alphanumeric(15), 10 + i, Address::default());
    data.insert(customer).unwrap();
  }

  println!("Len of vecpack is {}", data.len());
  println!("Last item is {:?}", data.last().unwrap().unpack());
  println!("Get by id 4 {:?}", data.find_id(&4u32).unwrap().unpack());
}
