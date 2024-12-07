#![no_std]

pub mod fs;
pub mod semaphore;
pub mod task;

use linkme::distributed_slice as def_api_handler;
/// 用与定义API处理函数的宏
///
/// # Example
/// ```rust
/// #[handle_api(ACCESS)]
/// fn access(pathname: *const i8, mode: i32) -> i32 {
///    unsafe { libc::access(pathname, mode) }
/// }
/// ```
pub use linkme::distributed_slice as handle_api;

#[macro_export]
macro_rules! define_api_handler {
    ($name:ident, $fn_name:ident, $return_type:ty, $($param_name:ident: $param_type:ty),*) => {
        #[def_api_handler]
        pub static $name: [fn($($param_name: $param_type),*) -> $return_type];


        pub fn $fn_name($($param_name: $param_type),*) -> $return_type {
            let mut iter = $name.iter();
            let Some(handler) = iter.next() else {
                panic!("No handler found for {}", stringify!($fn_name));
            };

            assert!(
                iter.next().is_none(),
                "Multiple handlers found for {}", stringify!($fn_name)
            );

            handler($($param_name),*)
        }
    };
}
