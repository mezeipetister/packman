fn main() {
  let res = bincode::serialize("hello bello").unwrap();
  println!("Len is {}", std::mem::size_of_val(&res));
}
