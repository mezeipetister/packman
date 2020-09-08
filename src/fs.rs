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
pub struct Inode {
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

pub fn create_packfile() {
  PackFile::<u32>::create_new("demo_data", 0, None, None, None).unwrap();
}

pub struct PackFile<T> {
  superblock: Option<Superblock>,
  pub inodes: [Option<Inode>; 2],
  file_ptr: MmapMut,
  data_type: PhantomData<*const T>,
}

impl<T> PackFile<T> {
  fn try_load_data(&mut self) -> T {
    todo!()
  }
  pub fn from_path(path: &str) -> PackResult<PackFile<T>> {
    let file = OpenOptions::new().read(true).write(true).open(path)?;
    let mmap = unsafe { MmapMut::map_mut(&file)? };
    let mut cursor = Cursor::new(&mmap);
    let sb = Superblock::deserialize_from(&mut cursor)?;
    cursor.seek(SeekFrom::Start(SUPERBLOCK_SIZE as u64))?;
    let inode_a = Inode::deserialize_from(&mut cursor)?;
    cursor.seek(SeekFrom::Start((SUPERBLOCK_SIZE + INODE_SIZE) as u64))?;
    let inode_b = Inode::deserialize_from(&mut cursor)?;
    Ok(PackFile {
      superblock: Some(sb),
      inodes: [Some(inode_a), Some(inode_b)],
      file_ptr: mmap,
      data_type: PhantomData,
    })
  }
  fn save_data(&mut self, data: T) -> PackResult<T> {
    todo!()
  }
  fn recover(&mut self) {}
  fn is_healthy(&mut self) -> bool {
    true
  }
  pub fn create_new(
    path: &str,
    id: u64,
    alias: Option<String>,
    owner: Option<String>,
    workspace_id: Option<u64>,
  ) -> PackResult<()> {
    let file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.set_len((SUPERBLOCK_SIZE + INODE_SIZE * 2) as u64)?;
    let mut buf = BufWriter::new(&file);
    let mut sb = Superblock::new(id, alias, owner, workspace_id);
    sb.serialize_into(&mut buf)?;

    let mut inode = Inode::new(0, 0, 0, 0);

    buf.seek(SeekFrom::Start(SUPERBLOCK_SIZE as u64))?;
    inode.serialize_into(&mut buf)?; // save the first inode
    buf.seek(SeekFrom::Start((SUPERBLOCK_SIZE + INODE_SIZE) as u64))?;
    inode.serialize_into(&mut buf)?; // save the second infode

    buf.flush()?;
    Ok(())
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
