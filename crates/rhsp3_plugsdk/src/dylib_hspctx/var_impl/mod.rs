use crate::var_type::{Var, VarTypeOwned, VarTypeOwnedSealed};
use rhsp3_internal_common::errors::*;
use std::{
    borrow::Borrow,
    fmt::{Debug, Formatter},
};

pub unsafe trait VarTypeOwnedCdylib: VarTypeOwnedSealed {
    type HspPointerParam: Debug + Copy + Sized;
    unsafe fn early_ty_check(ptr: Self::HspPointerParam) -> Result<()>;
    unsafe fn set_hsp_pointer(
        ptr: Self::HspPointerParam,
        value: &Self::VarSetParam,
    ) -> Result<()>;
    unsafe fn get_hsp_pointer<'a>(ptr: Self::HspPointerParam) -> Result<Self::VarReturn<'a>>;
}

pub struct DylibVar<T: VarTypeOwned> {
    ptr: T::HspPointerParam,
}
impl<T: VarTypeOwned> DylibVar<T> {
    pub unsafe fn new(ptr: T::HspPointerParam) -> Result<Self> {
        T::early_ty_check(ptr)?;
        Ok(DylibVar { ptr })
    }
}
impl<T: VarTypeOwned> Var<T> for DylibVar<T> {
    fn set(&mut self, value: impl Borrow<T::VarSetParam>) -> Result<()> {
        unsafe { T::set_hsp_pointer(self.ptr, value.borrow()) }
    }
    fn get<'a>(&'a mut self) -> Result<T::VarReturn<'a>> {
        unsafe { T::get_hsp_pointer(self.ptr) }
    }
}
impl<T: VarTypeOwned> Debug for DylibVar<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DylibVar").field(&self.ptr).finish()
    }
}

mod impl_numeric;
