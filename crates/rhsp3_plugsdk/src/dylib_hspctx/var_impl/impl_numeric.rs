use crate::{
    dylib_hspctx::{ctx_fns, var_impl::VarTypeOwnedCdylib},
    var_type::VarTypeOwnedSealed,
};
use rhsp3_internal_abi::hsp3struct::PVal;
use rhsp3_internal_common::{ctx::*, errors::*};
use std::ffi::c_void;

macro_rules! integral_type {
    ($ty:ty) => {
        unsafe impl VarTypeOwnedCdylib for $ty {
            type HspPointerParam = *mut PVal;
            unsafe fn early_ty_check(ptr: Self::HspPointerParam) -> Result<()> {
                hsp_ty_check(&*ptr, HspType::Int)
            }
            unsafe fn set_hsp_pointer(
                ptr: Self::HspPointerParam,
                value: &Self::VarSetParam,
            ) -> Result<()> {
                let raw_val = Self::to_hsp_param(*value)?;
                ctx_fns::set_va(ptr, HspType::Int, &raw_val as *const _ as *const c_void)?;
                Ok(())
            }
            unsafe fn get_hsp_pointer<'a>(
                ptr: Self::HspPointerParam,
            ) -> Result<Self::VarReturn<'a>> {
                let proc = ctx_fns::get_var_proc(HspType::Int)?;
                let ptr = proc.get_ptr(ptr)? as *mut i32;
                Self::from_hsp_param(*ptr)
            }
        }
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
