// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bus;
use Device;
use Object;
use ffi;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct DeviceMonitor(Object<ffi::GstDeviceMonitor, ffi::GstDeviceMonitorClass, DeviceMonitorClass>) @extends Object;

    match fn {
        get_type => || ffi::gst_device_monitor_get_type(),
    }
}

unsafe impl Send for DeviceMonitor {}
unsafe impl Sync for DeviceMonitor {}

pub const NONE_DEVICE_MONITOR: Option<&DeviceMonitor> = None;

pub trait DeviceMonitorExt: 'static {
    fn get_bus(&self) -> Bus;

    fn get_devices(&self) -> Vec<Device>;

    fn get_providers(&self) -> Vec<GString>;

    fn get_show_all_devices(&self) -> bool;

    fn set_show_all_devices(&self, show_all: bool);

    fn start(&self) -> Result<(), glib::error::BoolError>;

    fn stop(&self);

    fn get_property_show_all(&self) -> bool;

    fn set_property_show_all(&self, show_all: bool);

    fn connect_property_show_all_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeviceMonitor>> DeviceMonitorExt for O {
    fn get_bus(&self) -> Bus {
        unsafe {
            from_glib_full(ffi::gst_device_monitor_get_bus(self.as_ref().to_glib_none().0))
        }
    }

    fn get_devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_monitor_get_devices(self.as_ref().to_glib_none().0))
        }
    }

    fn get_providers(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_monitor_get_providers(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_all_devices(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_device_monitor_get_show_all_devices(self.as_ref().to_glib_none().0))
        }
    }

    fn set_show_all_devices(&self, show_all: bool) {
        unsafe {
            ffi::gst_device_monitor_set_show_all_devices(self.as_ref().to_glib_none().0, show_all.to_glib());
        }
    }

    fn start(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(ffi::gst_device_monitor_start(self.as_ref().to_glib_none().0), "Failed to start")
        }
    }

    fn stop(&self) {
        unsafe {
            ffi::gst_device_monitor_stop(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_show_all(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"show-all\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_show_all(&self, show_all: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"show-all\0".as_ptr() as *const _, Value::from(&show_all).to_glib_none().0);
        }
    }

    fn connect_property_show_all_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-all\0".as_ptr() as *const _,
                Some(transmute(notify_show_all_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_show_all_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut ffi::GstDeviceMonitor, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceMonitor> {
    let f: &F = &*(f as *const F);
    f(&DeviceMonitor::from_glib_borrow(this).unsafe_cast())
}
