#![forbid(unused_must_use)]

mod var_type;

#[cfg(feature = "cdylib")]
mod dylib_hspctx;

#[doc(inline)]
pub use rhsp3_internal_macros::hsp_export;
pub use var_type::{Var, VarType, VarTypeOwned};

/// Not public API.
#[doc(hidden)]
#[deprecated = "This is only used for macro exports, and is not public API."]
pub mod __macro_export {
    pub use crate::var_type::*;
    pub use rhsp3_internal_common::errors::Result;

    #[cfg(feature = "cdylib")]
    pub use crate::dylib_hspctx::macro_export as dylib;
}
