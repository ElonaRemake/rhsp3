use crate::{dylib_hspctx::get_var_proc, var_type::VarTypeOwnedSealed};
use rhsp3_internal_abi::hsp3struct::PVal;
use rhsp3_internal_common::{ctx::*, errors::*};

pub unsafe trait VarTypeOwnedCdylib: VarTypeOwnedSealed {
    type HspPointerParam: Copy + Sized;
    unsafe fn set_hsp_pointer<'a>(
        ptr: Self::HspPointerParam,
        value: &Self::VarSetParam,
    ) -> Result<()>;
    unsafe fn get_hsp_pointer<'a>(ptr: Self::HspPointerParam) -> Result<Self::VarReturn<'a>>;
}

macro_rules! integral_type {
    ($ty:ty) => {
        unsafe impl VarTypeOwnedCdylib for $ty {
            type HspPointerParam = *mut PVal;
            unsafe fn set_hsp_pointer<'a>(
                ptr: Self::HspPointerParam,
                value: &Self::VarSetParam,
            ) -> Result<()> {
                let var = &mut *ptr;
                hsp_ty_check(var, HspType::Int)?;
                let proc = get_var_proc(HspType::Int)?;
                let ptr = proc.get_ptr_mut(var)? as *mut i32;
                (*ptr) = Self::to_hsp_param(*value)?;
                Ok(())
            }

            unsafe fn get_hsp_pointer<'a>(
                ptr: Self::HspPointerParam,
            ) -> Result<Self::VarReturn<'a>> {
                let var = &*ptr;
                hsp_ty_check(var, HspType::Int)?;
                let proc = get_var_proc(HspType::Int)?;
                let ptr = proc.get_ptr(var)? as *const i32;
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
