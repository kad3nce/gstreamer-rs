// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use GLDisplay;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct GLDisplayEGL(Object<ffi::GstGLDisplayEGL, ffi::GstGLDisplayEGLClass>): [
        GLDisplay,
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_gl_display_egl_get_type(),
    }
}

impl GLDisplayEGL {
    pub fn new() -> GLDisplayEGL {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_gl_display_egl_new())
        }
    }

    //pub fn new_with_egl_display<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(display: P) -> GLDisplayEGL {
    //    unsafe { TODO: call ffi::gst_gl_display_egl_new_with_egl_display() }
    //}

    //pub fn get_from_native(type_: GLDisplayType, display: /*Unimplemented*/Fundamental: UIntPtr) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::gst_gl_display_egl_get_from_native() }
    //}
}

impl Default for GLDisplayEGL {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for GLDisplayEGL {}
unsafe impl Sync for GLDisplayEGL {}
