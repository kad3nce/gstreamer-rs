// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use BaseTransformClass;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct BaseTransform(Object<ffi::GstBaseTransform, ffi::GstBaseTransformClass, BaseTransformClass>): [
        gst::Element => gst_ffi::GstElement,
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_base_transform_get_type(),
    }
}

unsafe impl Send for BaseTransform {}
unsafe impl Sync for BaseTransform {}

pub trait BaseTransformExt {
    //fn get_allocator(&self, allocator: /*Ignored*/gst::Allocator, params: /*Ignored*/gst::AllocationParams);

    fn get_buffer_pool(&self) -> Option<gst::BufferPool>;

    fn is_in_place(&self) -> bool;

    fn is_passthrough(&self) -> bool;

    fn is_qos_enabled(&self) -> bool;

    fn reconfigure_sink(&self);

    fn reconfigure_src(&self);

    fn set_gap_aware(&self, gap_aware: bool);

    fn set_in_place(&self, in_place: bool);

    fn set_passthrough(&self, passthrough: bool);

    fn set_prefer_passthrough(&self, prefer_passthrough: bool);

    fn set_qos_enabled(&self, enabled: bool);

    fn update_qos(&self, proportion: f64, diff: gst::ClockTimeDiff, timestamp: gst::ClockTime);

    fn update_src_caps(&self, updated_caps: &gst::Caps) -> bool;

    fn get_property_qos(&self) -> bool;

    fn set_property_qos(&self, qos: bool);

    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BaseTransform> + IsA<glib::object::Object>> BaseTransformExt for O {
    //fn get_allocator(&self, allocator: /*Ignored*/gst::Allocator, params: /*Ignored*/gst::AllocationParams) {
    //    unsafe { TODO: call ffi::gst_base_transform_get_allocator() }
    //}

    fn get_buffer_pool(&self) -> Option<gst::BufferPool> {
        unsafe {
            from_glib_full(ffi::gst_base_transform_get_buffer_pool(self.to_glib_none().0))
        }
    }

    fn is_in_place(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_transform_is_in_place(self.to_glib_none().0))
        }
    }

    fn is_passthrough(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_transform_is_passthrough(self.to_glib_none().0))
        }
    }

    fn is_qos_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_transform_is_qos_enabled(self.to_glib_none().0))
        }
    }

    fn reconfigure_sink(&self) {
        unsafe {
            ffi::gst_base_transform_reconfigure_sink(self.to_glib_none().0);
        }
    }

    fn reconfigure_src(&self) {
        unsafe {
            ffi::gst_base_transform_reconfigure_src(self.to_glib_none().0);
        }
    }

    fn set_gap_aware(&self, gap_aware: bool) {
        unsafe {
            ffi::gst_base_transform_set_gap_aware(self.to_glib_none().0, gap_aware.to_glib());
        }
    }

    fn set_in_place(&self, in_place: bool) {
        unsafe {
            ffi::gst_base_transform_set_in_place(self.to_glib_none().0, in_place.to_glib());
        }
    }

    fn set_passthrough(&self, passthrough: bool) {
        unsafe {
            ffi::gst_base_transform_set_passthrough(self.to_glib_none().0, passthrough.to_glib());
        }
    }

    fn set_prefer_passthrough(&self, prefer_passthrough: bool) {
        unsafe {
            ffi::gst_base_transform_set_prefer_passthrough(self.to_glib_none().0, prefer_passthrough.to_glib());
        }
    }

    fn set_qos_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_base_transform_set_qos_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    fn update_qos(&self, proportion: f64, diff: gst::ClockTimeDiff, timestamp: gst::ClockTime) {
        unsafe {
            ffi::gst_base_transform_update_qos(self.to_glib_none().0, proportion, diff, timestamp.to_glib());
        }
    }

    fn update_src_caps(&self, updated_caps: &gst::Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_base_transform_update_src_caps(self.to_glib_none().0, updated_caps.to_glib_none().0))
        }
    }

    fn get_property_qos(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "qos".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_qos(&self, qos: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "qos".to_glib_none().0, Value::from(&qos).to_glib_none().0);
        }
    }

    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::qos",
                transmute(notify_qos_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_qos_trampoline<P>(this: *mut ffi::GstBaseTransform, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<BaseTransform> {
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&BaseTransform::from_glib_borrow(this).downcast_unchecked())
}
