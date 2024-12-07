use std::thread::{sleep, spawn};

use libc::O_EXCL;
use posix_api::{
    fs::{ACCESS, CLOSE, MMAP, OPEN, UNLINK, WRITE},
    handle_api,
    semaphore::{
        sem_close, sem_getvalue, sem_open, sem_post, sem_unlink, sem_wait, O_CREAT, O_RDWR,
    },
};

extern crate semaphores;

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

#[handle_api(UNLINK)]
fn unlink(pathname: *const i8) -> i32 {
    unsafe { libc::unlink(pathname) }
}

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

#[test]
fn test_post() {
    let sem = sem_open(
        "test\0".as_ptr() as *const i8,
        O_CREAT | O_EXCL | O_RDWR,
        0o644,
        1,
    );

    sem_post(sem);

    let mut sval = 0;

    sem_getvalue(sem, &mut sval as *mut libc::c_int);

    assert!(sval == 2);

    sem_unlink("test\0".as_ptr() as *const i8);
}

#[test]
fn test_wait() {
    let sem = sem_open(
        "test\0".as_ptr() as *const i8,
        O_CREAT | O_EXCL | O_RDWR,
        0o644,
        1,
    );

    sem_wait(sem);

    let mut sval = 0;

    sem_getvalue(sem, &mut sval as *mut libc::c_int);

    assert!(sval == 0);

    sem_unlink("test\0".as_ptr() as *const i8);
}

#[test]
fn test_pw() {
    let name = "test\0".as_ptr() as *const i8;

    let _sem = sem_open(name, O_CREAT | O_EXCL | O_RDWR, 0o644, 1);

    let mut vec = Vec::new();
    for i in 0..3 {
        let thread = spawn(move || {
            let sem = sem_open("test\0".as_ptr() as *const i8, O_RDWR, 0o644, 0);

            sem_wait(sem);

            println!("Thread{} acquired semaphore", i);

            let mut value = 0;
            sem_getvalue(sem, &mut value as *mut libc::c_int);
            assert_eq!(value, 0);
            println!("Thread{} value: {}", i, value);

            sleep(std::time::Duration::from_secs(1));

            sem_post(sem);

            sem_close(sem);
        });
        vec.push(thread);
    }

    for thread in vec {
        thread.join().unwrap();
    }

    sem_unlink("test\0".as_ptr() as *const i8);
}
