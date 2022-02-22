#![rustfmt::skip]

impl AsyncDebug for Input {}
#[automatically_derived]
impl Input {
    async fn async_debug(&self) -> async_debug_input::InputAsyncDebug<&RwLock> {
        async_debug_input::InputAsyncDebug {
            test: &RwLock::lock(&self.test).await,
        }
    }
}
mod async_debug_input {
    use super::*;
    #[derive(Debug)]
    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    pub struct InputAsyncDebug<T_AsyncDebug_test> {
        pub(super) test: T_AsyncDebug_test,
    }
}
