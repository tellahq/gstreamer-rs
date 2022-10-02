// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! This modules gathers GStreamer's formatted value concepts together.
//!
//! GStreamer uses formatted values to differentiate value units in some APIs.
//! In C this is done by qualifying an integer value by a companion enum
//! [`GstFormat`]. In Rust, most APIs can use a specific type for each format.
//! Each format type embeds the actual value using the new type pattern.
//!
//! # Specific Formatted Values
//!
//! Examples of specific formatted values include [`ClockTime`], [`Buffers`], etc.
//! These types represent both the quantity and the unit making it possible for Rust
//! to perform runtime and, to a certain extent, compile time invariants enforcement.
//!
//! Specific formatted values are also guaranteed to always represent a valid value.
//! For instance:
//!
//! - [`Percent`] only allows values in the range [0, 1_000_000].
//! - [`ClockTime`] can use all `u64` values except `u64::MAX` which is reserved by
//!   the C constant `GST_CLOCK_TIME_NONE`.
//!
//! ## Examples
//!
//! ### Querying the pipeline for a time position
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::prelude::ElementExtManual;
//! # gst::init();
//! # let pipeline = gst::Pipeline::new(None);
//! let res = pipeline.query_position::<gst::ClockTime>();
//! ```
//!
//! ## Seeking to a specific time position
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::prelude::ElementExtManual;
//! # gst::init();
//! # let pipeline = gst::Pipeline::new(None);
//! # let seek_flags = gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT;
//! let seek_pos = gst::ClockTime::from_seconds(10);
//! let res = pipeline.seek_simple(seek_flags, seek_pos);
//! ```
//!
//! ### Downcasting a `Segment` for specific formatted value use
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::FormattedValue;
//! # gst::init();
//! # let segment = gst::FormattedSegment::<gst::ClockTime>::new().upcast();
//! // Downcasting the generic `segment` for `gst::ClockTime` use.
//! let time_segment = segment.downcast_ref::<gst::ClockTime>().expect("time segment");
//! // Setters and getters conform to `gst::ClockTime`.
//! // This is enforced at compilation time.
//! let start = time_segment.start();
//! assert_eq!(start.format(), gst::Format::Time);
//! ```
//!
//! ### Displaying a formatted value
//!
//! Formatted values implement the [`Display`] trait which allows getting
//! human readable representations.
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::prelude::Displayable;
//! let time = gst::ClockTime::from_nseconds(45_834_908_569_837);
//!
//! assert_eq!(format!("{}", time), "12:43:54.908569837");
//! assert_eq!(format!("{:.0}", time), "12:43:54");
//! ```
//!
//! ## Some operations available on specific formatted values
//!
//! ```
//! # use gstreamer as gst;
//! let cur_pos = gst::ClockTime::ZERO;
//!
//! // All four arithmetic operations can be used:
//! let fwd = cur_pos + 2 * gst::ClockTime::SECOND / 3 - gst::ClockTime::MSECOND;
//!
//! // Examples of operations which make sure not to overflow:
//! let bwd = cur_pos.saturating_sub(2 * gst::ClockTime::SECOND);
//! let further = cur_pos.checked_mul(2).expect("Overflowed");
//!
//! // Specific formatted values can be compared:
//! assert!(fwd > bwd);
//! assert_ne!(fwd, cur_pos);
//!
//! # fn next() -> gst::ClockTime { gst::ClockTime::ZERO };
//! // Use `gst::ClockTime::MAX` for the maximum valid value:
//! let mut min_pos = gst::ClockTime::MAX;
//! for _ in 0..4 {
//!     min_pos = min_pos.min(next());
//! }
//!
//! // And `gst::ClockTime::ZERO` for the minimum value:
//! let mut max_pos = gst::ClockTime::ZERO;
//! for _ in 0..4 {
//!     max_pos = max_pos.max(next());
//! }
//!
//! // Specific formatted values implement the `MulDiv` trait:
//! # use gst::prelude::MulDiv;
//! # let (samples, rate) = (1024u64, 48000u64);
//! let duration = samples
//!     .mul_div_round(*gst::ClockTime::SECOND, rate)
//!     .map(gst::ClockTime::from_nseconds);
//! ```
//!
//! ## Types in operations
//!
//! Additions and substractions are available with the specific formatted value type
//! as both left and right hand side operands.
//!
//! On the other hand, multiplications are only available with plain integers.
//! This is because multiplying a `ClockTime` by a `ClockTime` would result in
//! `ClockTime²`, whereas a `u64 * ClockTime` (or `ClockTime * u64`) still
//! results in `ClockTime`.
//!
//! Divisions are available with both the specific formatted value and plain
//! integers as right hand side operands. The difference is that
//! `ClockTime / ClockTime` results in `u64` and `ClockTime / u64` results in
//! `ClockTime`.
//!
//! # Optional specific formatted values
//!
//! Optional specific formatted values are represented as a standard Rust
//! `Option<F>`. This departs from the C APIs which uses a sentinel that must
//! be checked in order to figure out whether the value is defined.
//!
//! Besides giving access to the usual `Option` features, this ensures the APIs
//! enforce mandatory or optional variants whenever possible.
//!
//! Note: for each specific formatted value `F`, the constant `F::NONE` is defined
//! as a shortcut for `Option::<F>::None`. For `gst::ClockTime`, this constant is
//! equivalent to the C constant `GST_CLOCK_TIME_NONE`.
//!
//! ## Examples
//!
//! ### Building a seek `Event` with undefined `stop` time
//!
//! ```
//! # use gstreamer as gst;
//! # gst::init();
//! # let seek_flags = gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT;
//! let seek_evt = gst::event::Seek::new(
//!     1.0f64,
//!     seek_flags,
//!     gst::SeekType::Set,
//!     10 * gst::ClockTime::SECOND, // start at 10s
//!     gst::SeekType::Set,
//!     gst::ClockTime::NONE,        // stop is undefined
//! );
//! ```
//!
//! ### Displaying an optional formatted value
//!
//! Optional formatted values can take advantage of the [`Display`] implementation
//! of the base specific formatted value. We have to workaround the [orphan rule]
//! that forbids the implementation of [`Display`] for `Option<FormattedValue>`
//! though. This is why displaying an optional formatted value necessitates calling
//! [`display()`].
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::prelude::Displayable;
//! let opt_time = Some(45_834_908_569_837 * gst::ClockTime::NSECOND);
//!
//! assert_eq!(format!("{}", opt_time.display()), "12:43:54.908569837");
//! assert_eq!(format!("{:.0}", opt_time.display()), "12:43:54");
//! assert_eq!(format!("{:.0}", gst::ClockTime::NONE.display()), "--:--:--");
//! ```
//!
//! ### Some operations available on optional formatted values
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::prelude::*;
//! let pts = Some(gst::ClockTime::ZERO);
//! assert!(pts.is_some());
//!
//! // All four arithmetic operations can be used. Ex.:
//! let fwd = pts.opt_add(2 * gst::ClockTime::SECOND);
//! // `pts` is defined, so `fwd` will contain the addition result in `Some`,
//! assert!(fwd.is_some());
//! // otherwise `fwd` would be `None`.
//!
//! // Examples of operations which make sure not to overflow:
//! let bwd = pts.opt_saturating_sub(2 * gst::ClockTime::SECOND);
//! let further = pts.opt_checked_mul(2).expect("Overflowed");
//!
//! // Optional specific formatted values can be compared:
//! assert_eq!(fwd.opt_gt(bwd), Some(true));
//! assert_ne!(fwd, pts);
//! assert_eq!(fwd.opt_min(bwd), bwd);
//!
//! // Optional specific formatted values operations also apply to non-optional values:
//! assert_eq!(fwd.opt_lt(gst::ClockTime::SECOND), Some(false));
//! assert_eq!(gst::ClockTime::SECOND.opt_lt(fwd), Some(true));
//!
//! // Comparing a defined values to an undefined value results in `None`:
//! assert_eq!(bwd.opt_gt(gst::ClockTime::NONE), None);
//! assert_eq!(gst::ClockTime::ZERO.opt_lt(gst::ClockTime::NONE), None);
//! ```
//!
//! # Signed formatted values
//!
//! Some APIs can return a signed formatted value. See [`Segment::to_running_time_full`]
//! for an example. In Rust, we use the [`Signed`] enum wrapper around the actual
//! formatted value.
//!
//! For each signed specific formatted value `F`, the constants `F::MIN_SIGNED` and
//! `F::MAX_SIGNED` represent the minimum and maximum signed values for `F`.
//!
//! ## Examples
//!
//! ### Handling a signed formatted value
//!
//! ```
//! # use gstreamer as gst;
//! # gst::init();
//! # let segment = gst::FormattedSegment::<gst::ClockTime>::new();
//! use gst::Signed::*;
//! match segment.to_running_time_full(2 * gst::ClockTime::SECOND) {
//!     Some(Positive(pos_rtime)) => println!("positive rtime {}", pos_rtime),
//!     Some(Negative(pos_rtime)) => println!("negative rtime {}", pos_rtime),
//!     None => println!("undefined rtime"),
//! }
//! ```
//!
//! ### Converting a formatted value into a signed formatted value
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::prelude::UnsignedIntoSigned;
//! let pos = gst::ClockTime::SECOND;
//!
//! let positive_one_sec = pos.into_positive();
//! assert!(positive_one_sec.is_positive());
//!
//! let negative_one_sec = pos.into_negative();
//! assert!(negative_one_sec.is_negative());
//! ```
//!
//! ### Handling one sign only
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::prelude::UnsignedIntoSigned;
//! # struct NegativeError;
//! let p_one_sec = gst::ClockTime::SECOND.into_positive();
//!
//! let one_sec = p_one_sec.positive().expect("positive");
//! let one_sec_or_zero = p_one_sec.positive().unwrap_or(gst::ClockTime::ZERO);
//!
//! let one_sec_or_err = p_one_sec.positive_or(NegativeError);
//! let one_sec_or_else_err = p_one_sec.positive_or_else(|value| {
//!     println!("{} is negative", value);
//!     NegativeError
//! });
//! ```
//!
//! ### Displaying a signed formatted value
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::prelude::Displayable;
//! # gst::init();
//! # let mut segment = gst::FormattedSegment::<gst::ClockTime>::new();
//! # segment.set_start(10 * gst::ClockTime::SECOND);
//! let start = segment.start().unwrap();
//! assert_eq!(format!("{:.0}", start), "0:00:10");
//!
//! let p_rtime = segment.to_running_time_full(20 * gst::ClockTime::SECOND);
//! // Use `display()` with optional signed values.
//! assert_eq!(format!("{:.0}", p_rtime.display()), "+0:00:10");
//!
//! let p_rtime = segment.to_running_time_full(gst::ClockTime::ZERO);
//! assert_eq!(format!("{:.0}", p_rtime.display()), "-0:00:10");
//!
//! let p_rtime = segment.to_running_time_full(gst::ClockTime::NONE);
//! assert_eq!(format!("{:.0}", p_rtime.display()), "--:--:--");
//! ```
//!
//! ## Some operations available for signed formatted values
//!
//! All the operations available for formatted values can be used with
//! signed formatted values.
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::prelude::UnsignedIntoSigned;
//! let p_one_sec = gst::ClockTime::SECOND.into_positive();
//! let p_two_sec = (2 * gst::ClockTime::SECOND).into_positive();
//! let n_one_sec = gst::ClockTime::SECOND.into_negative();
//!
//! assert_eq!(p_one_sec + p_one_sec, p_two_sec);
//! assert_eq!(p_two_sec - p_one_sec, p_one_sec);
//! assert_eq!(gst::ClockTime::ZERO - p_one_sec, n_one_sec);
//! assert_eq!(p_one_sec * 2u64, p_two_sec);
//! assert_eq!(n_one_sec * -1i64, p_one_sec);
//! assert_eq!(p_two_sec / 2u64, p_one_sec);
//! assert_eq!(p_two_sec / p_one_sec, 2);
//!
//! // Examples of operations which make sure not to overflow:
//! assert_eq!(p_one_sec.saturating_sub(p_two_sec), n_one_sec);
//! assert_eq!(p_one_sec.checked_mul(2), Some(p_two_sec));
//!
//! // Signed formatted values can be compared:
//! assert!(p_one_sec > n_one_sec);
//!
//! # fn next() -> gst::Signed<gst::ClockTime> { gst::ClockTime::ZERO.into_positive() };
//! // Use `gst::ClockTime::MAX_SIGNED` for the maximum valid signed value:
//! let mut min_signed_pos = gst::ClockTime::MAX_SIGNED;
//! for _ in 0..4 {
//!     min_signed_pos = min_signed_pos.min(next());
//! }
//!
//! // And `gst::ClockTime::MIN_SIGNED` for the minimum valid signed value:
//! let mut max_signed_pos = gst::ClockTime::MIN_SIGNED;
//! for _ in 0..4 {
//!     max_signed_pos = max_signed_pos.max(next());
//! }
//!
//! // Signed formatted values implement the `MulDiv` trait:
//! # use gst::prelude::MulDiv;
//! # let rate = 48000u64;
//! let samples = gst::format::Default(1024).into_negative();
//! let duration = samples
//!     .mul_div_round(*gst::ClockTime::SECOND, rate)
//!     .map(|signed_default| {
//!         let signed_u64 = signed_default.into_inner_signed();
//!         gst::Signed::<gst::ClockTime>::from_nseconds(signed_u64)
//!     })
//!     .unwrap();
//! assert!(duration.is_negative());
//! ```
//!
//! ### Some operations available for optional signed formatted values
//!
//! All the operations available for optional formatted values can be used
//! with signed formatted values.
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::prelude::*;
//! let p_one_sec = gst::ClockTime::SECOND.into_positive();
//! let p_two_sec = (2 * gst::ClockTime::SECOND).into_positive();
//! let n_one_sec = gst::ClockTime::SECOND.into_negative();
//!
//! // Signed `ClockTime` addition with optional and non-optional operands.
//! assert_eq!(Some(p_one_sec).opt_add(p_one_sec), Some(p_two_sec));
//! assert_eq!(p_two_sec.opt_add(Some(n_one_sec)), Some(p_one_sec));
//!
//! // This can also be used with unsigned formatted values.
//! assert_eq!(Some(p_one_sec).opt_add(gst::ClockTime::SECOND), Some(p_two_sec));
//!
//! // Etc...
//! ```
//!
//! # Generic Formatted Values
//!
//! Sometimes, generic code can't assume a specific format will be used. For such
//! use cases, the [`GenericFormattedValue`] enum makes it possible to select
//! the appropriate behaviour at runtime.
//!
//! Most variants embed an optional specific formatted value.
//!
//! ## Example
//!
//! ### Generic handling of the position from a `SegmentDone` event
//!
//! ```
//! # use gstreamer as gst;
//! # use gst::prelude::{Displayable, ElementExtManual};
//! # gst::init();
//! # let event = gst::event::SegmentDone::new(gst::format::Buffers(512));
//! if let gst::EventView::SegmentDone(seg_done_evt) = event.view() {
//!     use gst::GenericFormattedValue::*;
//!     match seg_done_evt.get() {
//!         Buffers(buffers) => println!("Segment done @ {}", buffers.display()),
//!         Bytes(bytes) => println!("Segment done @ {}", bytes.display()),
//!         Time(time) => println!("Segment done @ {}", time.display()),
//!         other => println!("Unexpected format for Segment done position {other:?}"),
//!     }
//! }
//! ```
//!
//! [`GstFormat`]: https://gstreamer.freedesktop.org/documentation/gstreamer/gstformat.html?gi-language=c
//! [`ClockTime`]: struct.ClockTime.html
//! [`Buffers`]: struct.Buffers.html
//! [`Percent`]: struct.Percent.html
//! [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
//! [`display()`]: ../prelude/trait.Displayable.html
//! [orphan rule]: https://doc.rust-lang.org/book/ch10-02-traits.html?highlight=orphan#implementing-a-trait-on-a-type
//! [`Segment::to_running_time_full`]: ../struct.FormattedSegment.html#method.to_running_time_full
//! [`Signed`]: enum.Signed.html
//! [`GenericFormattedValue`]: generic/enum.GenericFormattedValue.html

