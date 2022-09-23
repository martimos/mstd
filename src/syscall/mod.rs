use crate::io::FileDescriptor;
use core::arch::asm;
use kernel_constants::syscall::error::Errno;
use kernel_constants::syscall::Syscall;

#[cfg(not(target_os = "martim"))]
compile_error!("Syscalls only implemented for MartimOS");

pub struct Syscalls;

pub type Result<T> = core::result::Result<T, Errno>;

impl Syscalls {
    /// Write syscall.
    ///
    /// Writes `buf` to the given `fd` file descriptor. The amount of bytes
    /// written is returned upon successful completion.
    pub fn write<T>(fd: FileDescriptor, buf: T) -> Result<usize>
    where
        T: AsRef<[u8]>,
    {
        let slice = buf.as_ref();
        let res = unsafe {
            // Safety: this is safe because slice is valid here
            sys_write(fd.as_usize(), slice.as_ptr(), slice.len())
        };
        if res < 0 {
            Err(Errno::try_from(-res).expect("invalid errno"))
        } else {
            Ok(res as usize)
        }
    }

    /// Read syscall.
    ///
    /// Reads from the given `fd` into the given `buf`. The amount of
    /// bytes read is returned upon successful completion.
    pub fn read<T>(fd: FileDescriptor, buf: &mut T) -> Result<usize>
    where
        T: AsMut<[u8]>,
    {
        let slice = buf.as_mut();
        let res = unsafe {
            // Safety: this is safe because slice is valid here
            sys_read(fd.as_usize(), slice.as_mut_ptr(), slice.len())
        };
        if res < 0 {
            Err(Errno::try_from(res).expect("invalid errno"))
        } else {
            Ok(res as usize)
        }
    }
}

unsafe fn sys_write(fd: usize, buf: *const u8, count: usize) -> isize {
    unsafe { syscall_3(Syscall::Write, fd, buf as usize, count) }
}

unsafe fn sys_read(fd: usize, buf: *mut u8, count: usize) -> isize {
    unsafe { syscall_3(Syscall::Read, fd, buf as usize, count) }
}

unsafe fn syscall_3<T>(syscall_number: T, arg1: usize, arg2: usize, arg3: usize) -> isize
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
