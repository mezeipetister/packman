use packman::fs::PackFile;
use packman::Pack;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let mut c: Pack<i32> =
        Pack::load_or_init(PathBuf::from("data"), "d1").unwrap();

    c.update(|c| {
        *c = 42;
    })
    .unwrap();
}
