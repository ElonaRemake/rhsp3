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
        }
        unsafe impl VarTypeOwnedSealed for $ty {
            unsafe fn from_hsp_param(param: Self::HspParam) -> Result<Self> {
                try_conv!(param, $ty)
            }
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
