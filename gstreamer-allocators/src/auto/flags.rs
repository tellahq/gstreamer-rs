// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{bitflags::bitflags, translate::*};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstFdMemoryFlags")]
    pub struct FdMemoryFlags: u32 {
        #[doc(alias = "GST_FD_MEMORY_FLAG_NONE")]
        const NONE = ffi::GST_FD_MEMORY_FLAG_NONE as _;
        #[doc(alias = "GST_FD_MEMORY_FLAG_KEEP_MAPPED")]
        const KEEP_MAPPED = ffi::GST_FD_MEMORY_FLAG_KEEP_MAPPED as _;
        #[doc(alias = "GST_FD_MEMORY_FLAG_MAP_PRIVATE")]
        const MAP_PRIVATE = ffi::GST_FD_MEMORY_FLAG_MAP_PRIVATE as _;
        #[doc(alias = "GST_FD_MEMORY_FLAG_DONT_CLOSE")]
        const DONT_CLOSE = ffi::GST_FD_MEMORY_FLAG_DONT_CLOSE as _;
    }
}

#[doc(hidden)]
impl IntoGlib for FdMemoryFlags {
    type GlibType = ffi::GstFdMemoryFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstFdMemoryFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstFdMemoryFlags> for FdMemoryFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstFdMemoryFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}
