use parking_lot::{Mutex, MutexGuard};
use rhsp3_internal_abi::hsp3struct::{HSP3TYPEINFO, HSPCTX, HSPEXINFO};
use rhsp3_internal_common::{bail_lit, ctx::HspContext, ensure_lit, errors::*};
use std::thread::{current, ThreadId};

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

struct StoredHspContext {
    target_thread: ThreadId,
    context: DylibHspContext,
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
fn check_ctx(ctx: &mut Option<StoredHspContext>) -> Result<&mut DylibHspContext> {
    let ctx = match ctx {
        Some(x) => x,
        None => bail_lit!("No `HspContext` is loaded."),
    };
    ensure_lit!(
        ctx.target_thread == current().id(),
        "`HspContext` was created for a different thread."
    );
    Ok(&mut ctx.context)
}

pub fn with_active_ctx<R>(callback: impl FnOnce(&mut DylibHspContext) -> Result<R>) -> Result<R> {
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
    });
    crate::dylib_hspctx::ctx_fns::set_hspctx_ptr(ctx)?;
    Ok(())
}
