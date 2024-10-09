mod file_system;
#[cfg(feature = "memory")]
mod memory;
#[cfg(feature = "memory")]
pub use memory::MemoryFileSystem;
#[cfg(feature = "os")]
mod os;
#[cfg(feature = "os")]
pub use os::OsFileSystem;

pub use crate::file_system::FileSystem;
