// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::RTSPFilterResult;
use crate::RTSPMedia;
use crate::RTSPSessionMedia;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstRTSPSession")]
    pub struct RTSPSession(Object<ffi::GstRTSPSession, ffi::GstRTSPSessionClass>);

    match fn {
        type_ => || ffi::gst_rtsp_session_get_type(),
    }
}

impl RTSPSession {
    pub const NONE: Option<&'static RTSPSession> = None;

    #[doc(alias = "gst_rtsp_session_new")]
    pub fn new(sessionid: &str) -> RTSPSession {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_session_new(sessionid.to_glib_none().0)) }
    }
}

unsafe impl Send for RTSPSession {}
unsafe impl Sync for RTSPSession {}

pub trait RTSPSessionExt: 'static {
    #[doc(alias = "gst_rtsp_session_allow_expire")]
    fn allow_expire(&self);

    #[doc(alias = "gst_rtsp_session_filter")]
    fn filter(
        &self,
        func: Option<&mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult)>,
    ) -> Vec<RTSPSessionMedia>;

    #[doc(alias = "gst_rtsp_session_get_header")]
    #[doc(alias = "get_header")]
    fn header(&self) -> Option<glib::GString>;

    #[doc(alias = "gst_rtsp_session_get_media")]
    #[doc(alias = "get_media")]
    fn media(&self, path: &str) -> (Option<RTSPSessionMedia>, i32);

    #[doc(alias = "gst_rtsp_session_get_sessionid")]
    #[doc(alias = "get_sessionid")]
    fn sessionid(&self) -> Option<glib::GString>;

    #[doc(alias = "gst_rtsp_session_get_timeout")]
    #[doc(alias = "get_timeout")]
    fn timeout(&self) -> u32;

    //#[doc(alias = "gst_rtsp_session_is_expired")]
    //fn is_expired(&self, now: /*Ignored*/&mut glib::TimeVal) -> bool;

    #[doc(alias = "gst_rtsp_session_is_expired_usec")]
    fn is_expired_usec(&self, now: i64) -> bool;

    #[doc(alias = "gst_rtsp_session_manage_media")]
    fn manage_media(
        &self,
        path: &str,
        media: &impl IsA<RTSPMedia>,
    ) -> Result<RTSPSessionMedia, glib::BoolError>;

    //#[doc(alias = "gst_rtsp_session_next_timeout")]
    //fn next_timeout(&self, now: /*Ignored*/&mut glib::TimeVal) -> i32;

    #[doc(alias = "gst_rtsp_session_next_timeout_usec")]
    fn next_timeout_usec(&self, now: i64) -> i32;

    #[doc(alias = "gst_rtsp_session_prevent_expire")]
    fn prevent_expire(&self);

    #[doc(alias = "gst_rtsp_session_release_media")]
    fn release_media(&self, media: &impl IsA<RTSPSessionMedia>) -> bool;

    #[doc(alias = "gst_rtsp_session_set_timeout")]
    fn set_timeout(&self, timeout: u32);

    #[doc(alias = "gst_rtsp_session_touch")]
    fn touch(&self);

    #[doc(alias = "extra-timeout")]
    fn extra_timeout(&self) -> u32;

    #[doc(alias = "extra-timeout")]
    fn set_extra_timeout(&self, extra_timeout: u32);

    #[doc(alias = "timeout-always-visible")]
    fn is_timeout_always_visible(&self) -> bool;

    #[doc(alias = "timeout-always-visible")]
    fn set_timeout_always_visible(&self, timeout_always_visible: bool);

    #[doc(alias = "extra-timeout")]
    fn connect_extra_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "timeout")]
    fn connect_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[doc(alias = "timeout-always-visible")]
    fn connect_timeout_always_visible_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<RTSPSession>> RTSPSessionExt for O {
    fn allow_expire(&self) {
        unsafe {
            ffi::gst_rtsp_session_allow_expire(self.as_ref().to_glib_none().0);
        }
    }

    fn filter(
        &self,
        func: Option<&mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult)>,
    ) -> Vec<RTSPSessionMedia> {
        let func_data: Option<
            &mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult),
        > = func;
        unsafe extern "C" fn func_func(
            sess: *mut ffi::GstRTSPSession,
            media: *mut ffi::GstRTSPSessionMedia,
            user_data: glib::ffi::gpointer,
        ) -> ffi::GstRTSPFilterResult {
            let sess = from_glib_borrow(sess);
            let media = from_glib_borrow(media);
            let callback: *mut Option<
                &mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult),
            > = user_data as *const _ as usize
                as *mut Option<
                    &mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult),
                >;
            let res = if let Some(ref mut callback) = *callback {
                callback(&sess, &media)
            } else {
                panic!("cannot get closure...")
            };
            res.into_glib()
        }
        let func = if func_data.is_some() {
            Some(func_func as _)
        } else {
            None
        };
        let super_callback0: &Option<
            &mut dyn (FnMut(&RTSPSession, &RTSPSessionMedia) -> RTSPFilterResult),
        > = &func_data;
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_rtsp_session_filter(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    fn header(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_get_header(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn media(&self, path: &str) -> (Option<RTSPSessionMedia>, i32) {
        unsafe {
            let mut matched = mem::MaybeUninit::uninit();
            let ret = from_glib_none(ffi::gst_rtsp_session_get_media(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                matched.as_mut_ptr(),
            ));
            (ret, matched.assume_init())
        }
    }

    fn sessionid(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_session_get_sessionid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn timeout(&self) -> u32 {
        unsafe { ffi::gst_rtsp_session_get_timeout(self.as_ref().to_glib_none().0) }
    }

    //fn is_expired(&self, now: /*Ignored*/&mut glib::TimeVal) -> bool {
    //    unsafe { TODO: call ffi:gst_rtsp_session_is_expired() }
    //}

    fn is_expired_usec(&self, now: i64) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_session_is_expired_usec(
                self.as_ref().to_glib_none().0,
                now,
            ))
        }
    }

    fn manage_media(
        &self,
        path: &str,
        media: &impl IsA<RTSPMedia>,
    ) -> Result<RTSPSessionMedia, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::gst_rtsp_session_manage_media(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                media.as_ref().to_glib_full(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to manage media"))
        }
    }

    //fn next_timeout(&self, now: /*Ignored*/&mut glib::TimeVal) -> i32 {
    //    unsafe { TODO: call ffi:gst_rtsp_session_next_timeout() }
    //}

    fn next_timeout_usec(&self, now: i64) -> i32 {
        unsafe { ffi::gst_rtsp_session_next_timeout_usec(self.as_ref().to_glib_none().0, now) }
    }

    fn prevent_expire(&self) {
        unsafe {
            ffi::gst_rtsp_session_prevent_expire(self.as_ref().to_glib_none().0);
        }
    }

    fn release_media(&self, media: &impl IsA<RTSPSessionMedia>) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_session_release_media(
                self.as_ref().to_glib_none().0,
                media.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_timeout(&self, timeout: u32) {
        unsafe {
            ffi::gst_rtsp_session_set_timeout(self.as_ref().to_glib_none().0, timeout);
        }
    }

    fn touch(&self) {
        unsafe {
            ffi::gst_rtsp_session_touch(self.as_ref().to_glib_none().0);
        }
    }

    fn extra_timeout(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "extra-timeout")
    }

    fn set_extra_timeout(&self, extra_timeout: u32) {
        glib::ObjectExt::set_property(self.as_ref(), "extra-timeout", &extra_timeout)
    }

    fn is_timeout_always_visible(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "timeout-always-visible")
    }

    fn set_timeout_always_visible(&self, timeout_always_visible: bool) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "timeout-always-visible",
            &timeout_always_visible,
        )
    }

    fn connect_extra_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_extra_timeout_trampoline<
            P: IsA<RTSPSession>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPSession,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPSession::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::extra-timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_extra_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<
            P: IsA<RTSPSession>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPSession,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPSession::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_timeout_always_visible_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_always_visible_trampoline<
            P: IsA<RTSPSession>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPSession,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPSession::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout-always-visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeout_always_visible_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
