// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Clip;
use crate::Container;
use crate::Extractable;
use crate::MetaContainer;
use crate::SourceClip;
use crate::TimelineElement;
use crate::VideoTestPattern;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GESTestClip")]
    pub struct TestClip(Object<ffi::GESTestClip, ffi::GESTestClipClass>) @extends SourceClip, Clip, Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_test_clip_get_type(),
    }
}

impl TestClip {
    #[doc(alias = "ges_test_clip_new")]
    pub fn new() -> Option<TestClip> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_test_clip_new()) }
    }

    #[doc(alias = "ges_test_clip_new_for_nick")]
    #[doc(alias = "new_for_nick")]
    pub fn for_nick(nick: &str) -> Option<TestClip> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_test_clip_new_for_nick(nick.to_glib_none().0)) }
    }
}

pub const NONE_TEST_CLIP: Option<&TestClip> = None;

pub trait TestClipExt: 'static {
    #[doc(alias = "ges_test_clip_get_frequency")]
    #[doc(alias = "get_frequency")]
    fn frequency(&self) -> f64;

    #[doc(alias = "ges_test_clip_get_volume")]
    #[doc(alias = "get_volume")]
    fn volume(&self) -> f64;

    #[doc(alias = "ges_test_clip_get_vpattern")]
    #[doc(alias = "get_vpattern")]
    fn vpattern(&self) -> VideoTestPattern;

    #[doc(alias = "ges_test_clip_is_muted")]
    fn is_muted(&self) -> bool;

    #[doc(alias = "ges_test_clip_set_frequency")]
    fn set_frequency(&self, freq: f64);

    #[doc(alias = "ges_test_clip_set_mute")]
    fn set_mute(&self, mute: bool);

    #[doc(alias = "ges_test_clip_set_volume")]
    fn set_volume(&self, volume: f64);

    #[doc(alias = "ges_test_clip_set_vpattern")]
    fn set_vpattern(&self, vpattern: VideoTestPattern);

    fn freq(&self) -> f64;

    fn set_freq(&self, freq: f64);

    #[doc(alias = "freq")]
    fn connect_freq_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "mute")]
    fn connect_mute_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "volume")]
    fn connect_volume_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "vpattern")]
    fn connect_vpattern_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TestClip>> TestClipExt for O {
    fn frequency(&self) -> f64 {
        unsafe { ffi::ges_test_clip_get_frequency(self.as_ref().to_glib_none().0) }
    }

    fn volume(&self) -> f64 {
        unsafe { ffi::ges_test_clip_get_volume(self.as_ref().to_glib_none().0) }
    }

    fn vpattern(&self) -> VideoTestPattern {
        unsafe {
            from_glib(ffi::ges_test_clip_get_vpattern(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_muted(&self) -> bool {
        unsafe { from_glib(ffi::ges_test_clip_is_muted(self.as_ref().to_glib_none().0)) }
    }

    fn set_frequency(&self, freq: f64) {
        unsafe {
            ffi::ges_test_clip_set_frequency(self.as_ref().to_glib_none().0, freq);
        }
    }

    fn set_mute(&self, mute: bool) {
        unsafe {
            ffi::ges_test_clip_set_mute(self.as_ref().to_glib_none().0, mute.into_glib());
        }
    }

    fn set_volume(&self, volume: f64) {
        unsafe {
            ffi::ges_test_clip_set_volume(self.as_ref().to_glib_none().0, volume);
        }
    }

    fn set_vpattern(&self, vpattern: VideoTestPattern) {
        unsafe {
            ffi::ges_test_clip_set_vpattern(self.as_ref().to_glib_none().0, vpattern.into_glib());
        }
    }

    fn freq(&self) -> f64 {
        unsafe {
            let mut value = glib::Value::from_type(<f64 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"freq\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `freq` getter")
        }
    }

    fn set_freq(&self, freq: f64) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"freq\0".as_ptr() as *const _,
                freq.to_value().to_glib_none().0,
            );
        }
    }

    fn connect_freq_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_freq_trampoline<P: IsA<TestClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTestClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TestClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::freq\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_freq_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_mute_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mute_trampoline<P: IsA<TestClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTestClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TestClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mute\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mute_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_volume_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_volume_trampoline<P: IsA<TestClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTestClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TestClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::volume\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_volume_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_vpattern_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vpattern_trampoline<P: IsA<TestClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTestClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TestClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vpattern\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vpattern_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
