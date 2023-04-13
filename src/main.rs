use std::{
    fs, io,
    path::{Path, PathBuf},
    string::String,
    sync::atomic::AtomicI32,
};
// use bytes::Buf;
fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
pub enum Error {
    /// IOError wrapps io::Error
    IOError(io::Error),

    /// Error about invalid options and arguments
    InvalidArgs(String),
}

// Options for local file based WAL.
pub struct Options {
    // NoSync disables fsync after writes. This is less durable and puts the
    // log at risk of data loss when there's a server crash.
    no_sync: bool,
    // Size of each segment. This is just a target size, actual size may differ.
    // Default is 50 MB.
    segment_size: i32, //MByte
}

#[derive(Debug)]
pub struct WALWritableFile {}

/// The encoding of blocks is largely borrowed from LevelDB's/RocksDB's write ahead log. Log file
/// consists of a sequence of variable length records. Records are grouped by BlockSize(32k).
///       +-----+-------------+--+----+----------+------+-- ... ----+
/// File  | r0  |        r1   |P | r2 |    r3    |  r4  |           |
///       +-----+-------------+--+----+----------+------+-- ... ----+
///       <---- BlockSize ------>|<--- BlockSize ------>|
#[derive(Debug)]
pub struct Block {}

#[derive(Debug)]
pub struct Record {}

pub trait LocalWAL {
    fn new(path: &str) -> Self;
    fn check_exist();
    fn create_dir();
    fn truncate();

    fn write(); // write bytes into WAL
    fn read(seqID: i64); // read data from WAL
    fn open();
    fn close();

    fn first_seq();
    fn last_seq();
}

pub struct WALManager {
    wal_dir: PathBuf,
    log_num: AtomicI32,
}

impl WALManager {

    /// with_wal_dir sets the write ahead log directory
    fn with_wal_dir(self: &mut Self, dir: String) -> Result<(), Error> {
        let path = Path::new(&dir);
        let mut canonicalized_path = PathBuf::new();

        match path.canonicalize() {
            Ok(pathbuf) => canonicalized_path = pathbuf,
            Err(e) => {
                return Err(Error::IOError(e));
            }
        }
        // If the specified path is not on file system, we will create it.
        if !canonicalized_path.exists() {
            match fs::create_dir_all(canonicalized_path.clone()) {
                Ok(()) => println!("test"),
                Err(e) => {
                    return Err(Error::IOError(e));
                }
            }
        }
        // Check the path. If the path is a regular file or something else, the procedure will fail.
        if !canonicalized_path.is_dir() {
            return Err(Error::InvalidArgs(format!(
                "WAL dir path({}) exists, but it is not a directory",
                dir
            )));
        }
        self.wal_dir = canonicalized_path;
        Ok(())
    }

    fn gen_next_full_path(self: &Self) -> String {
        let formatted_num = format!(
            "{:10}",
            self.log_num
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );

        String::from("test")
    }
}

pub struct LocalWALEntry {
    id: i64,
    checksum: i32,
}

pub struct LocalFileWALImpl {
    abs_path: String,
}

impl LocalWAL for LocalFileWALImpl {
    fn new(path: &str) -> Self {
        LocalFileWALImpl {
            abs_path: String::new(),
        }
    }

    fn check_exist() {}

    fn create_dir() {}

    fn truncate() {}

    fn write() {}

    fn read(seqID: i64) {}

    fn open() {}

    fn close() {}

    fn first_seq() {}

    fn last_seq() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Buf;

    #[test]
    fn test_buffer() {
        let mut buf = &b"hello world"[..];

        println!("default value: {:?}", buf);

        assert_eq!(b'h', buf.get_u8());
        assert_eq!(b'e', buf.get_u8());
        assert_eq!(b'l', buf.get_u8());

        let mut rest = [0; 8];
        buf.copy_to_slice(&mut rest);

        assert_eq!(&rest[..], &b"lo world"[..]);
    }
}
