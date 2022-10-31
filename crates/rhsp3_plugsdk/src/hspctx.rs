use parking_lot::Mutex;
use rhsp3_internal_abi::hsp3struct::HSPCTX;
use std::thread::ThreadId;

/// The HSP execution context.
#[derive(Debug)]
pub struct HspContext {
    ptr: *mut HSPCTX,
}
impl HspContext {
    pub(crate) unsafe fn from_ptr(ptr: *mut HSPCTX) -> HspContext {
        HspContext { ptr }
    }
}

struct StoredHspContext {
    target_thread: ThreadId,
    context: HspContext,
}
unsafe impl Send for StoredHspContext {}
unsafe impl Sync for StoredHspContext {}
static ACTIVE_CTX: Mutex<Option<StoredHspContext>> = Mutex::new(None);

pub fn with_active_ctx<R>(callback: impl FnOnce(&HspContext) -> R) -> R {
    let mut lock = ACTIVE_CTX.try_lock();
    todo!()
}
