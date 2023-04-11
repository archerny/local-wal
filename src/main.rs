use std::string::String;
// use bytes::Buf;
fn main() {
    println!("Hello, world!");
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
pub struct WALFile {}

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
