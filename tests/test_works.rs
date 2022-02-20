use async_debug::{AsyncDebug};
use tokio::sync::{RwLock, Mutex};

async fn unlock<T: Clone>(mtx: &Mutex<T>) -> T {
    mtx.lock().await.clone()
}

async fn read<T: Clone>(lock: &RwLock<T>) -> T {
    lock.read().await.clone()
}

#[tokio::test]
async fn test_works() {
    #[derive(Debug, AsyncDebug)]
    struct Mixed {
        #[async_debug(clone)]
        string: String,
        integer: u64,
        #[async_debug(parse = read, ty = Vec<String>)]
        rw_lock: RwLock<Vec<String>>,
        #[async_debug(parse = unlock, ty = Vec<u64>)]
        mutex: Mutex<Vec<u64>>,
    }

    let mixed = Mixed {
        string: "test".into(),
        integer: 42,
        rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        mutex: Mutex::from(vec![0, 1]),
    };



    assert_eq!(format!("{:?}", mixed.async_debug().await), "MixedDebug { string: \"test\", integer: 42, rw_lock: [\"string0\", \"string1\"], mutex: [0, 1] }");

    assert!(false);
}
