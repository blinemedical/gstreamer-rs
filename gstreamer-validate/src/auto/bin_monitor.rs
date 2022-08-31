// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ElementMonitor;
use crate::Monitor;
use crate::Reporter;
use crate::Runner;
use crate::Scenario;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstValidateBinMonitor")]
    pub struct BinMonitor(Object<ffi::GstValidateBinMonitor, ffi::GstValidateBinMonitorClass>) @extends ElementMonitor, Monitor, gst::Object, @implements Reporter;

    match fn {
        type_ => || ffi::gst_validate_bin_monitor_get_type(),
    }
}

impl BinMonitor {
    pub const NONE: Option<&'static BinMonitor> = None;

    #[doc(alias = "gst_validate_bin_monitor_new")]
    pub fn new(
        bin: &impl IsA<gst::Bin>,
        runner: &impl IsA<Runner>,
        parent: Option<&impl IsA<Monitor>>,
    ) -> BinMonitor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_validate_bin_monitor_new(
                bin.as_ref().to_glib_none().0,
                runner.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for BinMonitor {}
unsafe impl Sync for BinMonitor {}

pub trait BinMonitorExt: 'static {
    #[doc(alias = "gst_validate_bin_monitor_get_scenario")]
    #[doc(alias = "get_scenario")]
    fn scenario(&self) -> Option<Scenario>;

    #[doc(alias = "handles-states")]
    fn is_handles_states(&self) -> bool;

    #[doc(alias = "handles-states")]
    fn connect_handles_states_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<BinMonitor>> BinMonitorExt for O {
    fn scenario(&self) -> Option<Scenario> {
        unsafe {
            from_glib_full(ffi::gst_validate_bin_monitor_get_scenario(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_handles_states(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "handles-states")
    }

    fn connect_handles_states_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_handles_states_trampoline<
            P: IsA<BinMonitor>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstValidateBinMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BinMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::handles-states\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_handles_states_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}