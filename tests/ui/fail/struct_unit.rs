use async_debug::AsyncDebug;

#[derive(Debug, AsyncDebug)]
struct Mixed;

#[tokio::main]
async fn main() {
    let mixed = Mixed;

    assert_eq!(
        format!("{:?}", mixed.async_debug().await),
        "Mixed { string: \"test\", integer: 42, rw_lock: [\"string0\", \"string1\"], mutex: [0, 1], mutex_u128: 999 }",
    );
}
