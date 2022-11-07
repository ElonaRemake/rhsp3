use crate::var_type::{VarBuffer, VarBufferSealed};
use log::warn;
use rhsp3_internal_abi::hsp3struct::PVal;
use rhsp3_internal_common::{
    bail_code,
    ctx::{from_hsp_type, HspType},
    ensure_lit,
    errors::*,
};
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct DylibVarBuffer {
    data: *mut u8,
    length: usize,
}
impl DylibVarBuffer {
    pub unsafe fn new(ptr: *mut PVal) -> Result<Self> {
        let ptr = &*ptr;

        match from_hsp_type(ptr.flag)? {
            HspType::String | HspType::Double | HspType::Int => {}
            _ => bail_code!(TypeMismatch),
        }
        if ptr.offset != 0 {
            warn!(target: "rhsp3_plugsdk", "Array offsets passed to `VarBuffer`s are ignored!");
        }
        ensure_lit!(ptr.size >= 0);

        Ok(DylibVarBuffer { data: ptr.pt as *mut u8, length: ptr.size as usize })
    }
}
impl Deref for DylibVarBuffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.data, self.length) }
    }
}
impl DerefMut for DylibVarBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.length) }
    }
}
impl VarBufferSealed for DylibVarBuffer {}
impl VarBuffer for DylibVarBuffer {
    fn deref(&self) -> &[u8] {
        <Self as Deref>::deref(self)
    }
    fn deref_mut(&mut self) -> &mut [u8] {
        <Self as DerefMut>::deref_mut(self)
    }
}
