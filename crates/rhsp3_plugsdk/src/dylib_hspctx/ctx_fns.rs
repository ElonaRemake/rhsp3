use crate::dylib_hspctx::var_proc::HspVarProcWrapper;
use rhsp3_internal_abi::hsp3struct::{HspVarProc, HSP3TYPEINFO, HSPEXINFO};
use rhsp3_internal_common::{
    bail_lit,
    ctx::{to_hsp_type, HspType},
    ensure_lit,
    errors::*,
};
use std::{
    ffi::*,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, AtomicU16, Ordering},
};

static HSP_EXINFO: AtomicPtr<HSPEXINFO> = AtomicPtr::new(null_mut());
static HSP_VERSION: AtomicU16 = AtomicU16::new(0);

pub(super) fn set_hspctx_ptr(ptr: &HSP3TYPEINFO) -> Result<()> {
    HSP_EXINFO.store(ptr.hspexinfo, Ordering::Relaxed);
    let version = unsafe { (*ptr.hspexinfo).ver };
    ensure_lit!(version >= 0x3000, "HSP version is under 3.0?");
    HSP_VERSION.store(version as u16, Ordering::Relaxed);
    Ok(())
}
pub(super) fn check_version(proc_name: &'static str, min_version: u16) -> Result<()> {
    let ver = HSP_VERSION.load(Ordering::Relaxed);
    if ver < min_version {
        Err(error_new(ErrorKind::HspVersionError(proc_name, min_version, ver)))
    } else {
        Ok(())
    }
}

fn get_exinfo() -> Result<*const HSPEXINFO> {
    let ptr = HSP_EXINFO.load(Ordering::Relaxed);
    ensure_lit!(!ptr.is_null(), "No HSPEXINFO is loaded?");
    Ok(ptr as *const _)
}

macro_rules! fn_wrapper_exinfo {
    (
        $min_version:expr, $vis:vis $proc_name:ident, $ptr_name:ident,
        ($($param:ident : $param_ty:ty),* $(,)?) -> $return_ty:ty
    ) => {
        $vis unsafe fn $proc_name($($param: $param_ty,)*) -> Result<$return_ty> {
            check_version(stringify!($ptr_name), $min_version)?;

            let ptr = &*get_exinfo()?;
            let func = ptr.$ptr_name;
            match func {
                Some(v) => Ok(v($($param,)*)),
                None => bail_lit!(concat!("HSP function `", stringify!($ptr_name), "` is null?")),
            }
        }
    };
}

fn_wrapper_exinfo! {
    0x3000, get_var_proc_internal, HspFunc_getproc,
    (ty: c_int) -> *mut HspVarProc
}
pub unsafe fn get_var_proc(ty: HspType) -> Result<HspVarProcWrapper> {
    let proc = get_var_proc_internal(to_hsp_type(ty) as i32)?;
    Ok(HspVarProcWrapper::new(ty, proc))
}
