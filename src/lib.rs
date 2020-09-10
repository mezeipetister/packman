// The MIT License
// Copyright 2020 Peter Mezei <mezeipetister@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Made with (L) from Hungary
// If you need any help please contact me
// at <mezeipetister@gmail.com>

#![feature(test)]

extern crate bincode;
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::default::Default;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Read, Write};
use std::iter::IntoIterator;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

pub mod fs;

/// PackResult<T>
///
/// Generic Pack result type
/// contains Ok(T) or PackError
///
/// ```rust
/// use storaget::*;
/// let res_ok: PackResult<i32> = Ok(32);
/// let res_err: PackResult<i32> = Err(PackError::ObjectNotFound);
/// ```
pub type PackResult<T> = Result<T, PackError>;

/// Pack Error type
/// For internal use
pub enum PackError {
  /// Any error that has a custom message.
  /// Any kind of error that has no other
  /// more specific variant in Error::*
  InternalError(String),
  /// Serialize Error
  /// error occured during serialiuation
  SerializeError(String),
  /// Deserialize Error
  /// error occured during deserialization
  DeserializeError(String),
  /// IO Error
  /// error during file operations
  IOError(String),
  /// Object not found in a storage.
  /// Usually using with get_by_id()
  ObjectNotFound,
  /// Path not found
  /// Using at reading data from path.
  PathNotFound,
  /// ID Taken
  /// When VecPack ID not available
  IDTaken,
  /// The requested file
  /// is not a packfile
  NotPackfile,
  /// When the given file is a packfile
  /// but the version is not ok
  /// (required, found)
  PckflVersionError(u32, u32),
  /// When superblock checksum error
  /// almost impossible
  PckflCorruptedSuperblock,
  /// When both inodes has checksum issues
  /// almost impossible
  PckflCorruptedInodes,
  /// When packfile data is corrupted in both version
  /// almost impossible
  PckflDataError,
  BincodeError(String),
}

// serde_yaml::Error to PackError
// implementation
impl From<serde_yaml::Error> for PackError {
  fn from(from: serde_yaml::Error) -> Self {
    PackError::SerializeError(from.to_string())
  }
}

impl From<Box<bincode::ErrorKind>> for PackError {
  fn from(from: Box<bincode::ErrorKind>) -> Self {
    PackError::BincodeError(from.to_string())
  }
}

// Well formatted display text for users
// TODO: Use error code and language translation
// for end-user error messages.
impl fmt::Display for PackError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      PackError::InternalError(msg) => write!(f, "Internal error: {}", msg),
      PackError::SerializeError(msg) => {
        write!(f, "Pack serialization error: {}", msg)
      }
      PackError::DeserializeError(msg) => {
        write!(f, "Pack deserialization error: {}", msg)
      }
      PackError::IOError(msg) => write!(f, "Pack IO error: {}", msg),
      PackError::PathNotFound => write!(f, "Path not found"),
      PackError::ObjectNotFound => {
        write!(f, "Storage object not found in storage.")
      }
      PackError::IDTaken => write!(f, "VecPack ID already taken"),
      PackError::BincodeError(err) => write!(f, "Bincode error {}", err),
      PackError::NotPackfile => write!(f, "Not packfile, magic error"),
      PackError::PckflVersionError(expected, found) => write!(
        f,
        "Packfile version error. Reuired {}, found {}",
        expected, found
      ),
      PackError::PckflCorruptedSuperblock => {
        write!(f, "Packfile corrupted superblock")
      }
      PackError::PckflCorruptedInodes => write!(f, "Packfile corrupted inodes"),
      PackError::PckflDataError => write!(f, "Packfile corrupted data"),
    }
  }
}

// Well formatted debug text
// TODO: how to support localitation?
impl fmt::Debug for PackError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PackError::InternalError(msg) => write!(f, "Internal error: {}", msg),
      PackError::SerializeError(msg) => {
        write!(f, "Pack serialization error: {}", msg)
      }
      PackError::DeserializeError(msg) => {
        write!(f, "Pack deserialization error: {}", msg)
      }
      PackError::IOError(msg) => write!(f, "Pack IO error: {}", msg),
      PackError::PathNotFound => write!(f, "Path not found"),
      PackError::ObjectNotFound => {
        write!(f, "Storage object not found in storage.")
      }
      PackError::IDTaken => write!(f, "VecPack ID already taken"),
      PackError::BincodeError(err) => write!(f, "Bincode error {}", err),
      PackError::NotPackfile => write!(f, "Not packfile, magic error"),
      PackError::PckflVersionError(expected, found) => write!(
        f,
        "Packfile version error. Reuired {}, found {}",
        expected, found
      ),
      PackError::PckflCorruptedSuperblock => {
        write!(f, "Packfile corrupted superblock")
      }
      PackError::PckflCorruptedInodes => write!(f, "Packfile corrupted inodes"),
      PackError::PckflDataError => write!(f, "Packfile corrupted data"),
    }
  }
}

