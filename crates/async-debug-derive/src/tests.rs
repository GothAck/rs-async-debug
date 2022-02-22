use std::io::Write;

extern crate goldenfile;
use goldenfile::Mint;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::quote;

use crate::async_debug_impl;

fn fmt(ts: TokenStream) -> String {
    let f: syn::File = syn::parse2(ts).unwrap();
    format!(
        "\
            #![rustfmt::skip]\n\
            \n\
            {}\
        ",
        unparse(&f),
    )
}

#[test]
fn test_attr_async_call() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("test_attr_async_call.rs").unwrap();

    let input = quote! {
        #[derive(AsyncDebug)]
        struct Input {
            #[async_debug(async_call = RwLock::lock)]
            test: RwLock,
        }
    };

    let output = async_debug_impl(input).unwrap();

    file.write_all(fmt(output).as_bytes()).unwrap();
}

#[test]
fn test_attr_clone() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("test_attr_clone.rs").unwrap();

    let input = quote! {
        #[derive(AsyncDebug)]
        struct Input {
            #[async_debug(clone)]
            test: RwLock,
        }
    };

    let output = async_debug_impl(input).unwrap();

    file.write_all(fmt(output).as_bytes()).unwrap();
}

#[test]
fn test_attr_copy() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("test_attr_copy.rs").unwrap();

    let input = quote! {
        #[derive(AsyncDebug)]
        struct Input {
            #[async_debug(copy)]
            test: RwLock,
        }
    };

    let output = async_debug_impl(input).unwrap();

    file.write_all(fmt(output).as_bytes()).unwrap();
}

#[test]
fn test_attr_ty() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("test_attr_ty.rs").unwrap();

    let input = quote! {
        #[derive(AsyncDebug)]
        struct Input {
            #[async_debug(ty = TestType)]
            test: RwLock,
        }
    };

    let output = async_debug_impl(input).unwrap();

    file.write_all(fmt(output).as_bytes()).unwrap();
}

#[test]
fn test_no_or_empty_attrs() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("test_no_attrs.rs").unwrap();

    let input = quote! {
        #[derive(AsyncDebug)]
        struct Input {
            test: String,
            #[async_debug()]
            empty: u64,
        }
    };

    let output = async_debug_impl(input).unwrap();

    file.write_all(fmt(output).as_bytes()).unwrap();
}
