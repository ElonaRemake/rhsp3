use anymap::AnyMap;
use rhsp3_internal_abi::hsp3struct::{HSP3TYPEINFO, HSPCTX, HSPEXINFO};
use rhsp3_internal_common::{
    bail_lit,
    ctx::{HspContext, HspExtData},
    ensure_lit,
    errors::*,
};
use std::{
    cell::{RefCell, RefMut},
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
pub unsafe fn set_active_ctx(ctx: *mut HSP3TYPEINFO) -> Result<()> {
    let ctx = &*ctx;

    let box_ctx = Box::leak(Box::new(RefCell::new(DylibContext {
        target_thread: current().id(),
        context: DylibHspContext::from_ptr(ctx),
        map: AnyMap::new(),
    })));

    let old_ctx =
        ACTIVE_CTX.compare_exchange(null_mut(), box_ctx, Ordering::SeqCst, Ordering::SeqCst);
    if old_ctx.is_err() {
        std::mem::drop(Box::from_raw(box_ctx));
        bail_lit!("`set_active_ctx` called twice?");
    }
    crate::dylib_hspctx::ctx_fns::set_hspctx_ptr(ctx)?;
    Ok(())
}
