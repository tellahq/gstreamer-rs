// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
mod dma_buf_allocator;
#[cfg(any(target_os = "linux", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(target_os = "linux")))]
pub use self::dma_buf_allocator::DmaBufAllocator;

mod fd_allocator;
pub use self::fd_allocator::FdAllocator;

mod phys_memory_allocator;
pub use self::phys_memory_allocator::PhysMemoryAllocator;

mod flags;
pub use self::flags::FdMemoryFlags;

pub mod functions;

mod constants;
pub use self::constants::ALLOCATOR_DMABUF;
pub use self::constants::ALLOCATOR_FD;
pub use self::constants::CAPS_FEATURE_MEMORY_DMABUF;

#[doc(hidden)]
pub mod traits {
    pub use super::phys_memory_allocator::PhysMemoryAllocatorExt;
}
