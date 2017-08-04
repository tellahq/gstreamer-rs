// This file was generated by gir (9f70278) from gir-files (???)
// DO NOT EDIT

use Element;
use Object;
use TagList;
use TagMergeMode;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TagSetter(Object<ffi::GstTagSetter>): Element, Object;

    match fn {
        get_type => || ffi::gst_tag_setter_get_type(),
    }
}

unsafe impl Send for TagSetter {}
unsafe impl Sync for TagSetter {}

pub trait TagSetterExt {
    //fn add_tag_valist(&self, mode: TagMergeMode, tag: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //fn add_tag_valist_values(&self, mode: TagMergeMode, tag: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //fn add_tag_value(&self, mode: TagMergeMode, tag: &str, value: /*Ignored*/&glib::Value);

    //fn add_tag_values(&self, mode: TagMergeMode, tag: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    //fn add_tags(&self, mode: TagMergeMode, tag: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_tag_list(&self) -> Option<TagList>;

    fn get_tag_merge_mode(&self) -> TagMergeMode;

    fn merge_tags(&self, list: &TagList, mode: TagMergeMode);

    fn reset_tags(&self);

    fn set_tag_merge_mode(&self, mode: TagMergeMode);
}

impl<O: IsA<TagSetter>> TagSetterExt for O {
    //fn add_tag_valist(&self, mode: TagMergeMode, tag: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gst_tag_setter_add_tag_valist() }
    //}

    //fn add_tag_valist_values(&self, mode: TagMergeMode, tag: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gst_tag_setter_add_tag_valist_values() }
    //}

    //fn add_tag_value(&self, mode: TagMergeMode, tag: &str, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call ffi::gst_tag_setter_add_tag_value() }
    //}

    //fn add_tag_values(&self, mode: TagMergeMode, tag: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_tag_setter_add_tag_values() }
    //}

    //fn add_tags(&self, mode: TagMergeMode, tag: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_tag_setter_add_tags() }
    //}

    fn get_tag_list(&self) -> Option<TagList> {
        unsafe {
            from_glib_none(ffi::gst_tag_setter_get_tag_list(self.to_glib_none().0))
        }
    }

    fn get_tag_merge_mode(&self) -> TagMergeMode {
        unsafe {
            from_glib(ffi::gst_tag_setter_get_tag_merge_mode(self.to_glib_none().0))
        }
    }

    fn merge_tags(&self, list: &TagList, mode: TagMergeMode) {
        unsafe {
            ffi::gst_tag_setter_merge_tags(self.to_glib_none().0, list.to_glib_none().0, mode.to_glib());
        }
    }

    fn reset_tags(&self) {
        unsafe {
            ffi::gst_tag_setter_reset_tags(self.to_glib_none().0);
        }
    }

    fn set_tag_merge_mode(&self, mode: TagMergeMode) {
        unsafe {
            ffi::gst_tag_setter_set_tag_merge_mode(self.to_glib_none().0, mode.to_glib());
        }
    }
}
