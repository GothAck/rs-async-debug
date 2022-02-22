![Crates.io](https://img.shields.io/crates/v/async-debug)
![Crates.io](https://img.shields.io/crates/l/async-debug)
![docs.rs](https://img.shields.io/docsrs/async-debug)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)
<!-- cargo-sync-readme start -->

# Async Debug
The `async-debug` crate makes it easy to debug structs and enums containing
values that require an async call to render.

For example:
```rust
use tokio::sync::RwLock;

#[derive(Debug)]
struct MyStruct {
    my_value: RwLock<String>
}

let my_struct = MyStruct { my_value: RwLock::from("Hello, world!".to_string()) };
println!("{:?}", my_struct );
```

Prints something like:
```text
MyStruct { my_value: RwLock { mr: 536870911, s: Semaphore { permits: 536870911 }, c: UnsafeCell { .. } } }
```

## Along comes Async Debug
Just derive from `async_debug::AsyncDebug` and add the appropriate attribute!

Add to cargo.toml:
```toml
[dependencies]
async-debug = "0.1.1"
```

```rust
use async_debug::AsyncDebug;
use tokio::sync::RwLock;

#[derive(AsyncDebug)]
struct MyStruct {
    #[async_debug(parse = RwLock::read, clone, ty = String)]
    my_value: RwLock<String>
}

let my_struct = MyStruct { my_value: RwLock::from("Hello, world!".to_string()) };
assert_eq!(
    format!("{:?}", my_struct.async_debug().await),
    "MyStructAsyncDebug { my_value: \"Hello, world!\" }",
);
```

<!-- cargo-sync-readme end -->
