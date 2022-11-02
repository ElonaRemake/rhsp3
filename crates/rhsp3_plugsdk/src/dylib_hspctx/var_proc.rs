use rhsp3_internal_abi::hsp3struct::{HspVarProc, PVal, PDAT};
use rhsp3_internal_common::{ctx::HspType, errors::*};

macro_rules! check_ptr_exists {
    ($self:expr, $name:ident) => {
        match (*$self.procs).$name {
            Some(x) => x,
            None => {
                return Err(error_new(ErrorKind::FunctionNotFoundForType(
                    $self.ty,
                    stringify!($name),
                )))
            }
        }
    };
}

pub struct HspVarProcWrapper {
    ty: HspType,
    procs: *const HspVarProc,
}
impl HspVarProcWrapper {
    pub(super) unsafe fn new(ty: HspType, procs: *const HspVarProc) -> HspVarProcWrapper {
        HspVarProcWrapper { ty, procs }
    }

    pub unsafe fn get_ptr(&self, val: *const PVal) -> Result<*const PDAT> {
        Ok(check_ptr_exists!(self, GetPtr)(val as *mut _) as *const _)
    }
    pub unsafe fn get_ptr_mut(&self, val: *mut PVal) -> Result<*mut PDAT> {
        Ok(check_ptr_exists!(self, GetPtr)(val))
    }
}
