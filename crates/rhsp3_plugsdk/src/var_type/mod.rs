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
use crate::dylib_hspctx::VarTypeOwnedCdylib;

pub unsafe trait VarTypeSealed {
    type HspParam;
    const PARAM_NAME: HspParamType;
}
pub unsafe trait VarTypeOwnedSealed: VarTypeSealed + Sized {
    unsafe fn from_hsp_param(param: Self::HspParam) -> Result<Self>;
    unsafe fn to_hsp_param(self) -> Result<Self::HspParam>;

    type VarSetParam;
    type VarReturn<'a>: Sized;
    const VAR_PARAM_NAME: HspParamType;
}

/// Represents a type that can be used as a parameter in a HSP plugin function.
pub unsafe trait VarType: VarTypeSealed {}

/// Represents a type that can be used as a owned parameter in a HSP plugin function.
pub unsafe trait VarTypeOwned:
    VarType + VarTypeOwnedSealed + Sized + VarTypeOwnedCdylib
{
}

mod impl_numeric;
