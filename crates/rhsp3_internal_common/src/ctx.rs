use crate::errors::*;
use rhsp3_internal_abi::hsp3struct::*;
use std::ffi::c_short;

/// The type of an HSP variable.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum HspType {
    /// An undefined HSP variable.
    Undefined,
    /// A HSP variable holding a label reference.
    Label,
    /// A HSP variable holding a string reference.
    String,
    /// A HSP variable holding a double reference.
    Double,
    /// A HSP variable holding an int reference.
    Int,
    /// A HSP variable holding a module reference (useful for creating structs).
    Struct,
    /// ???
    ComStruct,
}

pub fn from_hsp_type(ty: c_short) -> Result<HspType> {
    match ty as u32 {
        HSPVAR_FLAG_NONE => Ok(HspType::Undefined),
        HSPVAR_FLAG_LABEL => Ok(HspType::Label),
        HSPVAR_FLAG_STR => Ok(HspType::String),
        HSPVAR_FLAG_DOUBLE => Ok(HspType::Double),
        HSPVAR_FLAG_INT => Ok(HspType::Int),
        HSPVAR_FLAG_STRUCT => Ok(HspType::Struct),
        HSPVAR_FLAG_COMSTRUCT => Ok(HspType::ComStruct),
        _ => Err(error_new_str("`to_hsp_type` passed invalid value?")),
    }
}
pub fn to_hsp_type(ty: HspType) -> c_short {
    match ty {
        HspType::Undefined => HSPVAR_FLAG_NONE as c_short,
        HspType::Label => HSPVAR_FLAG_LABEL as c_short,
        HspType::String => HSPVAR_FLAG_STR as c_short,
        HspType::Double => HSPVAR_FLAG_DOUBLE as c_short,
        HspType::Int => HSPVAR_FLAG_INT as c_short,
        HspType::Struct => HSPVAR_FLAG_STRUCT as c_short,
        HspType::ComStruct => HSPVAR_FLAG_COMSTRUCT as c_short,
    }
}
pub fn hsp_ty_check(data: &PVal, ty: HspType) -> Result<()> {
    if data.flag != to_hsp_type(ty) {
        Err(error_new(ErrorKind::HspTypeError(ty, from_hsp_type(data.flag)?)).with_backtrace())
    } else {
        Ok(())
    }
}

pub trait HspContext {}

pub trait HspExtData: Sized + 'static {
    fn init() -> Result<Self>;
}
