// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{GLContext, GLStereoDownmix};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstGLViewConvert")]
    pub struct GLViewConvert(Object<ffi::GstGLViewConvert, ffi::GstGLViewConvertClass>) @extends gst::Object;

    match fn {
        type_ => || ffi::gst_gl_view_convert_get_type(),
    }
}

impl GLViewConvert {
    #[doc(alias = "gst_gl_view_convert_new")]
    pub fn new() -> GLViewConvert {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_gl_view_convert_new()) }
    }

    #[doc(alias = "gst_gl_view_convert_perform")]
    pub fn perform(&self, inbuf: &gst::Buffer) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(ffi::gst_gl_view_convert_perform(
                self.to_glib_none().0,
                inbuf.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_view_convert_reset")]
    pub fn reset(&self) {
        unsafe {
            ffi::gst_gl_view_convert_reset(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_view_convert_set_caps")]
    pub fn set_caps(
        &self,
        in_caps: &gst::Caps,
        out_caps: &gst::Caps,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_view_convert_set_caps(
                    self.to_glib_none().0,
                    in_caps.to_glib_none().0,
                    out_caps.to_glib_none().0
                ),
                "Failed to set caps"
            )
        }
    }

    #[doc(alias = "gst_gl_view_convert_set_context")]
    pub fn set_context(&self, context: &impl IsA<GLContext>) {
        unsafe {
            ffi::gst_gl_view_convert_set_context(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_view_convert_transform_caps")]
    pub fn transform_caps(
        &self,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: &gst::Caps,
    ) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_gl_view_convert_transform_caps(
                self.to_glib_none().0,
                direction.into_glib(),
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "downmix-mode")]
    pub fn downmix_mode(&self) -> GLStereoDownmix {
        ObjectExt::property(self, "downmix-mode")
    }

    #[doc(alias = "downmix-mode")]
    pub fn set_downmix_mode(&self, downmix_mode: GLStereoDownmix) {
        ObjectExt::set_property(self, "downmix-mode", downmix_mode)
    }

    #[doc(alias = "input-flags-override")]
    pub fn input_flags_override(&self) -> gst_video::VideoMultiviewFlags {
        ObjectExt::property(self, "input-flags-override")
    }

    #[doc(alias = "input-flags-override")]
    pub fn set_input_flags_override(&self, input_flags_override: gst_video::VideoMultiviewFlags) {
        ObjectExt::set_property(self, "input-flags-override", input_flags_override)
    }

    #[doc(alias = "input-mode-override")]
    pub fn input_mode_override(&self) -> gst_video::VideoMultiviewMode {
        ObjectExt::property(self, "input-mode-override")
    }

    #[doc(alias = "input-mode-override")]
    pub fn set_input_mode_override(&self, input_mode_override: gst_video::VideoMultiviewMode) {
        ObjectExt::set_property(self, "input-mode-override", input_mode_override)
    }

    #[doc(alias = "output-flags-override")]
    pub fn output_flags_override(&self) -> gst_video::VideoMultiviewFlags {
        ObjectExt::property(self, "output-flags-override")
    }

    #[doc(alias = "output-flags-override")]
    pub fn set_output_flags_override(&self, output_flags_override: gst_video::VideoMultiviewFlags) {
        ObjectExt::set_property(self, "output-flags-override", output_flags_override)
    }

    #[doc(alias = "output-mode-override")]
    pub fn output_mode_override(&self) -> gst_video::VideoMultiviewMode {
        ObjectExt::property(self, "output-mode-override")
    }

    #[doc(alias = "output-mode-override")]
    pub fn set_output_mode_override(&self, output_mode_override: gst_video::VideoMultiviewMode) {
        ObjectExt::set_property(self, "output-mode-override", output_mode_override)
    }

    #[doc(alias = "downmix-mode")]
    pub fn connect_downmix_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_downmix_mode_trampoline<
            F: Fn(&GLViewConvert) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLViewConvert,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::downmix-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_downmix_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "input-flags-override")]
    pub fn connect_input_flags_override_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_flags_override_trampoline<
            F: Fn(&GLViewConvert) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLViewConvert,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-flags-override\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_flags_override_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "input-mode-override")]
    pub fn connect_input_mode_override_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_mode_override_trampoline<
            F: Fn(&GLViewConvert) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLViewConvert,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-mode-override\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_mode_override_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "output-flags-override")]
    pub fn connect_output_flags_override_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_output_flags_override_trampoline<
            F: Fn(&GLViewConvert) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLViewConvert,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::output-flags-override\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_output_flags_override_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "output-mode-override")]
    pub fn connect_output_mode_override_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_output_mode_override_trampoline<
            F: Fn(&GLViewConvert) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLViewConvert,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::output-mode-override\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_output_mode_override_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GLViewConvert {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for GLViewConvert {}
unsafe impl Sync for GLViewConvert {}
