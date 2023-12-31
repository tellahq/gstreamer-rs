// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
mod rtp_base_payload;

mod rtp_base_depayload;

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
mod rtp_header_extension;

pub mod prelude {
    #[doc(hidden)]
    pub use gst::subclass::prelude::*;

    pub use super::rtp_base_depayload::{RTPBaseDepayloadImpl, RTPBaseDepayloadImplExt};
    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub use super::rtp_base_payload::{RTPBasePayloadImpl, RTPBasePayloadImplExt};
    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub use super::rtp_header_extension::{RTPHeaderExtensionImpl, RTPHeaderExtensionImplExt};
}
