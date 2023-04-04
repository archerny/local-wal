use std::string::String;
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

pub trait LocalWAL {
    fn new(path: &str)->Self;
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

    fn new(path: &str)->Self {
        LocalFileWALImpl { abs_path: String::new()}
    }

    fn check_exist() {

    }

    fn create_dir() {

    }

    fn truncate() {
    }
}

#[cfg(test)]
mod tests {
}