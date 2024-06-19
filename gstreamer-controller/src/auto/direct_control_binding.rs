// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstDirectControlBinding")]
    pub struct DirectControlBinding(Object<ffi::GstDirectControlBinding, ffi::GstDirectControlBindingClass>) @extends gst::ControlBinding, gst::Object;

    match fn {
        type_ => || ffi::gst_direct_control_binding_get_type(),
    }
}

impl DirectControlBinding {
    pub const NONE: Option<&'static DirectControlBinding> = None;

    #[doc(alias = "gst_direct_control_binding_new")]
    pub fn new(
        object: &impl IsA<gst::Object>,
        property_name: &str,
        cs: &impl IsA<gst::ControlSource>,
    ) -> DirectControlBinding {
        assert_initialized_main_thread!();
        unsafe {
            gst::ControlBinding::from_glib_none(ffi::gst_direct_control_binding_new(
                object.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                cs.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gst_direct_control_binding_new_absolute")]
    pub fn new_absolute(
        object: &impl IsA<gst::Object>,
        property_name: &str,
        cs: &impl IsA<gst::ControlSource>,
    ) -> DirectControlBinding {
        assert_initialized_main_thread!();
        unsafe {
            gst::ControlBinding::from_glib_none(ffi::gst_direct_control_binding_new_absolute(
                object.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                cs.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

unsafe impl Send for DirectControlBinding {}
unsafe impl Sync for DirectControlBinding {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DirectControlBinding>> Sealed for T {}
}

pub trait DirectControlBindingExt: IsA<DirectControlBinding> + sealed::Sealed + 'static {
    fn is_absolute(&self) -> bool {
        ObjectExt::property(self.as_ref(), "absolute")
    }

    #[doc(alias = "control-source")]
    fn control_source(&self) -> Option<gst::ControlSource> {
        ObjectExt::property(self.as_ref(), "control-source")
    }

    #[doc(alias = "control-source")]
    fn set_control_source<P: IsA<gst::ControlSource>>(&self, control_source: Option<&P>) {
        ObjectExt::set_property(self.as_ref(), "control-source", control_source)
    }

    #[doc(alias = "control-source")]
    fn connect_control_source_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_control_source_trampoline<
            P: IsA<DirectControlBinding>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDirectControlBinding,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DirectControlBinding::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::control-source\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_control_source_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DirectControlBinding>> DirectControlBindingExt for O {}
