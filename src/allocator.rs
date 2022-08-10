use core::alloc::{GlobalAlloc, Layout};

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("alloc_error_handler: {:?}", layout);
}

struct NoOpAllocator;

#[global_allocator]
static ALLOCATOR: NoOpAllocator = NoOpAllocator;

unsafe impl GlobalAlloc for NoOpAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        todo!()
    }
}
