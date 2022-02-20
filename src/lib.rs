pub use async_debug_derive::AsyncDebug;
pub use async_trait::async_trait;

#[async_trait]
pub trait AsyncDebug<T> {
    async fn async_debug(&self) -> T;
}
