// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use GLContext;
use GLStereoDownmix;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_ffi;
use gst_video;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct GLViewConvert(Object<ffi::GstGLViewConvert, ffi::GstGLViewConvertClass>): [
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_gl_view_convert_get_type(),
    }
}

impl GLViewConvert {
    pub fn new() -> GLViewConvert {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_gl_view_convert_new())
        }
    }

    pub fn perform(&self, inbuf: &gst::Buffer) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(ffi::gst_gl_view_convert_perform(self.to_glib_none().0, inbuf.to_glib_none().0))
        }
    }

    pub fn reset(&self) {
        unsafe {
            ffi::gst_gl_view_convert_reset(self.to_glib_none().0);
        }
    }

    pub fn set_caps(&self, in_caps: &gst::Caps, out_caps: &gst::Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_view_convert_set_caps(self.to_glib_none().0, in_caps.to_glib_none().0, out_caps.to_glib_none().0))
        }
    }

    pub fn set_context(&self, context: &GLContext) {
        unsafe {
            ffi::gst_gl_view_convert_set_context(self.to_glib_none().0, context.to_glib_none().0);
        }
    }

    pub fn transform_caps(&self, direction: gst::PadDirection, caps: &gst::Caps, filter: &gst::Caps) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_gl_view_convert_transform_caps(self.to_glib_none().0, direction.to_glib(), caps.to_glib_none().0, filter.to_glib_none().0))
        }
    }

    pub fn get_property_downmix_mode(&self) -> GLStereoDownmix {
        unsafe {
            let mut value = Value::from_type(<GLStereoDownmix as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, b"downmix-mode\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_downmix_mode(&self, downmix_mode: GLStereoDownmix) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, b"downmix-mode\0".as_ptr() as *const _, Value::from(&downmix_mode).to_glib_none().0);
        }
    }

    pub fn get_property_input_flags_override(&self) -> gst_video::VideoMultiviewFlags {
        unsafe {
            let mut value = Value::from_type(<gst_video::VideoMultiviewFlags as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, b"input-flags-override\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_input_flags_override(&self, input_flags_override: gst_video::VideoMultiviewFlags) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, b"input-flags-override\0".as_ptr() as *const _, Value::from(&input_flags_override).to_glib_none().0);
        }
    }

    pub fn get_property_input_mode_override(&self) -> gst_video::VideoMultiviewMode {
        unsafe {
            let mut value = Value::from_type(<gst_video::VideoMultiviewMode as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, b"input-mode-override\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_input_mode_override(&self, input_mode_override: gst_video::VideoMultiviewMode) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, b"input-mode-override\0".as_ptr() as *const _, Value::from(&input_mode_override).to_glib_none().0);
        }
    }

    pub fn get_property_output_flags_override(&self) -> gst_video::VideoMultiviewFlags {
        unsafe {
            let mut value = Value::from_type(<gst_video::VideoMultiviewFlags as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, b"output-flags-override\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_output_flags_override(&self, output_flags_override: gst_video::VideoMultiviewFlags) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, b"output-flags-override\0".as_ptr() as *const _, Value::from(&output_flags_override).to_glib_none().0);
        }
    }

    pub fn get_property_output_mode_override(&self) -> gst_video::VideoMultiviewMode {
        unsafe {
            let mut value = Value::from_type(<gst_video::VideoMultiviewMode as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, b"output-mode-override\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_output_mode_override(&self, output_mode_override: gst_video::VideoMultiviewMode) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, b"output-mode-override\0".as_ptr() as *const _, Value::from(&output_mode_override).to_glib_none().0);
        }
    }

    pub fn connect_property_downmix_mode_notify<F: Fn(&GLViewConvert) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&GLViewConvert) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0, b"notify::downmix-mode\0".as_ptr() as *const _,
                transmute(notify_downmix_mode_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_input_flags_override_notify<F: Fn(&GLViewConvert) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&GLViewConvert) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0, b"notify::input-flags-override\0".as_ptr() as *const _,
                transmute(notify_input_flags_override_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_input_mode_override_notify<F: Fn(&GLViewConvert) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&GLViewConvert) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0, b"notify::input-mode-override\0".as_ptr() as *const _,
                transmute(notify_input_mode_override_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_output_flags_override_notify<F: Fn(&GLViewConvert) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&GLViewConvert) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0, b"notify::output-flags-override\0".as_ptr() as *const _,
                transmute(notify_output_flags_override_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_property_output_mode_override_notify<F: Fn(&GLViewConvert) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&GLViewConvert) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0, b"notify::output-mode-override\0".as_ptr() as *const _,
                transmute(notify_output_mode_override_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

impl Default for GLViewConvert {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for GLViewConvert {}
unsafe impl Sync for GLViewConvert {}

unsafe extern "C" fn notify_downmix_mode_trampoline(this: *mut ffi::GstGLViewConvert, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&GLViewConvert) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_input_flags_override_trampoline(this: *mut ffi::GstGLViewConvert, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&GLViewConvert) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_input_mode_override_trampoline(this: *mut ffi::GstGLViewConvert, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&GLViewConvert) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_output_flags_override_trampoline(this: *mut ffi::GstGLViewConvert, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&GLViewConvert) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_output_mode_override_trampoline(this: *mut ffi::GstGLViewConvert, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &&(Fn(&GLViewConvert) + Send + Sync + 'static) = transmute(f);
    f(&from_glib_borrow(this))
}