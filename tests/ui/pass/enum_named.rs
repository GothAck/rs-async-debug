use async_debug::AsyncDebug;
use tokio::sync::{Mutex, RwLock};

#[derive(Debug, AsyncDebug)]
enum Mixed {
    Variant1 {
        string: String,
        integer: u64,
        #[async_debug(async_call = RwLock::read, clone, ty = Vec<String>)]
        rw_lock: RwLock<Vec<String>>,
        #[async_debug(async_call = Mutex::lock, clone, ty = Vec<u64>)]
        mutex: Mutex<Vec<u64>>,
        #[async_debug(async_call = Mutex::lock, copy, ty = u128)]
        mutex_u128: Mutex<u128>,
    },
    Variant2 {
        v2_string: String,
        #[async_debug(async_call = RwLock::read, clone, ty = Vec<String>)]
        v2_rw_lock: RwLock<Vec<String>>,
        #[async_debug(async_call = Mutex::lock, clone, ty = Vec<u64>)]
        v2_mutex: Mutex<Vec<u64>>,
        #[async_debug(async_call = Mutex::lock, copy, ty = u128)]
        v2_mutex_u128: Mutex<u128>,
    },
}

#[tokio::main]
async fn main() {
    let mixed_v1 = Mixed::Variant1 {
        string: "test".into(),
        integer: 42,
        rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        mutex: Mutex::from(vec![0, 1]),
        mutex_u128: Mutex::from(999),
    };

    assert_eq!(
        format!("{:?}", mixed_v1.async_debug().await),
        "Variant1 { string: \"test\", integer: 42, rw_lock: [\"string0\", \"string1\"], mutex: [0, 1], mutex_u128: 999 }",
    );

    let mixed_v2 = Mixed::Variant2 {
        v2_string: "test".into(),
        v2_rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        v2_mutex: Mutex::from(vec![0, 1]),
        v2_mutex_u128: Mutex::from(999),
    };

    assert_eq!(
        format!("{:?}", mixed_v2.async_debug().await),
        "Variant2 { v2_string: \"test\", v2_rw_lock: [\"string0\", \"string1\"], v2_mutex: [0, 1], v2_mutex_u128: 999 }",
    );
}
