use chrono::prelude::*;
use packman::fs::PackFile;
use std::path::Path;

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
  let pack_file: PackFile = match PackFile::open(Path::new(&path)) {
    Ok(pf) => pf,
    Err(err) => {
      println!("{}", err);
      return Ok(());
    }
  };
  let details = pack_file.metadata();
  println!("PackFile details");
  println!("-----------------");
  println!("Path: {}", details.path);
  println!("ID: {}", details.id);
  println!(
    "Workspace ID: {}",
    match details.workspace_id {
      Some(wid) => format!("{}", wid),
      None => "-".into(),
    }
  );
  println!(
    "Owner: {}",
    match details.owner {
      Some(o) => format!("{}", o),
      None => "-".into(),
    }
  );
  println!(
    "Date created: {}",
    Utc.timestamp(details.date_created as i64, 0)
  );
  println!("Packman version: {}", details.packman_version);
  println!("File version: {}", details.file_version);
  println!("File size in bytes: {}", details.file_size);
  println!("Inode A size: {}", details.inode_size_a);
  println!("Inode A offset: {}", details.inode_offset_a);
  println!("Inode A version: {}", details.inode_version_a);
  println!("Inode B size: {}", details.inode_size_b);
  println!("Inode B offset: {}", details.inode_offset_b);
  println!("Inode B version: {}", details.inode_version_b);
  Ok(())
}
