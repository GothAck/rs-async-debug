use tokio::sync::{Mutex, RwLock};

#[derive(Debug)]
#[allow(dead_code)]
struct Mixed {
    string: String,
    integer: u64,
    rw_lock: RwLock<Vec<String>>,
    mutex: Mutex<Vec<u64>>,
    mutex_u128: Mutex<u128>,
}

fn main() {
    let mixed = Mixed {
        string: "test".into(),
        integer: 42,
        rw_lock: RwLock::from(vec!["string0".into(), "string1".into()]),
        mutex: Mutex::from(vec![0, 1]),
        mutex_u128: Mutex::from(999),
    };

    assert_eq!(
        format!("{:?}", mixed),
        "Mixed { string: \"test\", integer: 42, rw_lock: RwLock { mr: 536870911, s: Semaphore { permits: 536870911 }, c: UnsafeCell { .. } }, mutex: Mutex { data: [0, 1] }, mutex_u128: Mutex { data: 999 } }",
    );
}