use thiserror::Error;

#[macro_use]
mod macros;

mod clock_time;
pub use clock_time::ClockTime;
#[cfg(feature = "serde")]
mod clock_time_serde;

mod compatible;
pub use compatible::*;

mod generic;
pub use generic::*;

mod signed;
pub use signed::*;

mod specific;
pub use specific::*;

mod undefined;
pub use undefined::*;

use crate::Format;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
#[error("invalid formatted value format {:?}", .0)]
pub struct FormattedValueError(Format);

pub trait FormattedValue: Copy + Clone + Sized + Into<GenericFormattedValue> + 'static {
    // rustdoc-stripper-ignore-next
    /// Type which allows building a `FormattedValue` of this format from any raw value.
    type FullRange: FormattedValueFullRange + From<Self>;

    #[doc(alias = "get_default_format")]
    fn default_format() -> Format;

    #[doc(alias = "get_format")]
    fn format(&self) -> Format;

    // rustdoc-stripper-ignore-next
    /// Returns `true` if this `FormattedValue` represents a defined value.
    fn is_some(&self) -> bool;

    // rustdoc-stripper-ignore-next
    /// Returns `true` if this `FormattedValue` represents an undefined value.
    fn is_none(&self) -> bool {
        !self.is_some()
    }

    unsafe fn into_raw_value(self) -> i64;
}

