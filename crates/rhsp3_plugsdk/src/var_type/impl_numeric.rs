use crate::var_type::{VarType, VarTypeOwned, VarTypeOwnedSealed, VarTypeSealed};
use rhsp3_internal_common::{errors::*, plugin::HspParamType};

macro_rules! try_conv {
    ($val:expr, $into:ty) => {
        match <$into>::try_from($val) {
            Ok(v) => Ok(v),
            Err(_) => Err(error_new(ErrorKind::IntegerNotInRange(stringify!($into), $val as i64))),
        }
    };
}

macro_rules! integral_type {
    ($ty:ty) => {
        unsafe impl VarTypeSealed for $ty {
            type HspParam = i32;
            const PARAM_NAME: HspParamType = HspParamType::Int;

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
        unsafe impl VarTypeOwnedSealed for $ty {
            #[inline(always)]
            unsafe fn from_hsp_param(param: Self::HspParam) -> Result<Self> {
                try_conv!(param, $ty)
            }
            #[inline(always)]
            unsafe fn to_hsp_param(self) -> Result<Self::HspParam> {
                try_conv!(self, i32)
            }

            type VarSetParam = $ty;
            type VarReturn<'a> = $ty;
            const VAR_PARAM_NAME: HspParamType = HspParamType::PVal;
        }
        unsafe impl VarType for $ty {}
        unsafe impl VarTypeOwned for $ty {}
    };
}

integral_type!(u8);
integral_type!(i8);
integral_type!(u16);
integral_type!(i16);
integral_type!(u32);
integral_type!(i32);
integral_type!(u64);
integral_type!(i64);
integral_type!(usize);
integral_type!(isize);
