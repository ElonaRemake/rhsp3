use rhsp3_internal_common::{errors::*, plugin::HspParamType};
use std::{borrow::Borrow, fmt::Debug};

/// Represents an variable passed in from HSP code.
pub trait Var<T: VarTypeOwned>: Debug + Sized {
    /// Sets the value of the variable.
    fn set(&mut self, value: impl Borrow<T::VarSetParam>) -> Result<()>;
    /// Gets the value of the variable.
    fn get<'a>(&'a mut self) -> Result<T::VarReturn<'a>>;
}

/// Represents the raw data backing a variable or array.
///
/// The exact details and contents of this type are platform and implementation dependant, but
/// can always be treated as an array and read back into memory without causing memory safety
/// issues. rhsp3 will not create a `VarBuffer` for a variable where this is not possible.
pub trait VarBuffer: Debug + Sized {}

/// Represents the raw data backing a variable or array.
///
/// Due to HSP's extension API, this is neither aware of the type of the underlying data, nor the
/// length of the underlying data. Using this is usually unsound.
///
/// However, unlike [`VarBuffer`] it can be used with variables not yet initialized in HSP, due
/// to the different definition type.
pub trait UnsafeVarBuffer: Debug + Sized {}

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

macro_rules! passthrough_owned {
    ($ty:ty, $param_ty:ty, $hsp_ty:ident $(,)?) => {
        unsafe impl VarTypeSealed for $ty {
            type HspParam = $param_ty;
            const PARAM_TYPE: HspParamType = HspParamType::$hsp_ty;

            unsafe fn from_hsp_param_ref<R>(
                param: Self::HspParam,
                callback: impl FnOnce(&Self) -> Result<R>,
            ) -> Result<R> {
                callback(&Self::from_hsp_param(param)?)
            }

            unsafe fn from_hsp_param_mut<R>(
                param: Self::HspParam,
                callback: impl FnOnce(&mut Self) -> Result<R>,
            ) -> Result<R> {
                callback(&mut Self::from_hsp_param(param)?)
            }

            unsafe fn into_hsp_param_mut<R>(
                self,
                callback: impl FnOnce(&mut Self::HspParam) -> Result<R>,
            ) -> Result<R> {
                callback(&mut Self::to_hsp_param(self)?)
            }
        }
    };
}

mod impl_numeric;
