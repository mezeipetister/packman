use crate::*;
use memmap::MmapMut;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::io::{Read, Write};
use std::marker::PhantomData;

const PACKMAN_VERSION: u32 = 1;
const SUPERBLOCK_SIZE: u32 = 1024 * 4; // 4 kib reserved for superblock
const INODE_SIZE: u32 = 1024; // 1 kib reserved for a single inode

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Superblock {
  id: u64,                     // u64 ID
  alias: Option<String>,       // Alias as char array
  owner: Option<String>,       // Owner information
  packman_version_number: u32, // packman version which is used to create this file
  date_created: u64,           // system time in UNIX timestamp (seconds)
  workspace_id: Option<u64>,   // workspace id if there is any
  checksum: u32,               // U32 checksum of the superblock
}

impl Superblock {
  pub fn new(
    id: u64,
    alias: Option<String>,
    owner: Option<String>,
    workspace_id: Option<u64>,
  ) -> Self {
    Self {
      id,
      alias,
      owner,
      packman_version_number: PACKMAN_VERSION,
      date_created: util::now(),
      workspace_id,
      checksum: 0,
    }
  }
  #[allow(dead_code)]
  pub fn serialize(&mut self) -> PackResult<Vec<u8>> {
    self.checksum();
    bincode::serialize(self).map_err(|e| e.into())
  }

  pub fn serialize_into<W>(&mut self, w: W) -> PackResult<()>
  where
    W: Write,
  {
    self.checksum();
    bincode::serialize_into(w, self).map_err(|e| e.into())
  }

  pub fn deserialize_from<R>(r: R) -> PackResult<Self>
  where
    R: Read,
  {
    let mut sb: Self = bincode::deserialize_from(r)?;
    if !sb.verify_checksum() {
      return Err(PackError::BincodeError(
        "Superblock checksum verification failed".into(),
      ));
    }

    Ok(sb)
  }
  fn checksum(&mut self) {
    self.checksum = 0;
    self.checksum = util::calculate_checksum(&self);
  }
  fn verify_checksum(&mut self) -> bool {
    let checksum = self.checksum;
    self.checksum = 0;
    let ok = checksum == util::calculate_checksum(&self);
    self.checksum = checksum;
    ok
  }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Inode {
  version: u64,        // Inode version, inrement one per data update
  offset: u64,         // Absolute offset of data bytes from the beginning
  size: u64,           // Data size in bytes
  date_created: u64,   // Data created in UNIX timestamp (seconds)
  checksum_inode: u32, // U32 checksum of the inode
  checksum_data: u32,  // U32 checksum of the underlying data
}

impl Inode {
  pub fn new(version: u64, offset: u64, size: u64, checksum_data: u32) -> Self {
    Self {
      version,
      offset,
      size,
      date_created: util::now(),
      checksum_inode: 0,
      checksum_data,
    }
  }
  #[allow(dead_code)]
  pub fn serialize(&mut self) -> PackResult<Vec<u8>> {
    self.checksum();
    bincode::serialize(self).map_err(|e| e.into())
  }

  pub fn serialize_into<W>(&mut self, w: W) -> PackResult<()>
  where
    W: Write,
  {
    self.checksum();
    bincode::serialize_into(w, self).map_err(|e| e.into())
  }

  pub fn deserialize_from<R>(r: R) -> PackResult<Self>
  where
    R: Read,
  {
    let mut sb: Self = bincode::deserialize_from(r)?;
    if !sb.verify_checksum() {
      return Err(PackError::BincodeError(
        "Inode checksum verification failed".into(),
      ));
    }

    Ok(sb)
  }
  fn checksum(&mut self) {
    self.checksum_inode = 0;
    self.checksum_inode = util::calculate_checksum(&self);
  }
  fn verify_checksum(&mut self) -> bool {
    let checksum_inode = self.checksum_inode;
    self.checksum_inode = 0;
    let ok = checksum_inode == util::calculate_checksum(&self);
    self.checksum_inode = checksum_inode;
    ok
  }
}

pub(crate) struct PackFile<T> {
  superblock: Option<Superblock>,
  inodes: [Option<Inode>; 2],
  file_ptr: MmapMut,
  data_type: PhantomData<*const T>,
}

impl<T> PackFile<T> {
  fn try_load_data(&mut self) -> T {
    todo!()
  }
  fn from_path(path: &str) -> PackResult<PackFile<T>> {
    todo!()
  }
  fn save_data(&mut self, data: T) -> PackResult<T> {
    todo!()
  }
  fn recover(&mut self) {}
  fn is_healthy(&mut self) -> bool {
    true
  }
  pub(crate) fn new(path: &str) -> PackResult<PackFile<T>> {
    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(path)?;
    todo!();
  }
}

mod util {
  use std::time;

  #[inline]
  pub fn calculate_checksum<S>(s: &S) -> u32
  where
    S: serde::Serialize,
  {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(&bincode::serialize(&s).unwrap());
    hasher.finalize()
  }

  #[inline]
  pub fn now() -> u64 {
    time::SystemTime::now()
      .duration_since(time::UNIX_EPOCH)
      .unwrap()
      .as_secs()
  }
}
