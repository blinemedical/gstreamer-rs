// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ClockTime;
use crate::ControlBinding;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct Object(Object<ffi::GstObject, ffi::GstObjectClass>);

    match fn {
        get_type => || ffi::gst_object_get_type(),
    }
}

impl Object {
    #[doc(alias = "gst_object_check_uniqueness")]
    pub fn check_uniqueness(list: &[Object], name: &str) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gst_object_check_uniqueness(
                list.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_object_default_deep_notify")]
    //pub fn default_deep_notify<P: IsA<glib::Object>, Q: IsA<Object>>(object: &P, orig: &Q, pspec: /*Ignored*/&glib::ParamSpec, excluded_props: &[&str]) {
    //    unsafe { TODO: call ffi:gst_object_default_deep_notify() }
    //}

    //#[doc(alias = "gst_object_ref_sink")]
    //pub fn ref_sink(object: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:gst_object_ref_sink() }
    //}

    //#[doc(alias = "gst_object_replace")]
    //pub fn replace<P: IsA<Object>, Q: IsA<Object>>(oldobj: Option<P>, newobj: Option<&Q>) -> bool {
    //    unsafe { TODO: call ffi:gst_object_replace() }
    //}
}

impl fmt::Display for Object {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&GstObjectExt::get_name(self))
    }
}

unsafe impl Send for Object {}
unsafe impl Sync for Object {}

pub const NONE_OBJECT: Option<&Object> = None;

