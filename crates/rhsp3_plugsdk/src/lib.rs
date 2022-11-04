#![feature(panic_always_abort)]
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

    pub mod reexport {
        pub use rhsp3_internal_abi;
        pub use rhsp3_internal_common;
        pub use std;
    }
}

#[cfg(feature = "cdylib")]
#[macro_export]
#[doc(hidden)]
macro_rules! __rhsp3_plugsdk__dylib {
    ($out_type:ident) => {
        use $crate::__macro_export::reexport::rhsp3_internal_abi::hsp3struct::HSP3TYPEINFO;

        unsafe fn init_impl(type_info: *mut HSP3TYPEINFO) -> i32 {
            $crate::__macro_export::dylib::check_error(|| {
                $crate::__macro_export::dylib::set_active_ctx(type_info)?;
                Ok(0)
            })
        }

        #[cfg(windows)]
        #[export_name = "__rhsp3_plugsdk__dylib_init"]
        pub unsafe extern "stdcall-unwind" fn init(type_info: *mut HSP3TYPEINFO) -> i32 {
            init_impl(type_info)
        }

        #[cfg(not(windows))]
        #[export_name = "__rhsp3_plugsdk__dylib_init"]
        pub unsafe extern "C-unwind" fn init(type_info: *mut HSP3TYPEINFO) -> i32 {
            init_impl(type_info)
        }
    };
}

#[cfg(not(feature = "cdylib"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __rhsp3_plugsdk__dylib {
    ($($rest:tt)*) => {};
}

/// Creates the types used by `rhsp3_plugsdk` to support each plugin.
///
/// This may only be used in the root of a crate for technical reasons, and must be present in the
/// crate root in order to use the `#[hsp_export]` attribute.
#[macro_export]
macro_rules! hpi_root {
    ($out_type:ident $(,)?) => {
        /// The type used to represent this HSP plugin, for direct linking into `rhsp3` engines or
        /// to generate `.as` headers for use with the official HSP3 implementation.
        pub enum $out_type {}

        /// The module used by rhsp3's codegen for this crate.
        #[deprecated = "This module is for internal use by rhsp3! It is not public API for either \
                        the crate it is defined in or its users."]
        #[doc(hidden)]
        #[allow(deprecated)]
        mod __rhsp3_root {
            use crate::$out_type;
            use $crate::__macro_export::{
                reexport::{rhsp3_internal_common::plugin::*, std::prelude::rust_2021::*},
                *,
            };

            #[allow(unused)]
            mod check_path {
                struct CheckPathStruct;
                fn check_is_macro_called_in_crate_root() {
                    let val = crate::__rhsp3_root::check_path::CheckPathStruct;
                    ::std::mem::drop(val);
                }
            }

            $crate::__rhsp3_plugsdk__dylib!($out_type);

            impl HspPluginSealed for $out_type {}
            impl HspPlugin for $out_type {}
        }
    };
}
