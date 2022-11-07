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
        passthrough_owned!($ty, i32, Int);
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
            const VAR_PARAM_TYPE: HspParamType = HspParamType::VarAsPVal;
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

macro_rules! floating_type {
    ($ty:ty, $hsp_ty:ident) => {
        passthrough_owned!($ty, $ty, $hsp_ty);
        unsafe impl VarTypeOwnedSealed for $ty {
            #[inline(always)]
            unsafe fn from_hsp_param(param: Self::HspParam) -> Result<Self> {
                Ok(param)
            }
            #[inline(always)]
            unsafe fn to_hsp_param(self) -> Result<Self::HspParam> {
                Ok(self)
            }

            type VarSetParam = $ty;
            type VarReturn<'a> = $ty;
            const VAR_PARAM_TYPE: HspParamType = HspParamType::VarAsPVal;
        }
        unsafe impl VarType for $ty {}
        unsafe impl VarTypeOwned for $ty {}
    };
}
floating_type!(f32, Float);
floating_type!(f64, Double);

passthrough_owned!(bool, <i32 as VarTypeSealed>::HspParam, Int);
unsafe impl VarTypeOwnedSealed for bool {
    #[inline(always)]
    unsafe fn from_hsp_param(param: Self::HspParam) -> Result<Self> {
        match param {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(error_new(ErrorKind::IntegerNotInRange("bool", param as i64))),
        }
    }
    #[inline(always)]
    unsafe fn to_hsp_param(self) -> Result<Self::HspParam> {
        Ok(self as i32)
    }

    type VarSetParam = bool;
    type VarReturn<'a> = bool;
    const VAR_PARAM_TYPE: HspParamType = <i32 as VarTypeOwnedSealed>::VAR_PARAM_TYPE;
}
unsafe impl VarType for bool {}
unsafe impl VarTypeOwned for bool {}
