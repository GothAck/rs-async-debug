use async_debug::AsyncDebug;

#[derive(Debug, AsyncDebug)]
union Mixed {
    integer: u64,
    float: f64,
}

#[tokio::main]
async fn main() {
    let mixed = Mixed {
        float: 3.141,
    };

    assert_eq!(
        format!("{:?}", mixed.async_debug().await),
        "MixedDebug { string: \"test\", integer: 42, rw_lock: [\"string0\", \"string1\"], mutex: [0, 1], mutex_u128: 999 }",
    );
}
