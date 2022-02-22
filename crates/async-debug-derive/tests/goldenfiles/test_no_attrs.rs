#![rustfmt::skip]

impl AsyncDebug for Input {}
#[automatically_derived]
impl Input {
    async fn async_debug(&self) -> async_debug_input::Input<&String, &u64> {
        async_debug_input::Input {
            test: &self.test,
            empty: &self.empty,
        }
    }
}
mod async_debug_input {
    use super::*;
    #[derive(Debug)]
    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    pub struct Input<T_AsyncDebug_test, T_AsyncDebug_empty> {
        pub(super) test: T_AsyncDebug_test,
        pub(super) empty: T_AsyncDebug_empty,
    }
}
