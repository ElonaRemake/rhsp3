use crate::{
    dylib_hspctx::{
        ctx::{check_error, DylibVarContext},
        var_impl::VarTypeOwnedCdylib,
        with_active_ctx,
    },
    utils::{decode_char, encode_str},
};
use rhsp3_internal_abi::hsp3struct::{PVal, HSPVAR_FLAG_STR};
use rhsp3_internal_common::{ensure_lit, errors::*};
use std::{
    cell::RefCell,
    ffi::{c_char, c_int, c_short, CString},
};

// TODO: Do something fancy here with codegen. :)

pub struct StrCtx {
    data: RefCell<Option<String>>,
    encoded_str: RefCell<Option<CString>>,
}

pub unsafe fn make_strctx(out: *mut c_int, str_ptr: *mut c_char) -> i32 {
    check_error(|| {
        with_active_ctx(|ctx| {
            *out = ctx.get_refs().0.str_ctx.alloc(StrCtx {
                data: RefCell::new(Some(decode_char(str_ptr)?.to_string())),
                encoded_str: RefCell::new(None),
            })?;
            Ok(0)
        })
    })
}
pub unsafe fn strctx_len(handle_id: c_int, out: *mut c_int) -> i32 {
    check_error(|| {
        with_active_ctx(|ctx| {
            let handle = ctx.get_refs().0.str_ctx.get(handle_id)?;
            let encoded = encode_str(&handle.data.borrow_mut().take().unwrap())?;

            let len = encoded.as_bytes_with_nul().len();
            ensure_lit!(len <= i32::MAX as usize, "Output string too long!");
            *out = len as i32;

            *handle.encoded_str.borrow_mut() = Some(encoded);

            Ok(0)
        })
    })
}
pub unsafe fn strctx_output(handle_id: c_int, out: *mut PVal) -> i32 {
    check_error(|| {
        with_active_ctx(|ctx| {
            let handle = ctx.get_refs().0.str_ctx.free(handle_id)?;
            let cstr = handle.encoded_str.borrow();
            let cstr = cstr.as_ref().unwrap();
            let len = cstr.as_bytes_with_nul().len();
            assert_eq!((*out).flag, HSPVAR_FLAG_STR as c_short);
            std::ptr::copy(cstr.as_ptr(), (*out).pt, len);
            Ok(0)
        })
    })
}

unsafe impl VarTypeOwnedCdylib for String {
    type HspPointerParam = c_int;
    unsafe fn early_ty_check(_: &DylibVarContext, _: Self::HspPointerParam) -> Result<()> {
        Ok(())
    }
    unsafe fn set_hsp_pointer(
        ctx: &DylibVarContext,
        ptr: Self::HspPointerParam,
        value: &Self::VarSetParam,
    ) -> Result<()> {
        let str = ctx.str_ctx.get(ptr)?;
        *str.data.borrow_mut() = Some(value.to_string());
        Ok(())
    }
    unsafe fn get_hsp_pointer<'a>(
        ctx: &DylibVarContext,
        ptr: Self::HspPointerParam,
    ) -> Result<Self::VarReturn<'a>> {
        Ok(ctx
            .str_ctx
            .get(ptr)?
            .data
            .borrow()
            .as_ref()
            .unwrap()
            .to_string())
    }
}
