use modinverse::modinverse;
use rhsp3_internal_common::{bail_code, errors::*};
use std::{
    cell::RefCell,
    ffi::c_int,
    hash::{Hash, Hasher},
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
    time::SystemTime,
};
use twox_hash::Xxh3Hash64;

#[cold]
#[inline(never)]
fn next_seed() -> Result<u32> {
    static ENTROPY: AtomicU32 = AtomicU32::new(0x12345);

    let time_diff = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let entropy = (std::process::id() as u64, time_diff.as_micros());
    loop {
        let cur_data = ENTROPY.load(Ordering::Relaxed);
        let mut hashed = Xxh3Hash64::with_seed(0x12345);
        cur_data.hash(&mut hashed);
        entropy.hash(&mut hashed);
        let result = hashed.finish();

        if ENTROPY
            .compare_exchange(cur_data, result as u32, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            return Ok((result >> 32) as u32);
        }
    }
}

struct ObjectHandlesCore<T> {
    obf_add: u32,
    obf_mul: u32,
    obf_mulinv: u32,
    obf_xor: u32,

    data: Vec<FreeListNode<T>>,
    free_head: usize,
}
enum FreeListNode<T> {
    Exists(T),
    Free(usize),
}
impl<T> ObjectHandlesCore<T> {
    fn new() -> Result<Self> {
        let obf_add = next_seed()?;
        let obf_mul = next_seed()? | 1;
        let obf_mulinv = match modinverse(obf_mul as u64, u32::MAX as u64 + 1) {
            None => unreachable!(),
            Some(v) => v as u32,
        };
        let obf_xor = next_seed()?;

        Ok(ObjectHandlesCore {
            obf_add,
            obf_mul,
            obf_mulinv,
            obf_xor,
            data: Vec::new(),
            free_head: 0,
        })
    }

    fn idx_obf(&self, handle: u32) -> c_int {
        ((handle * self.obf_mul + self.obf_add) ^ self.obf_xor) as c_int
    }
    fn idx_deobf(&self, handle: c_int) -> u32 {
        ((handle as u32 ^ self.obf_xor) - self.obf_add) * self.obf_mulinv
    }

    fn alloc(&mut self, value: T) -> c_int {
        if self.free_head == self.data.len() {
            let idx = self.data.len();
            assert!(idx <= u32::MAX as usize);
            self.data.push(FreeListNode::Exists(value));
            self.free_head += 1;
            self.idx_obf(idx as u32)
        } else {
            let idx = self.free_head;
            assert!(self.free_head < self.data.len(), "Free list corrupted.");
            match &self.data[idx] {
                FreeListNode::Exists(_) => panic!("Free list corrupted."),
                FreeListNode::Free(new_free) => {
                    self.free_head = *new_free;
                    self.data[idx] = FreeListNode::Exists(value);
                    self.idx_obf(idx as u32)
                }
            }
        }
    }
    fn get(&mut self, handle: c_int) -> Result<&mut T> {
        let idx = self.idx_deobf(handle) as usize;
        if idx > self.data.len() {
            bail_code!(OutOfBoundsAccess);
        }
        match &mut self.data[idx] {
            FreeListNode::Exists(v) => Ok(v),
            FreeListNode::Free(_) => bail_code!(OutOfBoundsAccess),
        }
    }
    fn free(&mut self, handle: c_int) -> Result<()> {
        let idx = self.idx_deobf(handle) as usize;
        if idx > self.data.len() {
            bail_code!(OutOfBoundsAccess);
        }
        match &self.data[idx] {
            FreeListNode::Exists(_) => {
                self.data[idx] = FreeListNode::Free(self.free_head);
                self.free_head = idx;
                Ok(())
            }
            FreeListNode::Free(_) => bail_code!(OutOfBoundsAccess),
        }
    }
}

/// A type that generates opaque and memory safe `int` handles for HSP code.
pub struct ObjectStore<T>(RefCell<ObjectHandlesCore<Rc<T>>>);
impl<T> ObjectStore<T> {
    /// Creates a new object store.
    pub fn new() -> Result<Self> {
        Ok(ObjectStore(RefCell::new(ObjectHandlesCore::new()?)))
    }

    /// Creates a handle for the given object.
    pub fn alloc(&self, value: T) -> Result<c_int> {
        Ok(self.0.borrow_mut().alloc(Rc::new(value)))
    }

    /// Returns the object for a given handle.
    pub fn get(&self, handle: c_int) -> Result<Rc<T>> {
        Ok(self.0.borrow_mut().get(handle)?.clone())
    }

    /// Frees a given handle.
    pub fn free(&self, handle: c_int) -> Result<()> {
        self.0.borrow_mut().free(handle)
    }
}
