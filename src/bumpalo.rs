use std::alloc::Layout;
use std::cell::RefCell;
use std::ptr::NonNull;

use allocator_api2::alloc::AllocError;
use allocator_api2::alloc::Allocator;
use bumpalo::Bump;

#[derive(Debug, Clone, Copy)]
pub struct ThreadLocalBump;

impl ThreadLocalBump {
    thread_local! {
        pub static BUMP: RefCell<Bump> = RefCell::new(Bump::new());
    }
}

unsafe impl Allocator for ThreadLocalBump {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Self::BUMP.with(|bump| (&*bump.borrow()).allocate(layout))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Self::BUMP.with(|bump| unsafe { (&*bump.borrow()).deallocate(ptr, layout) })
    }

    fn allocate_zeroed(&self, layout: std::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        Self::BUMP.with(|bump| (&*bump.borrow()).allocate_zeroed(layout))
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        Self::BUMP.with(|bump| unsafe { (&*bump.borrow()).grow(ptr, old_layout, new_layout) })
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        Self::BUMP.with(|bump| unsafe { (&*bump.borrow()).grow_zeroed(ptr, old_layout, new_layout) })
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        Self::BUMP.with(|bump| unsafe { (&*bump.borrow()).shrink(ptr, old_layout, new_layout) })
    }
}
