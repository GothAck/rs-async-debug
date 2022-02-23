use async_debug::AsyncDebug;
use tokio::sync::{Mutex, RwLock};

#[derive(Debug, AsyncDebug)]
struct Mixed {
    string: String,
    integer: u64,
    #[async_debug(async_call = RwLock::read(), clone, copy, ty = Vec<String>, skip)]
    rw_lock: RwLock<Vec<String>>,
    #[async_debug(async_call = Mutex::lock, clone, copy , ty = Vec<u64>)]
    mutex: Mutex<Vec<u64>>,
    #[async_debug(async_call = Mutex::lock, copy, ty = u128, skip)]
    mutex_u128: Mutex<u128>,
    #[async_debug(async_call = Mutex::lock(), copy, ty = u64)]
    mutex_u64: Mutex<u64>,
}

#[tokio::main]
async fn main() {
    let mixed = Mixed {
        string: "test".into(),
        integer: 42,
        rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        mutex: Mutex::from(vec![0, 1]),
        mutex_u128: Mutex::from(999),
        mutex_u64: Mutex::from(999),
    };

    assert_eq!(
        format!("{:?}", mixed.async_debug().await),
        "Mixed { string: \"test\", integer: 42, rw_lock: [\"string0\", \"string1\"], mutex: [0, 1], mutex_u128: 999 }",
    );
}