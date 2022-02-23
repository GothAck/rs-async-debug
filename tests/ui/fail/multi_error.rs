use async_debug::AsyncDebug;
use tokio::sync::{Mutex, RwLock};

#[derive(Debug, AsyncDebug)]
struct StructNamed {
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

#[derive(Debug, AsyncDebug)]
enum EnumNamed {
    Variant1 {
        string: String,
        integer: u64,
        #[async_debug(async_call = RwLock::read(), clone, copy, ty = Vec<String>, skip)]
        rw_lock: RwLock<Vec<String>>,
        #[async_debug(async_call = Mutex::lock, clone, copy, ty = Vec<u64>)]
        mutex: Mutex<Vec<u64>>,
        #[async_debug(async_call = Mutex::lock, copy, ty = u128, skip)]
        mutex_u128: Mutex<u128>,
        #[async_debug(async_call = Mutex::lock(), copy, ty = u64)]
        mutex_u64: Mutex<u64>,
    },
    Variant2 {
        v2_string: String,
        #[async_debug(async_call = RwLock::read(), clone, copy, ty = Vec<String>, skip)]
        v2_rw_lock: RwLock<Vec<String>>,
        #[async_debug(async_call = Mutex::lock, clone, copy, ty = Vec<u64>)]
        v2_mutex: Mutex<Vec<u64>>,
        #[async_debug(async_call = Mutex::lock, copy, ty = u128, skip)]
        v2_mutex_u128: Mutex<u128>,
        #[async_debug(async_call = Mutex::lock(), copy, ty = u64)]
        v2_mutex_u64: Mutex<u64>,
    },
}

#[tokio::main]
async fn main() {
    let struct_named = StructNamed {
        string: "test".into(),
        integer: 42,
        rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        mutex: Mutex::from(vec![0, 1]),
        mutex_u128: Mutex::from(999),
        mutex_u64: Mutex::from(999),
    };

    assert_eq!(
        format!("{:?}", struct_named.async_debug().await),
        "Mixed { string: \"test\", integer: 42, rw_lock: [\"string0\", \"string1\"], mutex: [0, 1], mutex_u128: 999, mutex_u64: 999 }",
    );

    let enum_named_v1 = EnumNamed::Variant1 {
        string: "test".into(),
        integer: 42,
        rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        mutex: Mutex::from(vec![0, 1]),
        mutex_u128: Mutex::from(999),
        mutex_u64: Mutex::from(999),
    };

    assert_eq!(
        format!("{:?}", enum_named_1.async_debug().await),
        "Variant1 { string: \"test\", integer: 42, rw_lock: [\"string0\", \"string1\"], mutex: [0, 1], mutex_u128: 999, mutex_u64: 999 }",
    );

    let enum_named_v2 = EnumNamed::Variant2 {
        v2_string: "test".into(),
        v2_rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        v2_mutex: Mutex::from(vec![0, 1]),
        v2_mutex_u128: Mutex::from(999),
        v2_mutex_u64: Mutex::from(999),
    };

    assert_eq!(
        format!("{:?}", enum_named_2.async_debug().await),
        "Variant2 { v2_string: \"test\", v2_rw_lock: [\"string0\", \"string1\"], v2_mutex: [0, 1], v2_mutex_u128: 999, mutex_u64: 999 }",
    );
}
