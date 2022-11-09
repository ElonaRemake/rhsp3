use crate::dylib_hspctx::{
    ctx::{check_error, set_active_ctx},
    var_impl,
};
use rhsp3_internal_abi::hsp3struct::{PVal, HSP3TYPEINFO};
use std::ffi::{c_char, c_int};

unsafe fn init_impl(type_info: *mut HSP3TYPEINFO) -> i32 {
    check_error(|| {
        set_active_ctx(type_info)?;
        Ok(0)
    })
}

#[cfg(windows)]
#[export_name = "__rhsp3_plugsdk__dylib_init"]
pub unsafe extern "stdcall-unwind" fn init(type_info: *mut HSP3TYPEINFO) -> i32 {
    init_impl(type_info)
}

#[cfg(not(windows))]
#[export_name = "__rhsp3_plugsdk__dylib_init"]
pub unsafe extern "C-unwind" fn init(type_info: *mut HSP3TYPEINFO) -> i32 {
    init_impl(type_info)
}

#[cfg(windows)]
#[export_name = "__rhsp3_plugsdk__dylib_make_strctx"]
pub unsafe extern "stdcall-unwind" fn make_strctx(out: *mut c_int, str_ptr: *mut c_char) -> i32 {
    var_impl::make_strctx(out, str_ptr)
}

#[cfg(not(windows))]
#[export_name = "__rhsp3_plugsdk__dylib_make_strctx"]
pub unsafe extern "C-unwind" fn make_strctx(out: *mut c_int, str_ptr: *mut c_char) -> i32 {
    var_impl::make_strctx(out, str_ptr)
}

#[cfg(windows)]
#[export_name = "__rhsp3_plugsdk__dylib_strctx_len"]
pub unsafe extern "stdcall-unwind" fn strctx_len(handle: c_int, out: *mut c_int) -> i32 {
    var_impl::strctx_len(handle, out)
}

#[cfg(not(windows))]
#[export_name = "__rhsp3_plugsdk__dylib_strctx_len"]
pub unsafe extern "C-unwind" fn strctx_len(handle: c_int, out: *mut c_int) -> i32 {
    var_impl::strctx_len(handle, out)
}

#[cfg(windows)]
#[export_name = "__rhsp3_plugsdk__dylib_strctx_output"]
pub unsafe extern "stdcall-unwind" fn strctx_output(handle: c_int, out: *mut PVal) -> i32 {
    var_impl::strctx_output(handle, out)
}

#[cfg(not(windows))]
#[export_name = "__rhsp3_plugsdk__dylib_strctx_output"]
pub unsafe extern "C-unwind" fn strctx_output(handle: c_int, out: *mut PVal) -> i32 {
    var_impl::strctx_output(handle, out)
}
