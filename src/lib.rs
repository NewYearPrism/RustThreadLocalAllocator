#[cfg(feature = "bumpalo")]
pub mod bumpalo;

use std::alloc::Layout;
use std::cell::OnceCell;
use std::marker::PhantomData;
use std::ptr::NonNull;

use allocator_api2::alloc::AllocError;
use allocator_api2::alloc::Allocator;

pub struct ThreadLocalAllocator<A: Allocator>(PhantomData<A>);

impl<A: Allocator> ThreadLocalAllocator<A> {
    thread_local! {
        static ALLOC: OnceCell<Box<dyn Allocator>> = OnceCell::new();
    }
}

unsafe impl<A: Allocator + Default + 'static> Allocator for ThreadLocalAllocator<A> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Self::ALLOC.with(|cell| cell.get_or_init(|| Box::new(A::default())).allocate(layout))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Self::ALLOC.with(|cell| unsafe {
            cell.get_or_init(|| Box::new(A::default()))
                .deallocate(ptr, layout)
        })
    }

    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Self::ALLOC.with(|cell| {
            cell.get_or_init(|| Box::new(A::default()))
                .allocate_zeroed(layout)
        })
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        Self::ALLOC.with(|cell| unsafe {
            cell.get_or_init(|| Box::new(A::default()))
                .grow(ptr, old_layout, new_layout)
        })
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        Self::ALLOC.with(|cell| unsafe {
            cell.get_or_init(|| Box::new(A::default()))
                .grow_zeroed(ptr, old_layout, new_layout)
        })
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        Self::ALLOC.with(|cell| unsafe {
            cell.get_or_init(|| Box::new(A::default()))
                .shrink(ptr, old_layout, new_layout)
        })
    }
}
