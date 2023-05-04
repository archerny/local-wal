use std::{
    fs, io,
    path::{Path, PathBuf},
    string::String,
    sync::atomic::AtomicI32,
};

use bytes::BytesMut;
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

/// The record format is also borrowed from RocksDB's `Legacy Record Format`
/// +---------+-----------+-----------+--- ... ---+
/// |CRC (4B) | Size (2B) | Kind (1B) | Payload   |
/// +---------+-----------+-----------+--- ... ---+
/// 
/// CRC = 32bit hash computed over the payload using CRC
/// Size = Length of the payload data
/// Kind = Type of record
///       (kZeroType, kFullType, kFirstType, kLastType, kMiddleType )
///       The type is used to group a bunch of records together to represent
///       blocks that are larger than kBlockSize
/// Payload = Byte stream as long as specified by the payload size
#[derive(Debug)]
pub struct Record {
    crc32: u32,
    size: u16,
    kind: u8,
    payload: BytesMut,
}

/// The Kind flag has the following states:
/// 0: rest of page will be empty
/// 1: a full record encoded in a single fragment
/// 2: first fragment of a record
/// 3: middle fragment of a record
/// 4: final fragment of a record
pub const R_KIND_ZERO: u8 = 0x0;
pub const R_KIND_FULL: u8 = 0x1;
pub const R_KIND_FIRST: u8 = 0x2;
pub const R_KIND_MIDDLE: u8 = 0x3;
pub const R_KIND_LAST: u8 = 0x4;

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

    /// gen_next_full_path generates the full path of next write ahead log
    fn gen_next_full_path(self: &Self) -> PathBuf {
        let formatted_num = format!(
            "{:10}",
            self.log_num
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );
        self.wal_dir.clone().join(formatted_num)
    }
}


pub struct WritableLogFile {
    full_path: PathBuf,
}

impl WritableLogFile {
    fn open()->Result<Self, Error> {
    }

    fn create()->Result<Self, Error> {

    }

    fn close(self: &Self)->Result<(), Error> {
        Ok(())
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
    use crc32fast::Hasher;
    use crc32fast;

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

    #[test]
    fn test_crc32() {
        let checksum = crc32fast::hash(b"foo bar baz");
    }
}
