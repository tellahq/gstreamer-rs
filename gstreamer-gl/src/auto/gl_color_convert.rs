// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::GLContext;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstGLColorConvert")]
    pub struct GLColorConvert(Object<ffi::GstGLColorConvert, ffi::GstGLColorConvertClass>) @extends gst::Object;

    match fn {
        type_ => || ffi::gst_gl_color_convert_get_type(),
    }
}

impl GLColorConvert {
    #[doc(alias = "gst_gl_color_convert_new")]
    pub fn new(context: &impl IsA<GLContext>) -> GLColorConvert {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_gl_color_convert_new(
                context.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_color_convert_set_caps")]
    pub fn set_caps(
        &self,
        in_caps: &gst::Caps,
        out_caps: &gst::Caps,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_color_convert_set_caps(
                    self.to_glib_none().0,
                    in_caps.to_glib_none().0,
                    out_caps.to_glib_none().0
                ),
                "Failed to set caps"
            )
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_gl_color_convert_swizzle_shader_string")]
    pub fn swizzle_shader_string(context: &impl IsA<GLContext>) -> glib::GString {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_gl_color_convert_swizzle_shader_string(
                context.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_color_convert_transform_caps")]
    pub fn transform_caps(
        context: &impl IsA<GLContext>,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: &gst::Caps,
    ) -> gst::Caps {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_gl_color_convert_transform_caps(
                context.as_ref().to_glib_none().0,
                direction.into_glib(),
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_gl_color_convert_yuv_to_rgb_shader_string")]
    pub fn yuv_to_rgb_shader_string(context: &impl IsA<GLContext>) -> glib::GString {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_gl_color_convert_yuv_to_rgb_shader_string(
                context.as_ref().to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for GLColorConvert {}
unsafe impl Sync for GLColorConvert {}
