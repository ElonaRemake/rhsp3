mod ctx;
mod ctx_fns;
mod var_impl;
mod var_proc;

pub use ctx::{with_active_ctx, DylibHspContext};
pub use var_impl::{DylibVar, VarTypeOwnedCdylib};

pub mod macro_export {
    pub use super::{
        ctx::{check_error, with_active_ctx, DylibContext, DylibHspContext},
        var_impl::DylibVar,
    };
}
