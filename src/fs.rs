use memmap::MmapMut;

pub(crate) struct Superblock {
  id: [u8; 255],          // 255 bytes len id
  packman_version: u32,   // packman version which is used to create this file
  date_created: u64,      // system time in UNIX timestamp (seconds)
  workspace: Option<u64>, // workspace id if there is any
  owner: [u8; 255],       // owner record
  checksum: u32,          // U32 checksum of the superblock
}

pub(crate) struct Inode {
  version: u64,        // Inode version, inrement one per data update
  offset: u64,         // Absolute offset of data bytes from the beginning
  size: u64,           // Data size in bytes
  date_created: u64,   // Data created in UNIX timestamp (seconds)
  checksum_inode: u32, // U32 checksum of the inode
  checksum_data: u32,  // U32 checksum of the underlying data
}

pub(crate) struct PackFile {
  superblock: Option<Superblock>,
  inodes: [Option<Inode>; 2],
  file_ptr: MmapMut,
}