// rustdoc-stripper-ignore-next
/// A [`FormattedValue`] which can be built from any raw value.
///
/// # Examples:
///
/// - `GenericFormattedValue` is the `FormattedValueFullRange` type for `GenericFormattedValue`.
/// - `Undefined` is the `FormattedValueFullRange` type for `Undefined`.
/// - `Option<Percent>` is the `FormattedValueFullRange` type for `Percent`.
pub trait FormattedValueFullRange: FormattedValue + TryFrom<GenericFormattedValue> {
    unsafe fn from_raw(format: Format, value: i64) -> Self;
}

// rustdoc-stripper-ignore-next
/// A trait implemented on the intrinsic type of a `FormattedValue`.
///
/// # Examples
///
/// - `GenericFormattedValue` is the intrinsic type for `GenericFormattedValue`.
/// - `Undefined` is the intrinsic type for `Undefined`.
/// - `Bytes` is the intrinsic type for `Option<Bytes>`.
pub trait FormattedValueIntrinsic: FormattedValue {}

pub trait FormattedValueNoneBuilder: FormattedValueFullRange {
    // rustdoc-stripper-ignore-next
    /// Returns the `None` value for `Self` as a `FullRange` if such a value can be represented.
    ///
    /// - For `SpecificFormattedValue`s, this results in `Option::<FormattedValueIntrinsic>::None`.
    /// - For `GenericFormattedValue`, this can only be obtained using [`Self::none_for_format`]
    ///   because the `None` is an inner value of some of the variants.
    ///
    /// # Panics
    ///
    /// Panics if `Self` is `GenericFormattedValue` in which case, the `Format` must be known.
    fn none() -> Self;

