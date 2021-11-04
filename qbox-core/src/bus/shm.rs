use super::{EventBus, Token, Topic};
use ahash::AHashMap;
use anyhow::Result;
use memmap2::{Mmap, MmapAsRawDesc, MmapMut, MmapOptions};
use parking_lot::RwLock;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::{any::Any, sync::Arc};
#[doc = "共享内存消息总线"]
pub struct SharedMemoryBus<T> {
    ridx: usize,
    widx: usize,
    inner: MmapMut,
    evfd: usize,
    subscriber: Arc<RwLock<AHashMap<Topic, Vec<(Token, Box<dyn Fn(Topic, T) + Send + Sync>)>>>>,
}

impl<T> SharedMemoryBus<T> {
    pub fn new<S: AsRef<Path>>(path: S) -> Result<Self> {
        let file = File::open(path)?;
        let inner = unsafe { MmapOptions::new().map_mut(&file)? };
        let subscriber = Arc::new(RwLock::new(AHashMap::new()));

        Ok(Self {
            subscriber,
            inner,
            ridx: 0,
            widx: 0,
            evfd: 0,
        })
    }
}

// fn page_size() -> usize {
//     unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
// }

// fn eventfd() -> Result<File, std::io::Error> {
//     let x = unsafe { libc::eventfd(0, libc::EFD_CLOEXEC) };
//     if x == -1 {
//         Err(std::io::Error::last_os_error())
//     } else {
//         Ok(unsafe { File::from_raw_fd(x) })
//     }
// }
