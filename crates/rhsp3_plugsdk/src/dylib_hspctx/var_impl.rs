use crate::var_type::{Var, VarTypeOwned};
use rhsp3_internal_common::errors::*;
use std::borrow::Borrow;

pub struct DylibVar<T: VarTypeOwned> {
    ptr: T::HspPointerParam,
}
impl<T: VarTypeOwned> DylibVar<T> {
    pub unsafe fn new(ptr: T::HspPointerParam) -> Self {
        DylibVar { ptr }
    }
}
impl<T: VarTypeOwned> Var<T> for DylibVar<T> {
    fn set(&mut self, value: impl Borrow<T::VarSetParam>) -> Result<()> {
        unsafe { T::set_hsp_pointer(self.ptr, value.borrow()) }
    }
    fn get<'a>(&'a self) -> Result<T::VarReturn<'a>> {
        unsafe { T::get_hsp_pointer(self.ptr) }
    }
}