impl From<io::Error> for PackError {
  fn from(err: io::Error) -> Self {
    PackError::IOError(format!("{}", err))
  }
}

/// Pack<T>
/// Small FS layer around type T
/// Pack is responsible to sync T to the filesystem.
pub struct Pack<T>
where
  T: Serialize + Sized + Clone,
{
  data: T,
  path: PathBuf,
}

/// PackGuard<'a, T>
/// Small mutable guard around type T
/// that implements Drop trait, and save T
/// to the filesystem when PackGuard is dropped.
///
/// Implements deref, deref_mut and drop
pub struct PackGuard<'a, T>
where
  T: Serialize + Sized + Clone,
{
  data: &'a mut T,
  path: &'a PathBuf,
}

/// VecPack<T>
/// Small FS layer around a Vec<Pack<T>>
/// The naming could be confusing a bit, as VecPack<T>
/// is rather FSLayer<Vec<Pack<T>>>, but maybe this could
/// be too long and unnecessary. So VecPack<T> behaves as
/// a special Vec<Pack<T>>.
pub struct VecPack<T>
where
  T: VecPackMember,
{
  data: Vec<Pack<T>>,
  path: PathBuf,
}

/// This trait defines the requirements
/// to be a member of a VecPack<T>
pub trait VecPackMember: Serialize + Sized + Clone {
  // type Target: fmt::Display + std::cmp::PartialEq;
  fn get_id(&self) -> &str;
}

pub trait TryFrom {
  type TryFrom: for<'de> Deserialize<'de> + Serialize + Default + Sized + Clone;
}

