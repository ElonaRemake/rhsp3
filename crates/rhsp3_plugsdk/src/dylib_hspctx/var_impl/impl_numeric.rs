use crate::{
    dylib_hspctx::{ctx::DylibVarContext, ctx_fns, var_impl::VarTypeOwnedCdylib},
    var_type::VarTypeOwnedSealed,
};
use rhsp3_internal_abi::hsp3struct::PVal;
use rhsp3_internal_common::{ctx::*, errors::*};
use std::ffi::c_void;

macro_rules! simple_type {
    ($ty:ty, $hsp_ty:ident) => {
        unsafe impl VarTypeOwnedCdylib for $ty {
            type HspPointerParam = *mut PVal;
            unsafe fn early_ty_check(
                _: &DylibVarContext,
                ptr: Self::HspPointerParam,
            ) -> Result<()> {
                hsp_ty_check(&*ptr, HspType::$hsp_ty)
            }
            unsafe fn set_hsp_pointer(
                _: &DylibVarContext,
                ptr: Self::HspPointerParam,
                value: &Self::VarSetParam,
            ) -> Result<()> {
                let raw_val = Self::to_hsp_param(*value)?;
                ctx_fns::set_va(ptr, HspType::$hsp_ty, &raw_val as *const _ as *const c_void)?;
                Ok(())
            }
            unsafe fn get_hsp_pointer<'a>(
                _: &DylibVarContext,
                ptr: Self::HspPointerParam,
            ) -> Result<Self::VarReturn<'a>> {
                let proc = ctx_fns::get_var_proc(HspType::$hsp_ty)?;
                let ptr = proc.get_ptr(ptr)? as *mut Self::HspParam;
                Self::from_hsp_param(*ptr)
            }
        }
    };
}

simple_type!(u8, Int);
simple_type!(i8, Int);
simple_type!(u16, Int);
simple_type!(i16, Int);
simple_type!(u32, Int);
simple_type!(i32, Int);
simple_type!(u64, Int);
simple_type!(i64, Int);
simple_type!(usize, Int);
simple_type!(isize, Int);

unsafe impl VarTypeOwnedCdylib for f32 {
    type HspPointerParam = *mut PVal;
    unsafe fn early_ty_check(_: &DylibVarContext, ptr: Self::HspPointerParam) -> Result<()> {
        hsp_ty_check(&*ptr, HspType::Double)
    }
    unsafe fn set_hsp_pointer(
        _: &DylibVarContext,
        ptr: Self::HspPointerParam,
        value: &Self::VarSetParam,
    ) -> Result<()> {
        let raw_val = *value as f64;
        ctx_fns::set_va(ptr, HspType::Double, &raw_val as *const _ as *const c_void)?;
        Ok(())
    }
    unsafe fn get_hsp_pointer<'a>(
        _: &DylibVarContext,
        ptr: Self::HspPointerParam,
    ) -> Result<Self::VarReturn<'a>> {
        let proc = ctx_fns::get_var_proc(HspType::Double)?;
        let ptr = proc.get_ptr(ptr)? as *mut f64;
        Ok(*ptr as f32)
    }
}
simple_type!(f64, Double);

simple_type!(bool, Int);
