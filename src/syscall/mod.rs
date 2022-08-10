use core::arch::asm;
use kernel_constants::syscall::Syscall;

pub fn sys_write(fd: usize, buf: *const u8, count: usize) -> isize {
    unsafe { syscall_3(Syscall::Write, fd, buf as usize, count) }
}

pub fn sys_read(fd: usize, buf: *mut u8, count: usize) -> isize {
    unsafe { syscall_3(Syscall::Read, fd, buf as usize, count) }
}

pub unsafe fn syscall_3<T>(syscall_number: T, arg1: usize, arg2: usize, arg3: usize) -> isize
where
    T: Into<usize>,
{
    let n = Into::<usize>::into(syscall_number);
    let ret;
    asm!(
    "syscall",
    in("rax") n,
    in("rdi") arg1,
    in("rsi") arg2,
    in("rdx") arg3,
    lateout("rax") ret,
    );
    ret
}
