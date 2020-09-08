fn main() {
  // packman::fs::create_packfile();
  let a = packman::fs::PackFile::<u32>::from_path("demo_data").unwrap();
  println!("{:?}", a.inodes[0]);
}
