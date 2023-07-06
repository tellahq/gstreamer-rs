// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstGLBaseSrc")]
    pub struct GLBaseSrc(Object<ffi::GstGLBaseSrc, ffi::GstGLBaseSrcClass>) @extends gst_base::PushSrc, gst_base::BaseSrc, gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_gl_base_src_get_type(),
    }
}

impl GLBaseSrc {
    pub const NONE: Option<&'static GLBaseSrc> = None;
}

unsafe impl Send for GLBaseSrc {}
unsafe impl Sync for GLBaseSrc {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::GLBaseSrc>> Sealed for T {}
}

pub trait GLBaseSrcExt: IsA<GLBaseSrc> + sealed::Sealed + 'static {
    #[doc(alias = "timestamp-offset")]
    fn timestamp_offset(&self) -> i64 {
        ObjectExt::property(self.as_ref(), "timestamp-offset")
    }

    #[doc(alias = "timestamp-offset")]
    fn set_timestamp_offset(&self, timestamp_offset: i64) {
        ObjectExt::set_property(self.as_ref(), "timestamp-offset", timestamp_offset)
    }

    #[doc(alias = "timestamp-offset")]
    fn connect_timestamp_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timestamp_offset_trampoline<
            P: IsA<GLBaseSrc>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLBaseSrc,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLBaseSrc::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timestamp-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timestamp_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<GLBaseSrc>> GLBaseSrcExt for O {}
