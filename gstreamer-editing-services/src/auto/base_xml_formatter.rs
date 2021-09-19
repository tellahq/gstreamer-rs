// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Extractable;
use crate::Formatter;

glib::wrapper! {
    #[doc(alias = "GESBaseXmlFormatter")]
    pub struct BaseXmlFormatter(Object<ffi::GESBaseXmlFormatter, ffi::GESBaseXmlFormatterClass>) @extends Formatter, @implements Extractable;

    match fn {
        type_ => || ffi::ges_base_xml_formatter_get_type(),
    }
}

impl BaseXmlFormatter {}

pub const NONE_BASE_XML_FORMATTER: Option<&BaseXmlFormatter> = None;
