use crate::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::io::{BufReader, BufWriter, Read, Write};
use std::marker::PhantomData;

const PACKMAN_MAGIC: u64 = 0xc1a0babe4e; // cio babe forever
const SUPERBLOCK_SIZE: u32 = 1024 * 4; // 4 kib reserved for superblock
const PACKMAN_VERSION: u32 = 1; // current packman version
const INODE_SIZE: u32 = 1024; // 1 kib reserved for a single inode

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct Superblock {
  magic: u64,                  // Magic
  packman_version_number: u32, // packman version which is used to create this file
  id: u64,                     // u64 ID
  owner: Option<String>,       // Owner info; max 256 character len * 4 max byte
  date_created: u64,           // system time in UNIX timestamp (seconds)
  workspace_id: Option<u64>,   // workspace id if there is any
  checksum: u32,               // U32 checksum of the superblock
}

impl Superblock {
  pub fn new(
    id: u64,
    owner: Option<String>,
    workspace_id: Option<u64>,
  ) -> Self {
    Self {
      magic: PACKMAN_MAGIC,
      packman_version_number: PACKMAN_VERSION,
      id,
      owner,
      date_created: util::now(),
      workspace_id,
      checksum: 0,
    }
  }
  pub fn verify_magic(&self) -> bool {
    self.magic == PACKMAN_MAGIC
  }
  pub fn get_magic(&self) -> u64 {
    self.magic
  }
  pub fn get_packman_version(&self) -> u32 {
    self.packman_version_number
  }
  pub fn get_id(&self) -> u64 {
    self.id
  }
  pub fn get_owner(&self) -> Option<&String> {
    self.owner.as_ref()
  }
  pub fn get_date_created(&self) -> u64 {
    self.date_created
  }
  pub fn get_workspace_id(&self) -> Option<u64> {
    self.workspace_id
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

  pub fn deserialize_from<R>(r: &mut R) -> PackResult<Self>
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
pub struct Inode<T> {
  version: u64,          // Inode version, inrement one per data update
  offset: u64,           // Absolute offset of data bytes from the beginning
  size: u64,             // Data size in bytes
  alias: Option<String>, // Alias
  date_created: u64,     // Data created in UNIX timestamp (seconds)
  checksum_inode: u32,   // U32 checksum of the inode
  checksum_data: u32,    // U32 checksum of the underlying data
  data_type: PhantomData<*const T>,
}

impl<T> Inode<T>
where
  for<'de> T: Serialize + Deserialize<'de> + Debug,
{
  pub fn new(
    alias: Option<String>,
    version: u64,
    offset: u64,
    size: u64,
    checksum_data: u32,
  ) -> Self {
    Self {
      alias,
      version,
      offset,
      size,
      date_created: util::now(),
      checksum_inode: 0,
      checksum_data,
      data_type: PhantomData,
    }
  }
  pub fn get_version(&self) -> u64 {
    self.version
  }
  pub fn get_offset(&self) -> u64 {
    self.offset
  }
  pub fn get_size(&self) -> u64 {
    self.size
  }
  pub fn get_alias(&self) -> Option<&String> {
    self.alias.as_ref()
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
  fn get_data_checksum(&self) -> u32 {
    self.checksum_data
  }
  fn load_data<R>(&self, reader: &mut R) -> PackResult<T>
  where
    R: Read + Seek,
  {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(self.get_size() as usize, 0);
    reader.seek(SeekFrom::Start(self.get_offset()))?;
    reader.read(&mut buf)?;
    let data = bincode::deserialize(&buf)?;
    let data_checksum = util::calculate_checksum(&data);
    if self.get_data_checksum() != data_checksum {
      return Err(PackError::PckflDataError);
    }
    Ok(data)
  }
}

pub struct PackFile<T>
where
  for<'de> T: Serialize + Deserialize<'de> + Debug,
{
  superblock: Option<Superblock>,
  pub inodes: [Inode<T>; 2],
  file_ptr: File,
  data_type: PhantomData<*const T>,
}

impl<T> PackFile<T>
where
  for<'de> T: Serialize + Deserialize<'de> + Debug,
{
  // Be aware of the pointer position! Set it to 0
  pub fn is_pack_file<R>(file_ptr: &mut R) -> PackResult<bool>
  where
    R: Read,
  {
    let mut magic_bytes = [0; 8];
    file_ptr.read(&mut magic_bytes)?;
    let magic: u64 =
      bincode::deserialize_from(Cursor::new(magic_bytes)).unwrap_or(0);
    Ok(magic == PACKMAN_MAGIC)
  }

  // Be aware of the pointer position! Set it to 8
  // (is_valid_version, requred version, found version)
  pub fn is_valid_version<R>(file_ptr: &mut R) -> PackResult<(bool, u32, u32)>
  where
    R: Read,
  {
    let mut version_bytes = [0; 4];
    file_ptr.read(&mut version_bytes)?;
    let version: u32 =
      bincode::deserialize_from(Cursor::new(version_bytes)).unwrap_or(0);
    Ok((version == PACKMAN_VERSION, PACKMAN_VERSION, version))
  }

  // Open file if it does exist
  // Otherwise it will create one
  pub fn open_or_init(
    path: &str,
    id: u64,
    alias: Option<String>,
    owner: Option<String>,
    workspace_id: Option<u64>,
    data: &T,
  ) -> PackResult<PackFile<T>> {
    // If path does not exist
    // Init it!
    if !std::fs::metadata(&path).is_ok() {
      PackFile::<T>::init(&path, id, alias, owner, workspace_id, &data)?;
    }
    PackFile::<T>::open(path)
  }

  // Open file if it exists
  // otherwise error
  pub fn open(path: &str) -> PackResult<PackFile<T>> {
    // Try open or error
    let file = OpenOptions::new().read(true).write(true).open(path)?;

    // Create a bufreader to read bytes
    let mut reader = BufReader::new(&file);

    // Try set cursor to 0
    reader.seek(SeekFrom::Start(0))?;
    if !PackFile::<T>::is_pack_file(&mut reader)? {
      return Err(PackError::NotPackfile);
    }

    // Lets check packfile version
    // No need to set seek position
    // as the version follows the magic
    let version_check = PackFile::<T>::is_valid_version(&mut reader)?;
    if !version_check.0 {
      return Err(PackError::PckflVersionError(
        version_check.1,
        version_check.2,
      ));
    }

    // Read superblock
    // Set cursor to 0 again
    reader.seek(SeekFrom::Start(0))?;
    let sb = Superblock::deserialize_from(&mut reader)?;

    // Read first inode
    // Set cursor to the first inode position
    // util::inode_offset_first()
    reader.seek(SeekFrom::Start(util::inode_offset_first()))?;
    let inode_a = Inode::deserialize_from(&mut reader)?;

    // Read second inode
    // Set cursor to the second inode position
    // util::inode_offset_second()
    reader.seek(SeekFrom::Start(util::inode_offset_second()))?;
    let inode_b = Inode::deserialize_from(&mut reader)?;

    // Create PackFile
    let pfile = PackFile {
      superblock: Some(sb),
      inodes: [inode_a, inode_b],
      file_ptr: file,
      data_type: PhantomData,
    };

    Ok(pfile)
  }

  // Load data from PackFile
  // Try latest
  // or try backup
  pub fn load_data(&mut self) -> PackResult<T> {
    let mut reader = BufReader::new(&self.file_ptr);
    match self.get_latest_inode().load_data(&mut reader) {
      Ok(data) => Ok(data),
      Err(err) => match err {
        PackError::PckflDataError => {
          // TODO: here we should trace or log?
          // TODO: also we must set the version to 0 for the current latest inode
          self.get_backup_inode().load_data(&mut reader)
        }
        _ => Err(err),
      },
    }
  }
  pub fn load_backup(&mut self) -> PackResult<T> {
    let mut reader = BufReader::new(&self.file_ptr);
    self.get_backup_inode().load_data(&mut reader)
  }
  fn save_data(&mut self, data: T) -> PackResult<T> {
    todo!()
  }
  fn recover(&mut self) {}
  fn is_healthy(&mut self) -> bool {
    true
  }
  pub fn init(
    path: &str,
    id: u64,
    alias: Option<String>,
    owner: Option<String>,
    workspace_id: Option<u64>,
    data: &T,
  ) -> PackResult<()> {
    let file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.set_len((SUPERBLOCK_SIZE + INODE_SIZE * 2) as u64)?;
    let mut buf = BufWriter::new(&file);
    let mut sb = Superblock::new(id, owner, workspace_id);
    sb.serialize_into(&mut buf)?;

    let mut inode_a = Inode::<T>::new(
      alias.clone(),
      1,
      (SUPERBLOCK_SIZE + 2 * INODE_SIZE + 1) as u64,
      bincode::serialized_size(&data)?,
      util::calculate_checksum(&data),
    );
    let mut inode_b = Inode::<T>::new(alias, 0, 0, 0, 0);

    buf.seek(SeekFrom::Start(SUPERBLOCK_SIZE as u64))?;
    inode_a.serialize_into(&mut buf)?; // save the first inode
    buf.seek(SeekFrom::Start((SUPERBLOCK_SIZE + INODE_SIZE) as u64))?;
    inode_b.serialize_into(&mut buf)?; // save the second infode

    // Set cursor to the data position
    buf.seek(SeekFrom::Start(inode_a.get_offset()))?;
    bincode::serialize_into(&mut buf, &data)?;

    buf.flush()?;
    Ok(())
  }
  fn get_latest_inode(&self) -> &Inode<T> {
    use InodePosition::*;
    match self.get_latest_inode_position() {
      First => &self.inodes[0],
      Second => &self.inodes[1],
    }
  }
  fn get_backup_inode(&self) -> &Inode<T> {
    use InodePosition::*;
    match self.get_latest_inode_position() {
      First => &self.inodes[1],
      Second => &self.inodes[0],
    }
  }
  fn get_latest_inode_position(&self) -> InodePosition {
    use InodePosition::*;
    if self.inodes[0].get_version() > self.inodes[1].get_version() {
      return First;
    } else {
      return Second;
    }
  }
  // Returns (offset in bytes, size in bytes)
  fn allocate_data(&self, data: &T) -> PackResult<(u64, u64)> {
    let required_size = bincode::serialized_size(&data)?;
    if self.get_latest_inode().get_offset() == 0 {
      return Ok((
        (SUPERBLOCK_SIZE + 2 * INODE_SIZE) as u64 + 1,
        required_size,
      ));
    }
    if self.get_latest_inode().get_offset()
      - (SUPERBLOCK_SIZE + 2 * INODE_SIZE) as u64
      > required_size
    {
      return Ok((
        (SUPERBLOCK_SIZE + 2 * INODE_SIZE) as u64 + 1,
        required_size,
      ));
    } else {
      return Ok((
        (self.get_latest_inode().get_offset()
          + self.get_latest_inode().get_size()
          + 1),
        required_size,
      ));
    }
  }
  fn update_first_inode<R>(
    &self,
    inode: &mut Inode<T>,
    reader: &mut R,
  ) -> PackResult<()>
  where
    R: Write + Seek,
  {
    reader.seek(SeekFrom::Start(util::inode_offset_first()))?;
    inode.serialize_into(reader)?;
    Ok(())
  }
  fn update_second_inode<R>(
    &self,
    inode: &mut Inode<T>,
    reader: &mut R,
  ) -> PackResult<()>
  where
    R: Write + Seek,
  {
    reader.seek(SeekFrom::Start(util::inode_offset_second()))?;
    inode.serialize_into(reader)?;
    Ok(())
  }
  pub fn write_data(&mut self, data: &T) -> PackResult<()> {
    let current_file_len = match self.file_ptr.metadata() {
      Ok(file_meta) => file_meta.len(),
      Err(_) => 0,
    };
    let latest_position = self.get_latest_inode_position();
    let latest_inode = self.get_latest_inode();
    let latest_inode_offset = self.get_latest_inode().get_offset();
    let latest_inode_size = self.get_latest_inode().get_size();
    let (offset, size) = self.allocate_data(&data)?;
    let checksum_data = util::calculate_checksum(&data);
    let mut new_inode = Inode::<T>::new(
      match latest_inode.get_alias() {
        Some(alias) => Some(alias.into()),
        None => None,
      },
      latest_inode.get_version() + 1,
      offset,
      size,
      checksum_data,
    );

    let mut cursor = BufWriter::new(&self.file_ptr);

    match latest_position {
      InodePosition::First => {
        // Save new inode to the 2nd position
        self.update_second_inode(&mut new_inode, &mut cursor)?;
      }
      InodePosition::Second => {
        // Save new inode to the first position
        self.update_first_inode(&mut new_inode, &mut cursor)?;
      }
    };

    // Resize the file if needed
    if new_inode.get_offset() > latest_inode_offset {
      if current_file_len <= new_inode.get_offset() + new_inode.get_size() {
        let mut v = Vec::new();
        v.resize(
          new_inode.get_offset() as usize + new_inode.get_size() as usize,
          0,
        );
        self
          .file_ptr
          .set_len(new_inode.get_offset() + new_inode.get_size())?;
      }
    } else {
      // If the file is larger then needed, then reduce its size
      if current_file_len > latest_inode_offset + latest_inode_size {
        self
          .file_ptr
          .set_len(latest_inode_offset + latest_inode_size)?;
      }
    }

    cursor.seek(SeekFrom::Start(offset))?;
    bincode::serialize_into(&mut cursor, &data)?;
    cursor.flush()?;
    Ok(())
  }
}

enum InodePosition {
  First,
  Second,
}

mod util {
  use serde::{Deserialize, Serialize};
  use std::fmt::Debug;

  use crate::fs::Inode;
  use crate::fs::INODE_SIZE;
  use crate::fs::SUPERBLOCK_SIZE;
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

  #[inline]
  pub(crate) fn superblock_offset() -> u64 {
    0
  }

  #[inline]
  pub(crate) fn inode_offset_first() -> u64 {
    SUPERBLOCK_SIZE as u64
  }

  #[inline]
  pub(crate) fn inode_offset_second() -> u64 {
    SUPERBLOCK_SIZE as u64 + INODE_SIZE as u64
  }

  #[inline]
  pub(crate) fn file_size<T>(inode_a: &Inode<T>, inode_b: &Inode<T>) -> u64
  where
    for<'de> T: Serialize + Deserialize<'de> + Debug,
  {
    // inode which data has the last position allocated
    let last_position_data_inode =
      if inode_a.get_offset() > inode_b.get_offset() {
        inode_a
      } else {
        inode_b
      };
    last_position_data_inode.get_offset() + last_position_data_inode.get_size()
  }
}
