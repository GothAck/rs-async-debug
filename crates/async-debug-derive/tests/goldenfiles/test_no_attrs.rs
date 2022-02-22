#![rustfmt::skip]

impl AsyncDebug for Input {}
#[automatically_derived]
impl Input {
    async fn async_debug(&self) -> InputAsyncDebug<&String, &u64> {
        InputAsyncDebug {
            test: &self.test,
            empty: &self.empty,
        }
    }
}
#[derive(Debug)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[automatically_derived]
struct InputAsyncDebug<T_AsyncDebug_test, T_AsyncDebug_empty> {
    test: T_AsyncDebug_test,
    empty: T_AsyncDebug_empty,
}
