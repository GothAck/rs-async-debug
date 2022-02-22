//! # Async Debug
//! The `async-debug` crate makes it easy to debug structs and enums containing
//! values that require an async call to render.
//!
//! For example:
//! ```rust
//! use tokio::sync::RwLock;
//!
//! #[derive(Debug)]
//! struct MyStruct {
//!     my_value: RwLock<String>
//! }
//!
//! # fn main() {
//! let my_struct = MyStruct { my_value: RwLock::from("Hello, world!".to_string()) };
//! println!("{:?}", my_struct );
//! # }
//! ```
//!
//! Prints something like:
//! ```text
//! MyStruct { my_value: RwLock { mr: 536870911, s: Semaphore { permits: 536870911 }, c: UnsafeCell { .. } } }
//! ```
//!
//! ## Along comes Async Debug
//! Just derive from `async_debug::AsyncDebug` and add the appropriate attribute!
//!
//! Add to cargo.toml:
//! ```toml
//! [dependencies]
//! async-debug = "0.1.0"
//! ```
//!
//! ```rust
//! use async_debug::AsyncDebug;
//! use tokio::sync::RwLock;
//!
//! #[derive(AsyncDebug)]
//! struct MyStruct {
//!     #[async_debug(parse = RwLock::read, clone, ty = String)]
//!     my_value: RwLock<String>
//! }
//!
//! # #[tokio::main]
//! # async fn main() {
//! let my_struct = MyStruct { my_value: RwLock::from("Hello, world!".to_string()) };
//! assert_eq!(
//!     format!("{:?}", my_struct.async_debug().await),
//!     "MyStructAsyncDebug { my_value: \"Hello, world!\" }",
//! );
//! # }
//! ```
pub use async_debug_derive::AsyncDebug;

pub trait AsyncDebug {}
