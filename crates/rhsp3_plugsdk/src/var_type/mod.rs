use rhsp3_internal_common::{errors::*, plugin::HspParamType};
use std::{borrow::Borrow, fmt::Debug};

/// Represents an variable passed in from HSP code.
pub trait Var<T: VarTypeOwned>: Debug + Sized {
    /// Sets the value of the variable.
    fn set(&mut self, value: impl Borrow<T::VarSetParam>) -> Result<()>;
    /// Gets the value of the variable.
    fn get<'a>(&'a mut self) -> Result<T::VarReturn<'a>>;
}

#[cfg(not(feature = "cdylib"))]
pub trait VarTypeOwnedCdylib {}

#[cfg(not(feature = "cdylib"))]
impl<T> VarTypeOwnedCdylib for T {}

#[cfg(feature = "cdylib")]
pub use crate::dylib_hspctx::VarTypeOwnedCdylib;

pub unsafe trait VarTypeSealed {
    type HspParam;
    const PARAM_TYPE: HspParamType;

    unsafe fn from_hsp_param_ref<R>(
        param: Self::HspParam,
        callback: impl FnOnce(&Self) -> Result<R>,
    ) -> Result<R>;

    unsafe fn from_hsp_param_mut<R>(
        param: Self::HspParam,
        callback: impl FnOnce(&mut Self) -> Result<R>,
    ) -> Result<R>;

    unsafe fn into_hsp_param_mut<R>(
        self,
        callback: impl FnOnce(&mut Self::HspParam) -> Result<R>,
    ) -> Result<R>;
}
pub unsafe trait VarTypeOwnedSealed: VarTypeSealed + Sized {
    unsafe fn from_hsp_param(param: Self::HspParam) -> Result<Self>;
    unsafe fn to_hsp_param(self) -> Result<Self::HspParam>;

    type VarSetParam;
    type VarReturn<'a>: Sized;
    const VAR_PARAM_TYPE: HspParamType;
}

/// Represents a type that can be used as a parameter in a HSP plugin function.
pub unsafe trait VarType: VarTypeSealed {}

/// Represents a type that can be used as a owned parameter in a HSP plugin function.
pub unsafe trait VarTypeOwned:
    VarType + VarTypeOwnedSealed + Sized + VarTypeOwnedCdylib
{
}

/// Represents a type that can be returned from a HSP plugin function.
pub trait HspReturnTy {
    /// Converts the type into a result.
    fn into_result(self) -> Result<i32>;
}
impl HspReturnTy for () {
    fn into_result(self) -> Result<i32> {
        Ok(0)
    }
}
impl HspReturnTy for Result<()> {
    fn into_result(self) -> Result<i32> {
        self?;
        Ok(0)
    }
}
impl HspReturnTy for Result<i32> {
    fn into_result(self) -> Result<i32> {
        self
    }
}

mod impl_numeric;
