// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::PlayerSignalDispatcher;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstPlayerGMainContextSignalDispatcher")]
    pub struct PlayerGMainContextSignalDispatcher(Object<ffi::GstPlayerGMainContextSignalDispatcher, ffi::GstPlayerGMainContextSignalDispatcherClass>) @implements PlayerSignalDispatcher;

    match fn {
        type_ => || ffi::gst_player_g_main_context_signal_dispatcher_get_type(),
    }
}

impl PlayerGMainContextSignalDispatcher {
    #[doc(alias = "application-context")]
    pub fn application_context(&self) -> Option<glib::MainContext> {
        ObjectExt::property(self, "application-context")
    }
}

unsafe impl Send for PlayerGMainContextSignalDispatcher {}
unsafe impl Sync for PlayerGMainContextSignalDispatcher {}
