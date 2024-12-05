use super::{def_api_handler, define_api_handler};

pub const SEM_FAILED: *mut libc::c_void = 0 as *mut libc::c_void;
pub const SEM_VALUE_MAX: libc::c_int = 32767;
pub const SEM_VALUE_MIN: libc::c_int = 0;
pub const O_CREAT: libc::c_int = 0o100;
pub const O_EXCL: libc::c_int = 0o200;
pub const O_RDWR: libc::c_int = 0o2;
pub const O_TRUNC: libc::c_int = 0o1000;
pub const O_WRONLY: libc::c_int = 0o1;

define_api_handler!(
    SEM_OPEN,
    sem_open,
    *mut libc::c_void,
    name: *const libc::c_char,
    oflag: libc::c_int,
    mode: libc::c_uint,
    value: libc::c_int
);

define_api_handler!(
    SEM_CLOSE,
    sem_close,
    libc::c_int,
    sem: *mut libc::c_void
);

define_api_handler!(
    SEM_UNLINK,
    sem_unlink,
    libc::c_int,
    name: *const libc::c_char
);

define_api_handler!(
    SEM_WAIT,
    sem_wait,
    libc::c_int,
    sem: *mut libc::c_void
);

define_api_handler!(
    SEM_POST,
    sem_post,
    libc::c_int,
    sem: *mut libc::c_void
);

define_api_handler!(
    SEM_GETVALUE,
    sem_getvalue,
    libc::c_int,
    sem: *mut libc::c_void,
    sval: *mut libc::c_int
);

define_api_handler!(
    SEM_INIT,
    sem_init,
    libc::c_int,
    sem: *mut libc::c_void,
    pshared: libc::c_int,
    value: libc::c_uint
);

define_api_handler!(
    SEM_DESTROY,
    sem_destroy,
    libc::c_int,
    sem: *mut libc::c_void
);

define_api_handler!(
    SEM_TIMEDWAIT,
    sem_timedwait,
    libc::c_int,
    sem: *mut libc::c_void,
    abstime: *const libc::timespec
);

define_api_handler!(
    SEM_TRYWAIT,
    sem_trywait,
    libc::c_int,
    sem: *mut libc::c_void
);
