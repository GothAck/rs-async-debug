#![rustfmt::skip]

impl AsyncDebug for Input {}
#[automatically_derived]
impl Input {
    async fn async_debug(&self) -> InputAsyncDebug<&RwLock> {
        InputAsyncDebug {
            test: &*self.test,
        }
    }
}
#[derive(Debug)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[automatically_derived]
struct InputAsyncDebug<T_AsyncDebug_test> {
    test: T_AsyncDebug_test,
}