/// Save DATA OBJECT to its path
/// Moved this logic into this separated private function
/// as we use it from the Drop implementation and from save method.
fn save_data_object<T>(path: &PathBuf, data: T) -> PackResult<()>
where
  T: Serialize,
{
  let mut buffer = BufWriter::new(File::create(path)?);
  // buffer.write_all(serde_yaml::to_string(&data)?.as_bytes())?;
  buffer.write_all(&bincode::serialize(&data).expect("bincode ser error"))?;
  buffer.flush()?;
  Ok(())
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NOTHING;

impl<'a, T> Pack<T>
where
  for<'de> T: Serialize
    + Deserialize<'de>
    + Default
    + Sized
    + Clone
    + 'a
    + TryFrom
    + std::convert::From<<T as TryFrom>::TryFrom>,
{
  // TODO: Why this is not an infinite loop when T == TryFrom?
  pub fn try_load_from_path(path: PathBuf) -> PackResult<Pack<T>> {
    match Pack::<T>::load_from_path(path.clone()) {
      Ok(pack_t) => Ok(pack_t),
      Err(_) => {
        let data = Pack::<T::TryFrom>::load_from_path(path.clone())?
          .into_inner()
          .into();
        let pack: Pack<T> = Pack {
          data: data,
          path: path,
        };
        pack.save()?;
        Ok(pack)
      }
    }
  }
  pub fn try_load_or_init(
    mut path: PathBuf,
    file_id: &str,
  ) -> PackResult<Pack<T>> {
    if !path.exists() {
      std::fs::create_dir_all(&path)?;
    }
    path.push(&format!("{}.", file_id));
    if !path.exists() {
      Pack::<T>::new(path.clone())?.save()?;
    }
    Pack::try_load_from_path(path)
  }
}

impl<'a, T> Pack<T>
where
  for<'de> T: Serialize + Deserialize<'de> + Default + Sized + Clone + 'a,
{
  // New Pack<T>
  // Private function
  fn new(path: PathBuf) -> PackResult<Self> {
    Ok(Pack {
      data: T::default(),
      path,
    })
  }
  pub fn from_str(buffer: &str, path: PathBuf) -> PackResult<Pack<T>> {
    // match serde_yaml::from_str::<T>(&buffer) {
    //   Ok(t) => Ok(Pack { data: t, path }),
    //   Err(err) => Err(PackError::DeserializeError(err.to_string())),
    // }
    match bincode::deserialize::<T>(&buffer.as_bytes()) {
      Ok(t) => Ok(Pack { data: t, path }),
      Err(err) => Err(PackError::DeserializeError(err.to_string())),
    }
  }
  /// Load Pack<T> from Path
  /// If Path is file and exists, then it tries to load
  /// then deserialize. Otherwise returns PackError.
  pub fn load_from_path(path: PathBuf) -> PackResult<Pack<T>> {
    // let mut file = File::open(&path)?;
    // let mut buffer = String::new();
    // file.read_to_string(&mut buffer)?;
    // Self::from_str(&buffer, path)
    let mut f = File::open(&path).expect("no file found");
    let metadata = std::fs::metadata(&path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    match bincode::deserialize::<T>(&buffer) {
      Ok(t) => Ok(Pack { data: t, path }),
      Err(err) => Err(PackError::DeserializeError(err.to_string())),
    }
  }
  /// Load or init Pack<T> from Path
  /// If Path does not exist, then it tries to create;
  /// Otherwise call Pack::load_from_path(Path).
  pub fn load_or_init(mut path: PathBuf, file_id: &str) -> PackResult<Pack<T>> {
    if !path.exists() {
      std::fs::create_dir_all(&path)?;
    }
    path.push(&format!("{}", file_id));
    if !path.exists() {
      Pack::<T>::new(path.clone())?.save()?;
    }
    Pack::load_from_path(path)
  }
  /// Save Pack<T> manually
  /// to FS. Returns PackError if something
  /// wrong occures.
  pub fn save(&self) -> PackResult<()> {
    save_data_object(&self.path, &self.data)
  }
  /// Update Pack<T>
  /// Tries to update T, if SUCCESS
  /// then tries to save to FS, if SUCCESS
  /// returns R. If Fail, then doing data T
  /// rollback to backup, then return PackError.
  pub fn update<F, R>(&mut self, mut f: F) -> PackResult<R>
  where
    F: FnMut(&mut T) -> R,
  {
    // First clone data as a backup.
    let backup = self.data.clone();
    // Let's do the update process.
    let res = f(&mut self.data);
    // Try to save data to the FS
    match self.save() {
      // If success, then return the update result(s)
      Ok(_) => Ok(res),
      // If there is error occured during
      // saveing updated data
      Err(err) => {
        // Then rollback data to the backup.
        self.data = backup;
        // Return error
        Err(err)
      }
    }
  }
  /// Get(Fn) -> R
  /// Access data through closure
  /// Unmutable data access
  pub fn get<F, R>(&self, f: F) -> R
  where
    F: Fn(&T) -> R,
  {
    f(&self.data)
  }
  /// Map(Fn) -> R
  /// Syntactic sugar for Get(Fn) -> R
  pub fn map<F, R>(&self, f: F) -> R
  where
    F: Fn(&T) -> R,
  {
    f(&self.data)
  }
  /// as_mut() -> PackGuard<'a, T>
  /// returns
  pub fn as_mut(&mut self) -> PackGuard<'_, T> {
    PackGuard {
      data: &mut self.data,
      path: &self.path,
    }
  }
  pub fn into_inner(self) -> T {
    self.data
  }
  pub fn unpack(&self) -> &T {
    &self.data
  }
}

impl<T> Deref for Pack<T>
where
  T: Serialize + Sized + Clone,
{
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl<'a, T> Deref for PackGuard<'a, T>
where
  T: Serialize + Sized + Clone,
{
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

impl<'a, T> DerefMut for PackGuard<'a, T>
where
  T: Serialize + Sized + Clone,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.data
  }
}

impl<'a, T> Drop for PackGuard<'a, T>
where
  T: Serialize + Sized + Clone,
{
  fn drop(&mut self) {
    // TODO: VERY IMPORTANT
    // Implement error LOGGING!
    // This auto save during drop cannot return PackError,
    // we have two options:
    //  - Panic(),
    //  - & | error log
    let _ = save_data_object(&self.path, &self.data);
  }
}

// impl<T> VecPack<T>
// where
//     for<'de> T: VecPackMember + Deserialize<'de> + Default + PackTryFrom,
// {
//     pub fn try_load_or_init(path: PathBuf) -> PackResult<VecPack<T>> {
//         match Self::load_or_init(path) {
//             Ok(res) => Ok(res),
//             Err(err) => Self::try_from().into(),
//         }
//     }
// }

impl<T> VecPack<T>
where
  for<'de> T: VecPackMember
    + Serialize
    + Deserialize<'de>
    + Default
    + Sized
    + Clone
    + TryFrom
    + std::convert::From<<T as TryFrom>::TryFrom>,
{
  pub fn try_load_or_init(path: PathBuf) -> PackResult<VecPack<T>> {
    // If path is a file
    // then panic!
    if path.is_file() {
      panic!(
        "Given VecPack path is not a dir. Path: {}",
        &path.to_str().unwrap()
      );
    }
    // If path does not exist,
    // then we create it.
    if !path.exists() {
      std::fs::create_dir_all(&path)?;
    }
    // Result empty VecPack<T>
    let mut result: VecPack<T> = VecPack::new(path.clone())?;
    // First collect all
    // the file names from path
    std::fs::read_dir(path.clone())?
      .filter_map(|file| {
        file.ok().and_then(|e| {
          e.path().file_name().and_then(|n| {
            n.to_str().map(|s| {
              let mut p = path.clone();
              p.push(s);
              p
            })
          })
        })
      })
      .collect::<Vec<PathBuf>>()
      // Then iter over path vector
      // and try to read and deserialize
      // them.
      .iter()
      .for_each(|path| {
        // Add deserialized T to VecPack<T>
        result
          .insert_pack(
            // Create Pack<T> from T
            Pack::<T>::try_load_from_path(path.clone()).expect(&format!(
              "Cannot deserialize file with ID: {}",
              (&path).to_str().unwrap()
            )),
          )
          .expect(&format!(
            "Error while adding file to VecPack with ID: {}",
            (&path).to_str().unwrap()
          ));
      });
    Ok(result)
  }
}

impl<T> VecPack<T>
where
  for<'de> T: VecPackMember + Deserialize<'de> + Default,
{
  // TODO: Check FS operations. What if path is a file?
  /// New VecPack<T>
  /// Requires a PathBuf and returns an empty VecPack<T>
  pub fn new(path: PathBuf) -> PackResult<VecPack<T>> {
    // Check whether path is a dir, or a file
    // If file, then panic!
    if path.is_file() {
      panic!(
        "Given VecPack path is not a dir. Path: {}",
        &path.to_str().unwrap()
      );
    }
    // If path does not exist,
    // then create it!
    if !path.exists() {
      std::fs::create_dir_all(&path)?;
    }
    // Create an empty VecPack<T>
    Ok(VecPack {
      data: Vec::new(),
      path,
    })
  }
  /// Load or init VecPack by a given Path
  /// If path does not exist,
  /// then we create it, then loads all the files,
  /// and tries to deserialize them.
  /// If a file cannot be read, or cannot be deserialized
  /// then we panic!
  pub fn load_or_init(path: PathBuf) -> PackResult<VecPack<T>> {
    // If path is a file
    // then panic!
    if path.is_file() {
      panic!(
        "Given VecPack path is not a dir. Path: {}",
        &path.to_str().unwrap()
      );
    }
    // If path does not exist,
    // then we create it.
    if !path.exists() {
      std::fs::create_dir_all(&path)?;
    }
    // Result empty VecPack<T>
    let mut result: VecPack<T> = VecPack::new(path.clone())?;
    // First collect all
    // the file names from path
    std::fs::read_dir(path.clone())?
      .filter_map(|file| {
        file.ok().and_then(|e| {
          e.path().file_name().and_then(|n| {
            n.to_str().map(|s| {
              let mut p = path.clone();
              p.push(s);
              p
            })
          })
        })
      })
      .collect::<Vec<PathBuf>>()
      // Then iter over path vector
      // and try to read and deserialize
      // them.
      .iter()
      .for_each(|path| {
        // Add deserialized T to VecPack<T>
        result
          .insert_pack(
            // Create Pack<T> from T
            Pack::<T>::load_from_path(path.clone()).expect(&format!(
              "Cannot deserialize file with ID: {}",
              (&path).to_str().unwrap()
            )),
          )
          .expect(&format!(
            "Error while adding file to VecPack with ID: {}",
            (&path).to_str().unwrap()
          ));
      });
    Ok(result)
  }
  /// Insert a new T to VecPack<T>
  /// Only if ID is not taken
  pub fn insert(&mut self, item: T) -> PackResult<()> {
    // Check if ID whether available
    if !&self.check_id_available(item.get_id()) {
      return Err(PackError::IDTaken);
    }
    // TODO: Move file name creation to a central place!
    let mut p = (&self.path).clone();
    p.push(&format!("{}.yml", item.get_id()));
    let p = Pack {
      data: item,
      path: p,
    };
    p.save()?;
    self.data.push(p);
    Ok(())
  }
  // pub fn remove_by_id(&mut self, id: &str) -> PackResult<()> {
  //     match self.iter().position(|i| i.get_id() == id) {
  //         Some(p) => {
  //             std::fs::remove_file(&self.find_id)?
  //         }
  //         None => Err(PackError::ObjectNotFound),
  //     }
  // }
  /// Insert Pack<T> to VecPack<T>
  /// Only if ID is not taken
  pub fn insert_pack(&mut self, item: Pack<T>) -> PackResult<()> {
    if !&self.check_id_available(item.get_id()) {
      return Err(PackError::IDTaken);
    }
    self.data.push(item);
    Ok(())
  }
  /// Find ID and returns &Pack<T>
  /// as an unmutable reference
  pub fn find_id(&self, id: &str) -> PackResult<&Pack<T>> {
    match self.iter().position(|i| i.get_id() == id) {
      Some(p) => Ok(&self.get(p).unwrap()),
      None => Err(PackError::ObjectNotFound),
    }
  }
  /// Find ID and returns &mut Pack<T>
  /// as a mutable reference
  pub fn find_id_mut(&mut self, id: &str) -> PackResult<&mut Pack<T>> {
    match &mut self.into_iter().position(|i| i.get_id() == id) {
      Some(p) => Ok(self.as_vec_mut().get_mut(*p).unwrap()),
      None => Err(PackError::ObjectNotFound),
    }
  }
  /// Check ID is available
  /// If ID is taken, returns false,
  /// otherwise returns true
  pub fn check_id_available(&self, id: &str) -> bool {
    match self.iter().position(|i| i.get_id() == id) {
      Some(_) => false,
      None => true,
    }
  }
  /// Returns data as a mutable
  /// reference to Vec<Pack<T>>
  pub fn as_vec_mut(&mut self) -> &mut Vec<Pack<T>> {
    &mut self.data
  }
  /// Returns data as unmutable
  /// reference to Vec<Pack<T>>
  pub fn as_vec(&self) -> &Vec<Pack<T>> {
    &self.data
  }
  /// Returns VecPack<T>
  /// &Path
  pub fn get_path(&self) -> &Path {
    &self.path.as_path()
  }
}

// Deref implementation for VecPack<T>
// It returns an unmutable reference to &Vec<Pack<T>>
impl<T> Deref for VecPack<T>
where
  T: VecPackMember,
{
  type Target = Vec<Pack<T>>;
  fn deref(&self) -> &Self::Target {
    &self.data
  }
}

// VecPack mutable iterator
// It implements Iterator and we use it to
// get a mutable iterator for VecPack<T>
// It only holds &'a mut Vec<Pack<T>>.
pub struct VecPackIterMut<'a, T>
where
  T: Serialize + Sized + Clone + 'a,
{
  data: &'a mut [Pack<T>],
}

// Iterator implementation for VecPackIterMut<'a, T>
// Many thank to Alice from Rust Forum
//
// See the thread here:
// https://users.rust-lang.org/t/magic-lifetime-using-iterator-next/34729/5
impl<'a, T> Iterator for VecPackIterMut<'a, T>
where
  T: Serialize + Sized + Clone + 'a,
{
  type Item = &'a mut Pack<T>;
  fn next(&mut self) -> Option<Self::Item> {
    let slice = std::mem::replace(&mut self.data, &mut []);
    match slice.split_first_mut() {
      Some((head, tail)) => {
        self.data = tail;
        Some(head)
      }
      None => None,
    }
  }
}

// Implement IntoIter for VecPack<T>
// TODO: Maybe too dangerous!
// TODO: Remove this implementation?
// impl<T> IntoIterator for VecPack<T>
// where
//     T: Serialize + Sized + Clone,
// {
//     type Item = Pack<T>;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.data.into_iter()
//     }
// }

// Implement IntoIter for &'a mut VecPack<T>
impl<'a, T> IntoIterator for &'a mut VecPack<T>
where
  T: VecPackMember,
{
  type Item = &'a mut Pack<T>;
  type IntoIter = VecPackIterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    VecPackIterMut {
      data: &mut self.data,
    }
  }
}

impl<'a, T> PackGuard<'a, T>
where
  for<'de> T: Serialize + Deserialize<'de> + Default + Sized + Clone + 'a,
{
  pub fn unpack(&mut self) -> &mut T {
    &mut self.data
  }
}

// fn demo(a: &mut VecPack<u32>) {
//     let b = &mut a
//         .into_iter()
//         .map(|i| {
//             (*i.as_mut()) += 1;
//             i.clone()
//         })
//         .collect::<Vec<u32>>();
//     println!("{:?}", b);
// }
