use crate::dylib_hspctx::ctx_fns::put_error;
use anymap::AnyMap;
use log::{error, info};
use rhsp3_internal_abi::hsp3struct::{HSP3TYPEINFO, HSPCTX, HSPEXINFO};
use rhsp3_internal_common::{
    bail_lit, ctx::HspContext, ensure_lit, errors::*, hsp_errors::to_hsp_error, plugin::HspExtData,
};
use std::{
    cell::{RefCell, RefMut},
    env,
    ptr::null_mut,
    rc::Rc,
    sync::atomic::{AtomicPtr, Ordering},
    thread::{current, ThreadId},
};

/// The HSP execution context.
#[derive(Debug)]
pub struct DylibHspContext {
    ctx: *mut HSPCTX,
    exinfo: *mut HSPEXINFO,
}
impl DylibHspContext {
    pub(crate) unsafe fn from_ptr(ctx: &HSP3TYPEINFO) -> DylibHspContext {
        DylibHspContext { ctx: ctx.hspctx, exinfo: ctx.hspexinfo }
    }
}
impl HspContext for DylibHspContext {}

pub struct HspExtDataGuard<T: HspExtData> {
    data: Rc<RefCell<T>>,
}
impl<T: HspExtData> Clone for HspExtDataGuard<T> {
    fn clone(&self) -> Self {
        HspExtDataGuard { data: self.data.clone() }
    }
}
impl<T: HspExtData> HspExtDataGuard<T> {
    #[cold]
    #[inline(never)]
    fn new() -> Result<Self> {
        Ok(HspExtDataGuard { data: Rc::new(RefCell::new(T::init()?)) })
    }
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.data.borrow_mut()
    }
}

pub struct DylibContext {
    target_thread: ThreadId,
    pub context: DylibHspContext,
    map: AnyMap,
}
impl DylibContext {
    pub fn get_ext_data<T: HspExtData>(&mut self) -> Result<HspExtDataGuard<T>> {
        if !self.map.contains::<HspExtDataGuard<T>>() {
            self.map.insert(HspExtDataGuard::<T>::new()?);
        }
        Ok(self.map.get::<HspExtDataGuard<T>>().unwrap().clone())
    }
}
unsafe impl Send for DylibContext {}
unsafe impl Sync for DylibContext {}

static ACTIVE_CTX: AtomicPtr<RefCell<DylibContext>> = AtomicPtr::new(null_mut());

pub fn with_active_ctx<R>(callback: impl FnOnce(&mut DylibContext) -> Result<R>) -> Result<R> {
    let ctx = ACTIVE_CTX.load(Ordering::SeqCst);
    ensure_lit!(!ctx.is_null(), "`HspContext is not yet loaded.");
    unsafe {
        let mut borrow = (*ctx).try_borrow_mut()?;
        ensure_lit!(
            borrow.target_thread == current().id(),
            "`HspContext` was created for a different thread."
        );
        callback(&mut *borrow)
    }
}

#[inline(never)]
unsafe fn set_active_ctx(ctx: *mut HSP3TYPEINFO) -> Result<()> {
    let ctx = &*ctx;

    // Build the new DylibContext
    let box_ctx = Box::leak(Box::new(RefCell::new(DylibContext {
        target_thread: current().id(),
        context: DylibHspContext::from_ptr(ctx),
        map: AnyMap::new(),
    })));

    // Store the DylibContext
    let old_ctx =
        ACTIVE_CTX.compare_exchange(null_mut(), box_ctx, Ordering::SeqCst, Ordering::SeqCst);
    if old_ctx.is_err() {
        std::mem::drop(Box::from_raw(box_ctx));
        bail_lit!("`set_active_ctx` called twice?");
    }

    // Setup other initialization steps
    crate::dylib_hspctx::ctx_fns::set_hspctx_ptr(ctx)?;
    #[cfg(not(panic = "abort"))]
    std::panic::always_abort(); // we need this for safety, due to the mixed exception handling.
    #[cfg(feature = "init_log")]
    {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "info");
        }
        rhsp3_internal_common::logger::try_init()?;
        info!(target: "rhsp3_plugsdk", "HPI interface initialized.");
    }

    Ok(())
}

pub unsafe fn check_error(func: impl FnOnce() -> Result<i32>) -> i32 {
    match func() {
        Ok(v) => v,
        Err(e) => {
            if e.backtrace().is_some() {
                error!(target: "rhsp3_plugsdk", "Internal error occurred: {}", e);
                error!(target: "rhsp3_plugsdk", "{:?}", e.backtrace().unwrap());
            }
            match put_error(to_hsp_error(e.error_code())) {
                Ok(_) => unreachable!(),
                Err(e) => panic!("Failed to throw HSP error: {e:?}"),
            }
        }
    }
}

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
