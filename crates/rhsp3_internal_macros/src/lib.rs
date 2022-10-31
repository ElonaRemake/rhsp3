//! This crate implements the macros used throughout the rhsp3 project.
//!
//! This is not public API and should not be used directly.

mod plugsdk;

use proc_macro::TokenStream;

macro_rules! wrap_attr {
    ($target:path, $attr:expr, $item:expr $(,)?) => {
        match $target($attr.into(), $item.into()) {
            Ok(v) => v.into(),
            Err(e) => e.into_compile_error().into(),
        }
    };
}

#[proc_macro_attribute]
pub fn hsp_export(attr: TokenStream, item: TokenStream) -> TokenStream {
    wrap_attr!(plugsdk::hsp_export, attr, item)
}
