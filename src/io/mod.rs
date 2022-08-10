use crate::syscall::sys_write;
use alloc::string::ToString;
use kernel_constants::io::fd::FD_SYSOUT;

pub fn _print(args: core::fmt::Arguments) {
    let s = args.to_string();
    let s_ptr = s.as_ptr();
    let s_len = s.len();
    let _ = sys_write(FD_SYSOUT, s_ptr, s_len);
}
