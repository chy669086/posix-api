use posix_api::{
    fs::{ACCESS, CLOSE, FSYNC, MMAP, OPEN, WRITE},
    handle_api,
    semaphore::{sem_close, sem_open, sem_unlink, O_CREAT, O_RDWR},
};

#[handle_api(ACCESS)]
fn access(_pathname: *const i8, _mode: i32) -> i32 {
    unsafe { libc::access(_pathname, _mode) }
}

#[handle_api(OPEN)]
fn open(pathname: *const i8, flags: i32, mode: u32) -> i32 {
    unsafe { libc::open(pathname, flags, mode) }
}

#[handle_api(WRITE)]
fn write(fd: i32, buf: *const libc::c_void, count: usize) -> isize {
    unsafe { libc::write(fd, buf, count) }
}

#[handle_api(FSYNC)]
fn fsync(fd: i32) -> i32 {
    unsafe { libc::fsync(fd) }
}

#[handle_api(CLOSE)]
fn close(fd: i32) -> i32 {
    unsafe { libc::close(fd) }
}

#[handle_api(MMAP)]
fn mmap(
    addr: *mut libc::c_void,
    length: usize,
    prot: i32,
    flags: i32,
    fd: i32,
    offset: i64,
) -> *mut libc::c_void {
    unsafe { libc::mmap(addr, length, prot, flags, fd, offset) }
}

extern crate semaphores;

#[test]
fn test_open() {
    let sem = sem_open(b"test\0".as_ptr() as *const i8, O_CREAT | O_RDWR, 0o644, 1);
    if sem == posix_api::semaphore::SEM_FAILED {
        panic!("Failed to create semaphore");
    }

    println!("Semaphore created");

    sem_close(sem);
    sem_unlink(b"test\0".as_ptr() as *const i8);
}
