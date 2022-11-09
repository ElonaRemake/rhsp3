use crate::{
    utils::decode_char,
    var_type::{VarType, VarTypeOwnedSealed, VarTypeSealed},
    VarTypeOwned,
};
use rhsp3_internal_common::{errors::*, plugin::HspParamType};
use std::ffi::c_char;

unsafe impl VarTypeSealed for str {
    type HspParam = *const c_char;
    const PARAM_TYPE: HspParamType = HspParamType::Str;

    unsafe fn from_hsp_param_ref<R>(
        param: Self::HspParam,
        callback: impl FnOnce(&Self) -> Result<R>,
    ) -> Result<R> {
        callback(&decode_char(param)?)
    }
    unsafe fn from_hsp_param_mut<R>(
        param: Self::HspParam,
        callback: impl FnOnce(&mut Self) -> Result<R>,
    ) -> Result<R> {
        let mut str = decode_char(param)?.to_string();
        callback(&mut str)
    }
}
unsafe impl VarType for str {}

passthrough_owned!(String, *const c_char, Str);
unsafe impl VarTypeOwnedSealed for String {
    unsafe fn from_hsp_param(param: Self::HspParam) -> Result<Self> {
        Ok(decode_char(param)?.to_string())
    }
    unsafe fn to_hsp_param(self) -> Result<Self::HspParam> {
        unreachable!()
    }

    type VarSetParam = str;
    type VarReturn<'a> = String;
    const VAR_PARAM_TYPE: HspParamType = HspParamType::StrAsHandle;
}
unsafe impl VarType for String {}
unsafe impl VarTypeOwned for String {}
