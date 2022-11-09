use crate::{
    dylib_hspctx::ctx::DylibVarContext,
    var_type::{Var, VarTypeOwned, VarTypeOwnedSealed},
};
use rhsp3_internal_common::errors::*;
use std::{
    borrow::Borrow,
    fmt::{Debug, Formatter},
};

pub unsafe trait VarTypeOwnedCdylib: VarTypeOwnedSealed {
    type HspPointerParam: Debug + Copy + Sized;
    unsafe fn early_ty_check(ctx: &DylibVarContext, ptr: Self::HspPointerParam) -> Result<()>;
    unsafe fn set_hsp_pointer(
        ctx: &DylibVarContext,
        ptr: Self::HspPointerParam,
        value: &Self::VarSetParam,
    ) -> Result<()>;
    unsafe fn get_hsp_pointer<'a>(
        ctx: &DylibVarContext,
        ptr: Self::HspPointerParam,
    ) -> Result<Self::VarReturn<'a>>;
}

pub struct DylibVar<'a, T: VarTypeOwned> {
    ctx: &'a DylibVarContext,
    ptr: T::HspPointerParam,
}
impl<'a, T: VarTypeOwned> DylibVar<'a, T> {
    pub unsafe fn new(ctx: &'a DylibVarContext, ptr: T::HspPointerParam) -> Result<Self> {
        T::early_ty_check(ctx, ptr)?;
        Ok(DylibVar { ctx, ptr })
    }
}
impl<'a, T: VarTypeOwned> Var<T> for DylibVar<'a, T> {
    fn set(&mut self, value: impl Borrow<T::VarSetParam>) -> Result<()> {
        unsafe { T::set_hsp_pointer(self.ctx, self.ptr, value.borrow()) }
    }
    fn get<'b>(&'b mut self) -> Result<T::VarReturn<'b>> {
        unsafe { T::get_hsp_pointer(self.ctx, self.ptr) }
    }
}
impl<'a, T: VarTypeOwned> Debug for DylibVar<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DylibVar").field(&self.ptr).finish()
    }
}

mod impl_numeric;
mod impl_str;

pub use impl_str::{make_strctx, strctx_len, strctx_output, StrCtx};
