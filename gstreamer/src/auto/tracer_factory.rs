// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Object;
use crate::PluginFeature;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstTracerFactory")]
    pub struct TracerFactory(Object<ffi::GstTracerFactory, ffi::GstTracerFactoryClass>) @extends PluginFeature, Object;

    match fn {
        type_ => || ffi::gst_tracer_factory_get_type(),
    }
}

impl TracerFactory {
    #[doc(alias = "gst_tracer_factory_get_tracer_type")]
    #[doc(alias = "get_tracer_type")]
    pub fn tracer_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gst_tracer_factory_get_tracer_type(
                self.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for TracerFactory {}
unsafe impl Sync for TracerFactory {}
