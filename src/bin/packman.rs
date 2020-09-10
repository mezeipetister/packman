use packman::fs::PackFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = std::env::args().collect();
  let path: String = match args.len() {
    x if x == 2 => match args[1].parse() {
      Ok(p) => p,
      Err(_) => "".into(),
    },
    _ => {
      println!("Please provide a packfile path");
      return Ok(());
    }
  };
  Ok(())
}
