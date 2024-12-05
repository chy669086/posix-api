mod api;

use super::{def_api_handler, define_api_handler};

pub use api::*;

define_api_handler!(
    OPEN,
    open,
    libc::c_int,
    name: *const libc::c_char,
    oflag: libc::c_int,
    mode: libc::c_uint
);

define_api_handler!(
    CLOSE,
    close,
    libc::c_int,
    fd: libc::c_int
);

define_api_handler!(
    READ,
    read,
    libc::ssize_t,
    fd: libc::c_int,
    buf: *mut libc::c_void,
    count: libc::size_t
);

define_api_handler!(
    WRITE,
    write,
    libc::ssize_t,
    fd: libc::c_int,
    buf: *const libc::c_void,
    count: libc::size_t
);

define_api_handler!(
    LSEEK,
    lseek,
    libc::off_t,
    fd: libc::c_int,
    offset: libc::off_t,
    whence: libc::c_int
);

define_api_handler!(
    UNLINK,
    unlink,
    libc::c_int,
    pathname: *const libc::c_char
);

define_api_handler!(
    ACCESS,
    access,
    libc::c_int,
    pathname: *const libc::c_char,
    mode: libc::c_int
);

define_api_handler!(
    MMAP,
    mmap,
    *mut libc::c_void,
    addr: *mut libc::c_void,
    length: libc::size_t,
    prot: libc::c_int,
    flags: libc::c_int,
    fd: libc::c_int,
    offset: libc::off_t
);

define_api_handler!(
    FSYNC,
    fsync,
    libc::c_int,
    fd: libc::c_int
);