pub trait GstObjectExt: 'static {
    #[doc(alias = "gst_object_add_control_binding")]
    fn add_control_binding<P: IsA<ControlBinding>>(
        &self,
        binding: &P,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_object_default_error")]
    fn default_error(&self, error: &glib::Error, debug: Option<&str>);

    #[doc(alias = "gst_object_get_control_binding")]
    fn get_control_binding(&self, property_name: &str) -> Option<ControlBinding>;

    #[doc(alias = "gst_object_get_control_rate")]
    fn get_control_rate(&self) -> ClockTime;

    #[doc(alias = "gst_object_get_name")]
    fn get_name(&self) -> glib::GString;

    #[doc(alias = "gst_object_get_parent")]
    fn get_parent(&self) -> Option<Object>;

    #[doc(alias = "gst_object_get_path_string")]
    fn get_path_string(&self) -> glib::GString;

    #[doc(alias = "gst_object_get_value")]
    fn get_value(&self, property_name: &str, timestamp: ClockTime) -> Option<glib::Value>;

    //#[doc(alias = "gst_object_get_value_array")]
    //fn get_value_array(&self, property_name: &str, timestamp: ClockTime, interval: ClockTime, n_values: u32, values: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool;

    #[doc(alias = "gst_object_has_active_control_bindings")]
    fn has_active_control_bindings(&self) -> bool;

    #[doc(alias = "gst_object_has_ancestor")]
    fn has_ancestor<P: IsA<Object>>(&self, ancestor: &P) -> bool;

    #[doc(alias = "gst_object_has_as_ancestor")]
    fn has_as_ancestor<P: IsA<Object>>(&self, ancestor: &P) -> bool;

    #[doc(alias = "gst_object_has_as_parent")]
    fn has_as_parent<P: IsA<Object>>(&self, parent: &P) -> bool;

    #[doc(alias = "gst_object_remove_control_binding")]
    fn remove_control_binding<P: IsA<ControlBinding>>(&self, binding: &P) -> bool;

    #[doc(alias = "gst_object_set_control_binding_disabled")]
    fn set_control_binding_disabled(&self, property_name: &str, disabled: bool);

    #[doc(alias = "gst_object_set_control_bindings_disabled")]
    fn set_control_bindings_disabled(&self, disabled: bool);

    #[doc(alias = "gst_object_set_control_rate")]
    fn set_control_rate(&self, control_rate: ClockTime);

    #[doc(alias = "gst_object_set_parent")]
    fn set_parent<P: IsA<Object>>(&self, parent: &P) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_object_suggest_next_sync")]
    fn suggest_next_sync(&self) -> ClockTime;

    #[doc(alias = "gst_object_sync_values")]
    fn sync_values(&self, timestamp: ClockTime) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_object_unparent")]
    fn unparent(&self);

    //fn connect_deep_notify<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Object>> GstObjectExt for O {
    fn add_control_binding<P: IsA<ControlBinding>>(
        &self,
        binding: &P,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::gst_object_add_control_binding(
                    self.as_ref().to_glib_none().0,
                    binding.as_ref().to_glib_none().0
                ),
                "Failed to add control binding"
            )
        }
    }

    fn default_error(&self, error: &glib::Error, debug: Option<&str>) {
        unsafe {
            ffi::gst_object_default_error(
                self.as_ref().to_glib_none().0,
                error.to_glib_none().0,
                debug.to_glib_none().0,
            );
        }
    }

    fn get_control_binding(&self, property_name: &str) -> Option<ControlBinding> {
        unsafe {
            from_glib_full(ffi::gst_object_get_control_binding(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    fn get_control_rate(&self) -> ClockTime {
        unsafe {
            from_glib(ffi::gst_object_get_control_rate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_name(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gst_object_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn get_parent(&self) -> Option<Object> {
        unsafe { from_glib_full(ffi::gst_object_get_parent(self.as_ref().to_glib_none().0)) }
    }

    fn get_path_string(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_object_get_path_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_value(&self, property_name: &str, timestamp: ClockTime) -> Option<glib::Value> {
        unsafe {
            from_glib_full(ffi::gst_object_get_value(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                timestamp.to_glib(),
            ))
        }
    }

    //fn get_value_array(&self, property_name: &str, timestamp: ClockTime, interval: ClockTime, n_values: u32, values: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:gst_object_get_value_array() }
    //}

    fn has_active_control_bindings(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_active_control_bindings(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_ancestor<P: IsA<Object>>(&self, ancestor: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_ancestor(
                self.as_ref().to_glib_none().0,
                ancestor.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_as_ancestor<P: IsA<Object>>(&self, ancestor: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_as_ancestor(
                self.as_ref().to_glib_none().0,
                ancestor.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_as_parent<P: IsA<Object>>(&self, parent: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_object_has_as_parent(
                self.as_ref().to_glib_none().0,
                parent.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_control_binding<P: IsA<ControlBinding>>(&self, binding: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_object_remove_control_binding(
                self.as_ref().to_glib_none().0,
                binding.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_control_binding_disabled(&self, property_name: &str, disabled: bool) {
        unsafe {
            ffi::gst_object_set_control_binding_disabled(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                disabled.to_glib(),
            );
        }
    }

    fn set_control_bindings_disabled(&self, disabled: bool) {
        unsafe {
            ffi::gst_object_set_control_bindings_disabled(
                self.as_ref().to_glib_none().0,
                disabled.to_glib(),
            );
        }
    }

    fn set_control_rate(&self, control_rate: ClockTime) {
        unsafe {
            ffi::gst_object_set_control_rate(
                self.as_ref().to_glib_none().0,
                control_rate.to_glib(),
            );
        }
    }

    fn set_parent<P: IsA<Object>>(&self, parent: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::gst_object_set_parent(
                    self.as_ref().to_glib_none().0,
                    parent.as_ref().to_glib_none().0
                ),
                "Failed to set parent object"
            )
        }
    }

    fn suggest_next_sync(&self) -> ClockTime {
        unsafe {
            from_glib(ffi::gst_object_suggest_next_sync(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn sync_values(&self, timestamp: ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::gst_object_sync_values(self.as_ref().to_glib_none().0, timestamp.to_glib()),
                "Failed to sync values"
            )
        }
    }

    fn unparent(&self) {
        unsafe {
            ffi::gst_object_unparent(self.as_ref().to_glib_none().0);
        }
    }

    //fn connect_deep_notify<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored prop: GObject.ParamSpec
    //}

    fn connect_property_parent_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Object>,
        {
            let f: &F = &*(f as *const F);
            f(&Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
