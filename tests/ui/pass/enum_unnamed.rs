use async_debug::AsyncDebug;
use tokio::sync::{Mutex, RwLock};

#[derive(Debug, AsyncDebug)]
enum Mixed {
    Variant1(
        String,
        u64,
        #[async_debug(parse = RwLock::read, clone, ty = Vec<String>)] RwLock<Vec<String>>,
        #[async_debug(parse = Mutex::lock, clone, ty = Vec<u64>)] Mutex<Vec<u64>>,
        #[async_debug(parse = Mutex::lock, copy, ty = u128)] Mutex<u128>,
    ),
    Variant2(
        String,
        #[async_debug(parse = RwLock::read, clone, ty = Vec<String>)] RwLock<Vec<String>>,
        #[async_debug(parse = Mutex::lock, clone, ty = Vec<u64>)] Mutex<Vec<u64>>,
        #[async_debug(parse = Mutex::lock, copy, ty = u128)] Mutex<u128>,
    ),
}

#[tokio::main]
async fn main() {
    let mixed_v1 = Mixed::Variant1(
        "test".into(),
        42,
        RwLock::from(vec!["string0".into(), "string1".into()]),
        Mutex::from(vec![0, 1]),
        Mutex::from(999),
    );

    assert_eq!(
        format!("{:?}", mixed_v1.async_debug().await),
        "Variant1(\"test\", 42, [\"string0\", \"string1\"], [0, 1], 999)",
    );

    let mixed_v2 = Mixed::Variant2(
        "test".into(),
        RwLock::from(vec!["string0".into(), "string1".into()]),
        Mutex::from(vec![0, 1]),
        Mutex::from(999),
    );

    assert_eq!(
        format!("{:?}", mixed_v2.async_debug().await),
        "Variant2(\"test\", [\"string0\", \"string1\"], [0, 1], 999)",
    );
}
