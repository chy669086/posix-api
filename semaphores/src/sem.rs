use core::sync::atomic::AtomicUsize;

#[repr(C)]
pub(crate) struct Sem {
    pub value: AtomicUsize,
    pub fd: i32,
    pub waiters: AtomicUsize,
}
