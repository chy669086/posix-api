#![no_std]

mod sem;

use core::sync::atomic::AtomicUsize;
use libc::PROT_READ;
use posix_api::fs::{access, close, fsync, mmap, open, write, F_OK, MAP_SHARED, PROT_WRITE};
use posix_api::handle_api;
use posix_api::semaphore::{O_CREAT, O_EXCL, SEM_CLOSE, SEM_FAILED, SEM_OPEN, SEM_POST, SEM_WAIT};

#[handle_api(SEM_OPEN)]
fn sem_open(
    name: *const libc::c_char,
    oflag: libc::c_int,
    mode: libc::c_uint,
    value: libc::c_int,
) -> *mut libc::c_void {
    // Implementation
    let f = access(name, F_OK);
    if f == 0 && oflag & O_EXCL != 0 {
        return SEM_FAILED;
    }

    if f != 0 && oflag & O_CREAT == 0 {
        return SEM_FAILED;
    }

    let fd = open(name, oflag, mode);
    if fd < 0 {
        return SEM_FAILED;
    }

    if f != 0 {
        let sem = sem::Sem {
            value: AtomicUsize::new(value as usize),
            fd,
            waiters: AtomicUsize::new(0),
        };
        write(
            fd,
            &sem as *const _ as *const libc::c_void,
            core::mem::size_of::<sem::Sem>(),
        );
        fsync(fd);
    }

    mmap(
        0 as *mut libc::c_void,
        core::mem::size_of::<sem::Sem>(),
        PROT_WRITE | PROT_READ,
        MAP_SHARED,
        fd,
        0,
    )
}

#[handle_api(SEM_POST)]
fn sem_post(sem: *mut libc::c_void) -> libc::c_int {
    // Implementation
    let sem = unsafe { &*(sem as *const sem::Sem) };
    sem.value.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    0
}

#[handle_api(SEM_CLOSE)]
fn sem_close(sem: *mut libc::c_void) -> libc::c_int {
    // Implementation
    let sem = unsafe { &*(sem as *const sem::Sem) };
    let fd = sem.fd;
    close(fd);
    0
}

#[handle_api(SEM_WAIT)]
fn sem_wait(sem: *mut libc::c_void) -> libc::c_int {
    // Implementation
    let sem = unsafe { &*(sem as *const sem::Sem) };
    loop {
        let value = sem.value.load(core::sync::atomic::Ordering::SeqCst);
        if value > 0 {
            if sem
                .value
                .compare_exchange(
                    value,
                    value - 1,
                    core::sync::atomic::Ordering::SeqCst,
                    core::sync::atomic::Ordering::SeqCst,
                )
                .is_ok()
            {
                break;
            }
        }
    }
    0
}
