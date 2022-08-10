use core::arch::asm;
use kernel_constants::syscall::Syscall;

pub fn sys_write(fd: usize, buf: *const u8, count: usize) -> isize {
    unsafe { syscall_3(Syscall::Write as usize, fd, buf as usize, count) }
}

pub fn sys_read(fd: usize, buf: *const u8, count: usize) -> isize {
    unsafe { syscall_3(Syscall::Read as usize, fd, buf as usize, count) }
}

pub unsafe fn syscall_3(syscall_number: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    let ret;
    asm!(
    "syscall",
    in("rax") syscall_number,
    in("rdi") arg1,
    in("rsi") arg2,
    in("rdx") arg3,
    lateout("rax") ret,
    );
    ret
}
