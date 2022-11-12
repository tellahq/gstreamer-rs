// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;

bitflags! {
    #[doc(alias = "GstRTPBufferFlags")]
    pub struct RTPBufferFlags: u32 {
        #[doc(alias = "GST_RTP_BUFFER_FLAG_RETRANSMISSION")]
        const RETRANSMISSION = ffi::GST_RTP_BUFFER_FLAG_RETRANSMISSION as _;
        #[doc(alias = "GST_RTP_BUFFER_FLAG_REDUNDANT")]
        const REDUNDANT = ffi::GST_RTP_BUFFER_FLAG_REDUNDANT as _;
    }
}

#[doc(hidden)]
impl IntoGlib for RTPBufferFlags {
    type GlibType = ffi::GstRTPBufferFlags;

    fn into_glib(self) -> ffi::GstRTPBufferFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTPBufferFlags> for RTPBufferFlags {
    unsafe fn from_glib(value: ffi::GstRTPBufferFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RTPBufferFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtp_buffer_flags_get_type()) }
    }
}

impl glib::value::ValueType for RTPBufferFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTPBufferFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RTPBufferFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    #[doc(alias = "GstRTPBufferMapFlags")]
    pub struct RTPBufferMapFlags: u32 {
        #[doc(alias = "GST_RTP_BUFFER_MAP_FLAG_SKIP_PADDING")]
        const SKIP_PADDING = ffi::GST_RTP_BUFFER_MAP_FLAG_SKIP_PADDING as _;
    }
}

#[doc(hidden)]
impl IntoGlib for RTPBufferMapFlags {
    type GlibType = ffi::GstRTPBufferMapFlags;

    fn into_glib(self) -> ffi::GstRTPBufferMapFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTPBufferMapFlags> for RTPBufferMapFlags {
    unsafe fn from_glib(value: ffi::GstRTPBufferMapFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for RTPBufferMapFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtp_buffer_map_flags_get_type()) }
    }
}

impl glib::value::ValueType for RTPBufferMapFlags {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for RTPBufferMapFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for RTPBufferMapFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
bitflags! {
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "GstRTPHeaderExtensionDirection")]
    pub struct RTPHeaderExtensionDirection: u32 {
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_DIRECTION_INACTIVE")]
        const INACTIVE = ffi::GST_RTP_HEADER_EXTENSION_DIRECTION_INACTIVE as _;
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_DIRECTION_SENDONLY")]
        const SENDONLY = ffi::GST_RTP_HEADER_EXTENSION_DIRECTION_SENDONLY as _;
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_DIRECTION_RECVONLY")]
        const RECVONLY = ffi::GST_RTP_HEADER_EXTENSION_DIRECTION_RECVONLY as _;
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_DIRECTION_SENDRECV")]
        const SENDRECV = ffi::GST_RTP_HEADER_EXTENSION_DIRECTION_SENDRECV as _;
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_DIRECTION_INHERITED")]
        const INHERITED = ffi::GST_RTP_HEADER_EXTENSION_DIRECTION_INHERITED as _;
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl IntoGlib for RTPHeaderExtensionDirection {
    type GlibType = ffi::GstRTPHeaderExtensionDirection;

    fn into_glib(self) -> ffi::GstRTPHeaderExtensionDirection {
        self.bits()
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GstRTPHeaderExtensionDirection> for RTPHeaderExtensionDirection {
    unsafe fn from_glib(value: ffi::GstRTPHeaderExtensionDirection) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl StaticType for RTPHeaderExtensionDirection {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtp_header_extension_direction_get_type()) }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for RTPHeaderExtensionDirection {
    type Type = Self;
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
unsafe impl<'a> FromValue<'a> for RTPHeaderExtensionDirection {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl ToValue for RTPHeaderExtensionDirection {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
bitflags! {
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "GstRTPHeaderExtensionFlags")]
    pub struct RTPHeaderExtensionFlags: u32 {
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_ONE_BYTE")]
        const ONE_BYTE = ffi::GST_RTP_HEADER_EXTENSION_ONE_BYTE as _;
        #[doc(alias = "GST_RTP_HEADER_EXTENSION_TWO_BYTE")]
        const TWO_BYTE = ffi::GST_RTP_HEADER_EXTENSION_TWO_BYTE as _;
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl IntoGlib for RTPHeaderExtensionFlags {
    type GlibType = ffi::GstRTPHeaderExtensionFlags;

    fn into_glib(self) -> ffi::GstRTPHeaderExtensionFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GstRTPHeaderExtensionFlags> for RTPHeaderExtensionFlags {
    unsafe fn from_glib(value: ffi::GstRTPHeaderExtensionFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl StaticType for RTPHeaderExtensionFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtp_header_extension_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for RTPHeaderExtensionFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
unsafe impl<'a> FromValue<'a> for RTPHeaderExtensionFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl ToValue for RTPHeaderExtensionFlags {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
