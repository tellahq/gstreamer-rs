// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::prelude::*;
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
use glib::{
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstAudioAggregatorPad")]
    pub struct AudioAggregatorPad(Object<ffi::GstAudioAggregatorPad, ffi::GstAudioAggregatorPadClass>) @extends gst_base::AggregatorPad, gst::Object;

    match fn {
        type_ => || ffi::gst_audio_aggregator_pad_get_type(),
    }
}

impl AudioAggregatorPad {
    pub const NONE: Option<&'static AudioAggregatorPad> = None;
}

unsafe impl Send for AudioAggregatorPad {}
unsafe impl Sync for AudioAggregatorPad {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AudioAggregatorPad>> Sealed for T {}
}

pub trait AudioAggregatorPadExt: IsA<AudioAggregatorPad> + sealed::Sealed + 'static {
    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "qos-messages")]
    fn is_qos_messages(&self) -> bool {
        ObjectExt::property(self.as_ref(), "qos-messages")
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "qos-messages")]
    fn set_qos_messages(&self, qos_messages: bool) {
        ObjectExt::set_property(self.as_ref(), "qos-messages", qos_messages)
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "qos-messages")]
    fn connect_qos_messages_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_messages_trampoline<
            P: IsA<AudioAggregatorPad>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioAggregatorPad,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioAggregatorPad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::qos-messages\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_qos_messages_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<AudioAggregatorPad>> AudioAggregatorPadExt for O {}
