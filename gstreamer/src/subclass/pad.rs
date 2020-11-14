// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_sys;

use glib;
use glib::translate::*;

use glib::subclass::prelude::*;

use Pad;

pub trait PadImpl: PadImplExt + ObjectImpl + Send + Sync {
    fn linked(&self, pad: &Pad, peer: &Pad) {
        self.parent_linked(pad, peer)
    }

    fn unlinked(&self, pad: &Pad, peer: &Pad) {
        self.parent_unlinked(pad, peer)
    }
}

pub trait PadImplExt {
    fn parent_linked(&self, pad: &Pad, peer: &Pad);

    fn parent_unlinked(&self, pad: &Pad, peer: &Pad);
}

impl<T: PadImpl> PadImplExt for T {
    fn parent_linked(&self, pad: &Pad, peer: &Pad) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstPadClass;

            (*parent_class)
                .linked
                .map(|f| f(pad.to_glib_none().0, peer.to_glib_none().0))
                .unwrap_or(())
        }
    }

    fn parent_unlinked(&self, pad: &Pad, peer: &Pad) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstPadClass;

            (*parent_class)
                .unlinked
                .map(|f| f(pad.to_glib_none().0, peer.to_glib_none().0))
                .unwrap_or(())
        }
    }
}

unsafe impl<T: PadImpl> IsSubclassable<T> for Pad {
    fn override_vfuncs(klass: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::override_vfuncs(klass);
        let klass = klass.as_mut();
        klass.linked = Some(pad_linked::<T>);
        klass.unlinked = Some(pad_unlinked::<T>);
    }
}

unsafe extern "C" fn pad_linked<T: PadImpl>(ptr: *mut gst_sys::GstPad, peer: *mut gst_sys::GstPad) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Pad> = from_glib_borrow(ptr);

    imp.linked(&wrap, &from_glib_borrow(peer))
}

unsafe extern "C" fn pad_unlinked<T: PadImpl>(
    ptr: *mut gst_sys::GstPad,
    peer: *mut gst_sys::GstPad,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Pad> = from_glib_borrow(ptr);

    imp.unlinked(&wrap, &from_glib_borrow(peer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use glib;
    use glib::subclass;
    use std::sync::atomic;

    pub mod imp {
        use super::*;

        pub struct TestPad {
            pub(super) linked: atomic::AtomicBool,
            pub(super) unlinked: atomic::AtomicBool,
        }

        impl ObjectSubclass for TestPad {
            const NAME: &'static str = "TestPad";
            type Type = super::TestPad;
            type ParentType = ::Pad;
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib_object_subclass!();

            fn new() -> Self {
                Self {
                    linked: atomic::AtomicBool::new(false),
                    unlinked: atomic::AtomicBool::new(false),
                }
            }
        }

        impl ObjectImpl for TestPad {}

        impl PadImpl for TestPad {
            fn linked(&self, pad: &Pad, peer: &Pad) {
                self.linked.store(true, atomic::Ordering::SeqCst);
                self.parent_linked(pad, peer)
            }

            fn unlinked(&self, pad: &Pad, peer: &Pad) {
                self.unlinked.store(true, atomic::Ordering::SeqCst);
                self.parent_unlinked(pad, peer)
            }
        }
    }

    glib_wrapper! {
        pub struct TestPad(ObjectSubclass<imp::TestPad>) @extends ::Pad, ::Object;
    }

    unsafe impl Send for TestPad {}
    unsafe impl Sync for TestPad {}

    impl TestPad {
        pub fn new(name: &str, direction: ::PadDirection) -> Self {
            glib::Object::new(
                TestPad::static_type(),
                &[("name", &name), ("direction", &direction)],
            )
            .unwrap()
            .downcast::<Self>()
            .unwrap()
        }
    }

    #[test]
    fn test_pad_subclass() {
        ::init().unwrap();

        let pad = TestPad::new("test", ::PadDirection::Src);

        assert_eq!(pad.get_name(), "test");

        let otherpad = ::Pad::new(Some("other-test"), ::PadDirection::Sink);
        pad.link(&otherpad).unwrap();
        pad.unlink(&otherpad).unwrap();

        let imp = imp::TestPad::from_instance(&pad);
        assert!(imp.linked.load(atomic::Ordering::SeqCst));
        assert!(imp.unlinked.load(atomic::Ordering::SeqCst));
    }
}
