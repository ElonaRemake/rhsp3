use anymap::AnyMap;
use parking_lot::{Mutex, MutexGuard};
use rhsp3_internal_abi::hsp3struct::{HSP3TYPEINFO, HSPCTX, HSPEXINFO};
use rhsp3_internal_common::{
    bail_lit,
    ctx::{HspContext, HspExtData},
    ensure_lit,
    errors::*,
};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
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
    fn new() -> Self {
        HspExtDataGuard { data: Rc::new(RefCell::new(T::init())) }
    }
    pub fn borrow_mut(&self) -> RefMut<T> {
        self.data.borrow_mut()
    }
}

pub struct StoredHspContext {
    target_thread: ThreadId,
    pub context: DylibHspContext,
    map: AnyMap,
}
impl StoredHspContext {
    pub fn get_ext_data<T: HspExtData>(&mut self) -> HspExtDataGuard<T> {
        if !self.map.contains::<HspExtDataGuard<T>>() {
            self.map.insert(HspExtDataGuard::<T>::new());
        }
        self.map.get::<HspExtDataGuard<T>>().unwrap().clone()
    }
}
unsafe impl Send for StoredHspContext {}
unsafe impl Sync for StoredHspContext {}
static ACTIVE_CTX: Mutex<Option<StoredHspContext>> = Mutex::new(None);

#[inline(never)]
fn lock_ctx() -> Result<MutexGuard<'static, Option<StoredHspContext>>> {
    match ACTIVE_CTX.try_lock() {
        Some(x) => Ok(x),
        None => bail_lit!("`HspContext` is already locked?"),
    }
}

#[inline(never)]
fn check_ctx(ctx: &mut Option<StoredHspContext>) -> Result<&mut StoredHspContext> {
    let ctx = match ctx {
        Some(x) => x,
        None => bail_lit!("No `HspContext` is loaded."),
    };
    ensure_lit!(
        ctx.target_thread == current().id(),
        "`HspContext` was created for a different thread."
    );
    Ok(ctx)
}

pub fn with_active_ctx<R>(callback: impl FnOnce(&mut StoredHspContext) -> Result<R>) -> Result<R> {
    let mut lock = lock_ctx()?;
    let ctx = check_ctx(&mut *lock)?;
    callback(ctx)
}

#[inline(never)]
pub unsafe fn set_active_ctx(ctx: *mut HSP3TYPEINFO) -> Result<()> {
    let ctx = &*ctx;

    let mut lock = lock_ctx()?;
    ensure_lit!(lock.is_none(), "`HspContext` is already set for this binary.");
    *lock = Some(StoredHspContext {
        target_thread: current().id(),
        context: DylibHspContext::from_ptr(ctx),
        map: AnyMap::new(),
    });
    crate::dylib_hspctx::ctx_fns::set_hspctx_ptr(ctx)?;
    Ok(())
}
