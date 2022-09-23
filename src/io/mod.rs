use crate::syscall::Syscalls;
use alloc::string::ToString;
use kernel_constants::io::fd::FD_SYSOUT;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FileDescriptor {
    handle: usize,
}

impl From<usize> for FileDescriptor {
    fn from(handle: usize) -> Self {
        Self { handle }
    }
}

impl FileDescriptor {
    pub fn as_usize(&self) -> usize {
        self.handle
    }
}

pub fn _print(args: core::fmt::Arguments) {
    let s = args.to_string();
    let _ = Syscalls::write(FD_SYSOUT.into(), s);
}
