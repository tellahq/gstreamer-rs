// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstAppSink")]
    pub struct AppSink(Object<ffi::GstAppSink, ffi::GstAppSinkClass>) @extends gst::Element, gst::Object, @implements gst::URIHandler;

    match fn {
        type_ => || ffi::gst_app_sink_get_type(),
    }
}

impl AppSink {
    #[doc(alias = "gst_app_sink_get_buffer_list_support")]
    #[doc(alias = "get_buffer_list_support")]
    pub fn is_buffer_list_support(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_app_sink_get_buffer_list_support(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_app_sink_get_caps")]
    #[doc(alias = "get_caps")]
    pub fn caps(&self) -> Option<gst::Caps> {
        unsafe { from_glib_full(ffi::gst_app_sink_get_caps(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_app_sink_get_drop")]
    #[doc(alias = "get_drop")]
    pub fn is_drop(&self) -> bool {
        unsafe { from_glib(ffi::gst_app_sink_get_drop(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_app_sink_get_max_buffers")]
    #[doc(alias = "get_max_buffers")]
    pub fn max_buffers(&self) -> u32 {
        unsafe { ffi::gst_app_sink_get_max_buffers(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_app_sink_get_wait_on_eos")]
    #[doc(alias = "get_wait_on_eos")]
    pub fn is_wait_on_eos(&self) -> bool {
        unsafe { from_glib(ffi::gst_app_sink_get_wait_on_eos(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_app_sink_is_eos")]
    pub fn is_eos(&self) -> bool {
        unsafe { from_glib(ffi::gst_app_sink_is_eos(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_app_sink_pull_object")]
    pub fn pull_object(&self) -> Result<gst::MiniObject, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_app_sink_pull_object(self.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to pull object"))
        }
    }

    #[doc(alias = "gst_app_sink_pull_preroll")]
    pub fn pull_preroll(&self) -> Result<gst::Sample, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_app_sink_pull_preroll(self.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to pull preroll sample"))
        }
    }

    #[doc(alias = "gst_app_sink_pull_sample")]
    pub fn pull_sample(&self) -> Result<gst::Sample, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_app_sink_pull_sample(self.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to pull sample"))
        }
    }

    #[doc(alias = "gst_app_sink_set_buffer_list_support")]
    pub fn set_buffer_list_support(&self, enable_lists: bool) {
        unsafe {
            ffi::gst_app_sink_set_buffer_list_support(
                self.to_glib_none().0,
                enable_lists.into_glib(),
            );
        }
    }

    //#[doc(alias = "gst_app_sink_set_callbacks")]
    //pub fn set_callbacks(&self, callbacks: /*Ignored*/&mut AppSinkCallbacks, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gst_app_sink_set_callbacks() }
    //}

    #[doc(alias = "gst_app_sink_set_caps")]
    pub fn set_caps(&self, caps: Option<&gst::Caps>) {
        unsafe {
            ffi::gst_app_sink_set_caps(self.to_glib_none().0, caps.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_app_sink_set_drop")]
    pub fn set_drop(&self, drop: bool) {
        unsafe {
            ffi::gst_app_sink_set_drop(self.to_glib_none().0, drop.into_glib());
        }
    }

    #[doc(alias = "gst_app_sink_set_max_buffers")]
    pub fn set_max_buffers(&self, max: u32) {
        unsafe {
            ffi::gst_app_sink_set_max_buffers(self.to_glib_none().0, max);
        }
    }

    #[doc(alias = "gst_app_sink_set_wait_on_eos")]
    pub fn set_wait_on_eos(&self, wait: bool) {
        unsafe {
            ffi::gst_app_sink_set_wait_on_eos(self.to_glib_none().0, wait.into_glib());
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_app_sink_try_pull_object")]
    pub fn try_pull_object(
        &self,
        timeout: impl Into<Option<gst::ClockTime>>,
    ) -> Option<gst::MiniObject> {
        unsafe {
            from_glib_full(ffi::gst_app_sink_try_pull_object(
                self.to_glib_none().0,
                timeout.into().into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_app_sink_try_pull_preroll")]
    pub fn try_pull_preroll(
        &self,
        timeout: impl Into<Option<gst::ClockTime>>,
    ) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::gst_app_sink_try_pull_preroll(
                self.to_glib_none().0,
                timeout.into().into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_app_sink_try_pull_sample")]
    pub fn try_pull_sample(
        &self,
        timeout: impl Into<Option<gst::ClockTime>>,
    ) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(ffi::gst_app_sink_try_pull_sample(
                self.to_glib_none().0,
                timeout.into().into_glib(),
            ))
        }
    }

    #[doc(alias = "buffer-list")]
    pub fn is_buffer_list(&self) -> bool {
        glib::ObjectExt::property(self, "buffer-list")
    }

    #[doc(alias = "buffer-list")]
    pub fn set_buffer_list(&self, buffer_list: bool) {
        glib::ObjectExt::set_property(self, "buffer-list", &buffer_list)
    }

    #[doc(alias = "buffer-list")]
    pub fn connect_buffer_list_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_list_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
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
                b"notify::buffer-list\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_buffer_list_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "caps")]
    pub fn connect_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_caps_trampoline<F: Fn(&AppSink) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSink,
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
                b"notify::caps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_caps_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drop")]
    pub fn connect_drop_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_drop_trampoline<F: Fn(&AppSink) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSink,
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
                b"notify::drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_drop_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "eos")]
    pub fn connect_eos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_eos_trampoline<F: Fn(&AppSink) + Send + Sync + 'static>(
            this: *mut ffi::GstAppSink,
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
                b"notify::eos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_eos_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-buffers")]
    pub fn connect_max_buffers_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_buffers_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
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
                b"notify::max-buffers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_buffers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "wait-on-eos")]
    pub fn connect_wait_on_eos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_wait_on_eos_trampoline<
            F: Fn(&AppSink) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAppSink,
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
                b"notify::wait-on-eos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wait_on_eos_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for AppSink {}
unsafe impl Sync for AppSink {}
