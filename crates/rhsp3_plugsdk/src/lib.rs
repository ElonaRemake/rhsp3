#![feature(panic_always_abort)]
#![cfg_attr(feature = "cdylib", feature(c_unwind))]
#![forbid(unused_must_use)]

mod var_type;

#[cfg(feature = "cdylib")]
mod dylib_hspctx;

#[doc(inline)]
pub use rhsp3_internal_macros::hsp_export;
pub use var_type::{Var, VarType, VarTypeOwned};

pub mod codegen;

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

    pub mod registration {
        pub trait Registration<const ID: usize> {
            fn run_chain(&self);
        }

        pub struct DerefRamp<'a, const ID: usize, T>(pub &'a T);
        impl<'a, const ID: usize, T> Copy for DerefRamp<'a, ID, T> {}
        impl<'a, const ID: usize, T> Clone for DerefRamp<'a, ID, T> {
            fn clone(&self) -> Self {
                *self
            }
        }

        pub trait DerefRampChainA {
            fn run_chain(self);
        }
        impl<'a, const ID: usize, T> DerefRampChainA for &DerefRamp<'a, ID, T>
        where T: Registration<ID>
        {
            #[inline(always)]
            fn run_chain(self) {
                self.0.run_chain()
            }
        }

        pub trait DerefRampChainB {
            fn run_chain(self);
        }
        impl<'a, const ID: usize, T> DerefRampChainB for DerefRamp<'a, ID, T> {
            #[inline(always)]
            fn run_chain(self) {}
        }
    }
}

/// Creates the types used by `rhsp3_plugsdk` to support each plugin.
///
/// This may only be used in the root of a crate for technical reasons, and must be present in the
/// crate root in order to use the `#[hsp_export]` attribute.
#[macro_export]
macro_rules! hpi_root {
    ($vis:vis $out_type:ident $(,)?) => {
        $crate::hpi_root!($vis $out_type, []);
    };
    ($vis:vis $out_type:ident, [ $($submodules:path),* $(,)? ] $(,)?) => {
        /// The type used to represent this HSP plugin, for direct linking into `rhsp3` engines or
        /// to generate `.as` headers for use with the official HSP3 implementation.
        $vis enum $out_type {}

        /// The module used by rhsp3's codegen for this crate.
        #[deprecated = "This module is for internal use by rhsp3! It is not public API for either \
                        the crate it is defined in or its users."]
        #[doc(hidden)]
        #[allow(deprecated)]
        mod __rhsp3_root {
            use crate::$out_type;
            use $crate::__macro_export::{
                reexport::{
                    rhsp3_internal_common::plugin::*,
                    std::{cell::RefCell, prelude::rust_2021::*},
                },
                registration::*,
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

            pub struct GatherPrototypes<'a>(
                pub std::cell::RefCell<&'a mut Vec<HspFunctionPrototype>>,
            );

            impl HspPluginSealed for $out_type {
                fn get_prototypes() -> Vec<HspFunctionPrototype> {
                    let mut prototypes = Vec::new();
                    let event = GatherPrototypes(std::cell::RefCell::new(&mut prototypes));

                    let helper = DerefRamp::<0, _>(&event);
                    (&helper).run_chain();

                    $(prototypes.extend(<$submodules as HspPluginSealed>::get_prototypes());)*

                    prototypes
                }

                fn dylib_init_link_name() -> &'static str {
                    "__rhsp3_plugsdk__dylib_init"
                }
            }
            impl HspPlugin for $out_type {}
        }
    };
}