    // rustdoc-stripper-ignore-next
    /// Returns the `None` value for `Self` if such a value can be represented.
    ///
    /// - For `SpecificFormattedValue`s, this is the same as `Self::none()`
    ///   if the `format` matches the `SpecificFormattedValue`'s format.
    /// - For `GenericFormattedValue` this is the variant for the specified `format`,
    ///   initialized with `None` as a value, if the `format` can represent that value.
    ///
    /// # Panics
    ///
    /// Panics if `None` can't be represented by `Self` for `format` or by the requested
    /// `GenericFormattedValue` variant.
    #[track_caller]
    fn none_for_format(format: Format) -> Self {
        skip_assert_initialized!();
        // This is the default impl. `GenericFormattedValue` must override.
        if Self::default_format() != format {
            panic!(
                "Expected: {:?}, requested {format:?}",
                Self::default_format()
            );
        }

        Self::none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::Displayable;

    fn with_compatible_formats<V1, V2>(
        arg1: V1,
        arg2: V2,
    ) -> Result<V2::Original, FormattedValueError>
    where
        V1: FormattedValue,
        V2: CompatibleFormattedValue<V1>,
    {
        skip_assert_initialized!();
        arg2.try_into_checked(arg1)
    }

    #[test]
    fn compatible() {
        assert_eq!(
            with_compatible_formats(ClockTime::ZERO, ClockTime::ZERO),
            Ok(ClockTime::ZERO),
        );
        assert_eq!(
            with_compatible_formats(ClockTime::ZERO, ClockTime::NONE),
            Ok(ClockTime::NONE),
        );
        assert_eq!(
            with_compatible_formats(ClockTime::NONE, ClockTime::ZERO),
            Ok(ClockTime::ZERO),
        );
        assert_eq!(
            with_compatible_formats(
                ClockTime::ZERO,
                GenericFormattedValue::Time(Some(ClockTime::ZERO)),
            ),
            Ok(GenericFormattedValue::Time(Some(ClockTime::ZERO))),
        );
        assert_eq!(
            with_compatible_formats(
                GenericFormattedValue::Time(Some(ClockTime::ZERO)),
                ClockTime::NONE,
            ),
            Ok(ClockTime::NONE),
        );
    }

    #[test]
    fn incompatible() {
        with_compatible_formats(
            ClockTime::ZERO,
            GenericFormattedValue::Buffers(Some(Buffers(42))),
        )
        .unwrap_err();
        with_compatible_formats(
            GenericFormattedValue::Buffers(Some(Buffers(42))),
            ClockTime::NONE,
        )
        .unwrap_err();
    }

    fn with_compatible_explicit<T, V>(arg: V, f: Format) -> Result<V::Original, FormattedValueError>
    where
        T: FormattedValue,
        V: CompatibleFormattedValue<T>,
    {
        skip_assert_initialized!();
        arg.try_into_checked_explicit(f)
    }

    #[test]
    fn compatible_explicit() {
        assert_eq!(
            with_compatible_explicit::<ClockTime, _>(ClockTime::ZERO, Format::Time),
            Ok(ClockTime::ZERO),
        );
        assert_eq!(
            with_compatible_explicit::<ClockTime, _>(ClockTime::NONE, Format::Time),
            Ok(ClockTime::NONE),
        );
        assert_eq!(
            with_compatible_explicit::<ClockTime, _>(ClockTime::ZERO, Format::Time),
            Ok(ClockTime::ZERO),
        );
        assert_eq!(
            with_compatible_explicit::<ClockTime, _>(
                GenericFormattedValue::Time(None),
                Format::Time
            ),
            Ok(GenericFormattedValue::Time(None)),
        );
        assert_eq!(
            with_compatible_explicit::<GenericFormattedValue, _>(ClockTime::NONE, Format::Time),
            Ok(ClockTime::NONE),
        );
    }

    #[test]
    fn incompatible_explicit() {
        with_compatible_explicit::<Buffers, _>(GenericFormattedValue::Time(None), Format::Buffers)
            .unwrap_err();
        with_compatible_explicit::<GenericFormattedValue, _>(Buffers::ZERO, Format::Time)
            .unwrap_err();
        with_compatible_explicit::<GenericFormattedValue, _>(
            GenericFormattedValue::Time(None),
            Format::Buffers,
        )
        .unwrap_err();
    }

    #[test]
    fn none_builder() {
        let ct_none: Option<ClockTime> = Option::<ClockTime>::none();
        assert!(ct_none.is_none());

        let ct_none: Option<ClockTime> = Option::<ClockTime>::none_for_format(Format::Time);
        assert!(ct_none.is_none());

        let gen_ct_none: GenericFormattedValue =
            GenericFormattedValue::none_for_format(Format::Time);
        assert!(gen_ct_none.is_none());

        assert!(ClockTime::ZERO.is_some());
        assert!(!ClockTime::ZERO.is_none());
    }

    #[test]
    #[should_panic]
    fn none_for_inconsistent_format() {
        let _ = Option::<ClockTime>::none_for_format(Format::Percent);
    }

    #[test]
    #[should_panic]
    fn none_for_unsupported_format() {
        let _ = GenericFormattedValue::none_for_format(Format::Undefined);
    }

    #[test]
    fn none_signed_builder() {
        let ct_none: Option<Signed<ClockTime>> = Option::<ClockTime>::none_signed();
        assert!(ct_none.is_none());

        let ct_none: Option<Signed<ClockTime>> =
            Option::<ClockTime>::none_signed_for_format(Format::Time);
        assert!(ct_none.is_none());

        let gen_ct_none: Signed<GenericFormattedValue> =
            GenericFormattedValue::none_signed_for_format(Format::Time);
        assert!(gen_ct_none.abs().is_none());
    }

    #[test]
    fn signed_optional() {
        let ct_1 = Some(ClockTime::SECOND);

        let signed = ct_1.into_positive().unwrap();
        assert_eq!(signed, Signed::Positive(ClockTime::SECOND));
        assert!(signed.is_positive());
        assert_eq!(signed.positive_or(()).unwrap(), ClockTime::SECOND);
        assert_eq!(signed.positive_or_else(|_| ()).unwrap(), ClockTime::SECOND);
        signed.negative_or(()).unwrap_err();
        assert_eq!(
            signed.negative_or_else(|val| val).unwrap_err(),
            ClockTime::SECOND
        );

        let signed = ct_1.into_negative().unwrap();
        assert_eq!(signed, Signed::Negative(ClockTime::SECOND));
        assert!(signed.is_negative());
        assert_eq!(signed.negative_or(()).unwrap(), ClockTime::SECOND);
        assert_eq!(signed.negative_or_else(|_| ()).unwrap(), ClockTime::SECOND);
        signed.positive_or(()).unwrap_err();
        assert_eq!(
            signed.positive_or_else(|val| val).unwrap_err(),
            ClockTime::SECOND
        );

        let ct_none = ClockTime::NONE;
        assert!(ct_none.into_positive().is_none());
        assert!(ct_none.into_negative().is_none());
    }

    #[test]
    fn signed_mandatory() {
        let ct_1 = ClockTime::SECOND;

        let signed = ct_1.into_positive();
        assert_eq!(signed, Signed::Positive(ct_1));
        assert!(signed.is_positive());
        assert_eq!(signed.positive(), Some(ct_1));
        assert!(!signed.is_negative());
        assert!(signed.negative().is_none());

        let signed = ct_1.into_negative();
        assert_eq!(signed, Signed::Negative(ct_1));
        assert!(signed.is_negative());
        assert_eq!(signed.negative(), Some(ct_1));
        assert!(!signed.is_positive());
        assert!(signed.positive().is_none());

        let def = Default(1);

        let signed = def.into_positive();
        assert_eq!(signed, Signed::Positive(def));
        assert!(signed.is_positive());
        assert_eq!(signed.positive(), Some(def));
        assert!(!signed.is_negative());
        assert!(signed.negative().is_none());

        let signed = def.into_negative();
        assert_eq!(signed, Signed::Negative(def));
        assert!(signed.is_negative());
        assert_eq!(signed.negative(), Some(def));
        assert!(!signed.is_positive());
        assert!(signed.positive().is_none());
    }

    #[test]
    fn signed_generic() {
        let ct_1 = GenericFormattedValue::Time(Some(ClockTime::SECOND));
        assert!(ct_1.is_some());

        let signed = ct_1.into_positive();
        assert_eq!(signed, Signed::Positive(ct_1));
        assert!(signed.is_positive());
        assert_eq!(signed.positive(), Some(ct_1));

        let signed = ct_1.into_negative();
        assert_eq!(signed, Signed::Negative(ct_1));
        assert!(signed.is_negative());
        assert_eq!(signed.negative(), Some(ct_1));

        let ct_none = GenericFormattedValue::Time(ClockTime::NONE);
        assert!(ct_none.is_none());

        let signed = ct_none.into_positive();
        assert_eq!(signed, Signed::Positive(ct_none));
        assert!(signed.is_positive());

        let signed = ct_none.into_negative();
        assert_eq!(signed, Signed::Negative(ct_none));
        assert!(signed.is_negative());
    }

    #[test]
    fn signed_roundtrip() {
        let ct_1 = Some(ClockTime::SECOND);
        let raw_ct_1 = unsafe { ct_1.into_raw_value() };

        let signed = unsafe { Option::<ClockTime>::from_raw(Format::Time, raw_ct_1) }
            .into_signed(1)
            .unwrap();
        assert_eq!(signed, Signed::Positive(ClockTime::SECOND));
        assert!(signed.is_positive());

        let signed = unsafe { Option::<ClockTime>::from_raw(Format::Time, raw_ct_1) }
            .into_signed(-1)
            .unwrap();
        assert_eq!(signed, Signed::Negative(ClockTime::SECOND));
        assert!(signed.is_negative());

        let ct_none = ClockTime::NONE;
        let raw_ct_none = unsafe { ct_none.into_raw_value() };

        let signed =
            unsafe { Option::<ClockTime>::from_raw(Format::Time, raw_ct_none) }.into_signed(1);
        assert!(signed.is_none());

        let signed =
            unsafe { Option::<ClockTime>::from_raw(Format::Time, raw_ct_none) }.into_signed(-1);
        assert!(signed.is_none());
    }

    #[test]
    fn display_new_types() {
        let bytes = Bytes(42);
        assert_eq!(&format!("{bytes}"), "42 bytes");
        assert_eq!(&format!("{}", bytes.display()), "42 bytes");

        assert_eq!(&format!("{}", Some(bytes).display()), "42 bytes");
        assert_eq!(&format!("{}", Bytes::NONE.display()), "undef. bytes");

        let gv_1 = GenericFormattedValue::Percent(Some(Percent(42)));
        assert_eq!(&format!("{gv_1}"), "42 %");
        assert_eq!(
            &format!("{}", GenericFormattedValue::Percent(None)),
            "undef. %"
        );
    }

    #[test]
    fn display_signed() {
        let p_bytes = Bytes(42).into_positive();
        assert_eq!(&format!("{p_bytes}"), "+42 bytes");
        assert_eq!(&format!("{}", p_bytes.display()), "+42 bytes");

        let some_p_bytes = Some(p_bytes);
        assert_eq!(&format!("{}", some_p_bytes.display()), "+42 bytes");

        let p_some_bytes = Signed::Positive(Some(Bytes(42)));
        assert_eq!(&format!("{}", p_some_bytes.display()), "+42 bytes");

        let n_bytes = Bytes(42).into_negative();
        assert_eq!(&format!("{n_bytes}"), "-42 bytes");
        assert_eq!(&format!("{}", n_bytes.display()), "-42 bytes");

        let some_n_bytes = Some(n_bytes);
        assert_eq!(&format!("{}", some_n_bytes.display()), "-42 bytes");

        let n_some_bytes = Signed::Negative(Some(Bytes(42)));
        assert_eq!(&format!("{}", n_some_bytes.display()), "-42 bytes");

        let p_none_bytes = Signed::Positive(Bytes::NONE);
        assert_eq!(&format!("{}", p_none_bytes.display()), "undef. bytes");
        let n_none_bytes = Signed::Negative(Bytes::NONE);
        assert_eq!(&format!("{}", n_none_bytes.display()), "undef. bytes");

        let none_s_bytes = Option::<Signed<Bytes>>::None;
        assert_eq!(&format!("{}", none_s_bytes.display()), "undef. bytes");
    }
}