use tokio::sync::{Mutex, RwLock};

mod module {
    use async_debug::AsyncDebug;
    use tokio::sync::{Mutex, RwLock};

    #[derive(Debug, AsyncDebug)]
    pub(super) struct StructUnnamed(
        pub String,
        pub u64,
        #[async_debug(async_call = RwLock::read, clone, ty = Vec<String>)] pub RwLock<Vec<String>>,
        #[async_debug(async_call = Mutex::lock, clone, ty = Vec<u64>)] pub Mutex<Vec<u64>>,
        #[async_debug(async_call = Mutex::lock, copy, ty = u128)] pub Mutex<u128>,
    );

    #[derive(Debug, AsyncDebug)]
    pub(super) struct StructNamed {
        pub string: String,
        pub integer: u64,
        #[async_debug(async_call = RwLock::read, clone, ty = Vec<String>)]
        pub rw_lock: RwLock<Vec<String>>,
        #[async_debug(async_call = Mutex::lock, clone, ty = Vec<u64>)]
        pub mutex: Mutex<Vec<u64>>,
        #[async_debug(async_call = Mutex::lock, copy, ty = u128)]
        pub mutex_u128: Mutex<u128>,
    }

    #[derive(Debug, AsyncDebug)]
    pub(super) enum EnumNamed {
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

    #[derive(Debug, AsyncDebug)]
    pub(super) enum EnumUnnamed {
        Variant1(
            String,
            u64,
            #[async_debug(async_call = RwLock::read, clone, ty = Vec<String>)] RwLock<Vec<String>>,
            #[async_debug(async_call = Mutex::lock, clone, ty = Vec<u64>)] Mutex<Vec<u64>>,
            #[async_debug(async_call = Mutex::lock, copy, ty = u128)] Mutex<u128>,
        ),
        Variant2(
            String,
            #[async_debug(async_call = RwLock::read, clone, ty = Vec<String>)] RwLock<Vec<String>>,
            #[async_debug(async_call = Mutex::lock, clone, ty = Vec<u64>)] Mutex<Vec<u64>>,
            #[async_debug(async_call = Mutex::lock, copy, ty = u128)] Mutex<u128>,
        ),
    }
}

#[tokio::main]
async fn main() {
    let struct_unnamed = module::StructUnnamed(
        "test".into(),
        42,
        RwLock::from(vec!["string0".into(), "string1".into()]),
        Mutex::from(vec![0, 1]),
        Mutex::from(999),
    );

    assert_eq!(
        format!("{:?}", struct_unnamed.async_debug().await),
        "StructUnnamed(\"test\", 42, [\"string0\", \"string1\"], [0, 1], 999)",
    );

    let struct_named = module::StructNamed {
        string: "test".into(),
        integer: 42,
        rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        mutex: Mutex::from(vec![0, 1]),
        mutex_u128: Mutex::from(999),
    };

    assert_eq!(
        format!("{:?}", struct_named.async_debug().await),
        "StructNamed { string: \"test\", integer: 42, rw_lock: [\"string0\", \"string1\"], mutex: [0, 1], mutex_u128: 999 }",
    );

    let enum_named_v1 = module::EnumNamed::Variant1 {
        string: "test".into(),
        integer: 42,
        rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        mutex: Mutex::from(vec![0, 1]),
        mutex_u128: Mutex::from(999),
    };

    assert_eq!(
        format!("{:?}", enum_named_v1.async_debug().await),
        "Variant1 { string: \"test\", integer: 42, rw_lock: [\"string0\", \"string1\"], mutex: [0, 1], mutex_u128: 999 }",
    );

    let enum_named_v2 = module::EnumNamed::Variant2 {
        v2_string: "test".into(),
        v2_rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        v2_mutex: Mutex::from(vec![0, 1]),
        v2_mutex_u128: Mutex::from(999),
    };

    assert_eq!(
        format!("{:?}", enum_named_v2.async_debug().await),
        "Variant2 { v2_string: \"test\", v2_rw_lock: [\"string0\", \"string1\"], v2_mutex: [0, 1], v2_mutex_u128: 999 }",
    );

    let enum_unnamed_v1 = module::EnumUnnamed::Variant1(
        "test".into(),
        42,
        RwLock::from(vec!["string0".into(), "string1".into()]),
        Mutex::from(vec![0, 1]),
        Mutex::from(999),
    );

    assert_eq!(
        format!("{:?}", enum_unnamed_v1.async_debug().await),
        "Variant1(\"test\", 42, [\"string0\", \"string1\"], [0, 1], 999)",
    );

    let enum_unnamed_v2 = module::EnumUnnamed::Variant2(
        "test".into(),
        RwLock::from(vec!["string0".into(), "string1".into()]),
        Mutex::from(vec![0, 1]),
        Mutex::from(999),
    );

    assert_eq!(
        format!("{:?}", enum_unnamed_v2.async_debug().await),
        "Variant2(\"test\", [\"string0\", \"string1\"], [0, 1], 999)",
    );
}
