use std::path::{Path, PathBuf};

use tokio::sync::{Mutex, RwLock};

#[derive(Debug)]
#[allow(dead_code)]
struct Mixed<T: AsRef<Path>> {
    string: String,
    integer: u64,
    rw_lock: RwLock<Vec<String>>,
    mutex: Mutex<Vec<u64>>,
    mutex_u128: Mutex<u128>,
    path: T,
}

fn main() {
    let mixed = Mixed {
        string: "test".into(),
        integer: 42,
        rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        mutex: Mutex::from(vec![0, 1]),
        mutex_u128: Mutex::from(999),
        path: PathBuf::new(),
    };

    println!("{:?}", mixed);
}
