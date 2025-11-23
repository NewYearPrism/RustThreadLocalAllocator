use std::alloc::Layout;
use std::ptr::NonNull;

use allocator_api2::alloc::AllocError;
use allocator_api2::alloc::Allocator;
use bumpalo::Bump;

#[derive(Debug, Clone, Copy)]
pub struct ThreadLocalBump;

impl ThreadLocalBump {
    thread_local! {
        pub static BUMP: Bump = Bump::new();
    }
}

unsafe impl Allocator for ThreadLocalBump {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Self::BUMP.with(|bump| bump.allocate(layout))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Self::BUMP.with(|bump| unsafe { bump.deallocate(ptr, layout) })
    }

    fn allocate_zeroed(&self, layout: std::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        Self::BUMP.with(|bump| bump.allocate_zeroed(layout))
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        Self::BUMP.with(|bump| unsafe { bump.grow(ptr, old_layout, new_layout) })
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        Self::BUMP.with(|bump| unsafe { bump.grow_zeroed(ptr, old_layout, new_layout) })
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        Self::BUMP.with(|bump| unsafe { bump.shrink(ptr, old_layout, new_layout) })
    }
}
