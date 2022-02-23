#![rustfmt::skip]

impl AsyncDebug for Input {}
#[automatically_derived]
impl Input {
    async fn async_debug(&self) -> async_debug_input::Input<&String> {
        async_debug_input::Input {
            test: &self.test,
        }
    }
}
mod async_debug_input {
    use super::*;
    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    pub struct Input<T_AsyncDebug_test> {
        pub(super) test: T_AsyncDebug_test,
    }
}
