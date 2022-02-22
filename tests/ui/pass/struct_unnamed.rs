use async_debug::AsyncDebug;
use tokio::sync::{Mutex, RwLock};

#[derive(Debug, AsyncDebug)]
struct Mixed(
    String,
    u64,
    #[async_debug(parse = RwLock::read, clone, ty = Vec<String>)] RwLock<Vec<String>>,
    #[async_debug(parse = Mutex::lock, clone, ty = Vec<u64>)] Mutex<Vec<u64>>,
    #[async_debug(parse = Mutex::lock, copy, ty = u128)] Mutex<u128>,
);

#[tokio::main]
async fn main() {
    let mixed = Mixed(
        "test".into(),
        42,
        RwLock::from(vec!["string0".into(), "string1".into()]),
        Mutex::from(vec![0, 1]),
        Mutex::from(999),
    );

    assert_eq!(
        format!("{:?}", mixed.async_debug().await),
        "MixedAsyncDebug(\"test\", 42, [\"string0\", \"string1\"], [0, 1], 999)",
    );
}
