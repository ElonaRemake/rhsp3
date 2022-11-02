//! A crate implementing common interfaces that allow the various crates under rhsp3 to
//! interoperate.

#[doc(inline)]
pub use rhsp3_internal_common::{
    bail, bail_lit,
    ctx::HspContext,
    ensure, ensure_lit,
    errors::{Error, ErrorWrapper, Result},
    hsp_errors::ErrorCode,
    plugin::HspPlugin,
};
