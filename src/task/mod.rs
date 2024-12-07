use super::{def_api_handler, define_api_handler};

define_api_handler!(SCHED_YIELD, sched_yield, libc::c_int,);
