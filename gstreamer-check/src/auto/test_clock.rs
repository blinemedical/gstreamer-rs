// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::ObjectType;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct TestClock(Object<ffi::GstTestClock, ffi::GstTestClockClass, TestClockClass>) @extends gst::Clock, gst::Object;

    match fn {
        get_type => || ffi::gst_test_clock_get_type(),
    }
}

impl TestClock {
    pub fn new() -> TestClock {
        assert_initialized_main_thread!();
        unsafe {
            gst::Clock::from_glib_full(ffi::gst_test_clock_new()).unsafe_cast()
        }
    }

    pub fn new_with_start_time(start_time: gst::ClockTime) -> TestClock {
        assert_initialized_main_thread!();
        unsafe {
            gst::Clock::from_glib_full(ffi::gst_test_clock_new_with_start_time(start_time.to_glib())).unsafe_cast()
        }
    }

    pub fn advance_time(&self, delta: gst::ClockTimeDiff) {
        unsafe {
            ffi::gst_test_clock_advance_time(self.to_glib_none().0, delta);
        }
    }

    pub fn crank(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_test_clock_crank(self.to_glib_none().0))
        }
    }

    pub fn get_next_entry_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_test_clock_get_next_entry_time(self.to_glib_none().0))
        }
    }

    //pub fn has_id(&self, id: /*Ignored*/gst::ClockID) -> bool {
    //    unsafe { TODO: call ffi::gst_test_clock_has_id() }
    //}

    pub fn peek_id_count(&self) -> u32 {
        unsafe {
            ffi::gst_test_clock_peek_id_count(self.to_glib_none().0)
        }
    }

    //pub fn peek_next_pending_id(&self, pending_id: /*Ignored*/&mut gst::ClockID) -> bool {
    //    unsafe { TODO: call ffi::gst_test_clock_peek_next_pending_id() }
    //}

    //pub fn process_id_list(&self, pending_list: /*Ignored*/&[&gst::ClockID]) -> u32 {
    //    unsafe { TODO: call ffi::gst_test_clock_process_id_list() }
    //}

    //pub fn process_next_clock_id(&self) -> /*Ignored*/Option<gst::ClockID> {
    //    unsafe { TODO: call ffi::gst_test_clock_process_next_clock_id() }
    //}

    pub fn set_time(&self, new_time: gst::ClockTime) {
        unsafe {
            ffi::gst_test_clock_set_time(self.to_glib_none().0, new_time.to_glib());
        }
    }

    //pub fn wait_for_multiple_pending_ids(&self, count: u32, pending_list: /*Unimplemented*/Vec<gst::ClockID>) {
    //    unsafe { TODO: call ffi::gst_test_clock_wait_for_multiple_pending_ids() }
    //}

    //pub fn wait_for_next_pending_id(&self, pending_id: /*Ignored*/&mut gst::ClockID) {
    //    unsafe { TODO: call ffi::gst_test_clock_wait_for_next_pending_id() }
    //}

    pub fn wait_for_pending_id_count(&self, count: u32) {
        unsafe {
            ffi::gst_test_clock_wait_for_pending_id_count(self.to_glib_none().0, count);
        }
    }

    pub fn get_property_clock_type(&self) -> gst::ClockType {
        unsafe {
            let mut value = Value::from_type(<gst::ClockType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.as_ptr() as *mut gobject_ffi::GObject, b"clock-type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_clock_type(&self, clock_type: gst::ClockType) {
        unsafe {
            gobject_ffi::g_object_set_property(self.as_ptr() as *mut gobject_ffi::GObject, b"clock-type\0".as_ptr() as *const _, Value::from(&clock_type).to_glib_none().0);
        }
    }

    pub fn get_property_start_time(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.as_ptr() as *mut gobject_ffi::GObject, b"start-time\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    //pub fn id_list_get_latest_time(pending_list: /*Ignored*/&[&gst::ClockID]) -> gst::ClockTime {
    //    unsafe { TODO: call ffi::gst_test_clock_id_list_get_latest_time() }
    //}

    pub fn connect_property_clock_type_notify<F: Fn(&TestClock) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::clock-type\0".as_ptr() as *const _,
                Some(transmute(notify_clock_type_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

impl Default for TestClock {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for TestClock {}
unsafe impl Sync for TestClock {}

unsafe extern "C" fn notify_clock_type_trampoline<F: Fn(&TestClock) + Send + Sync + 'static>(this: *mut ffi::GstTestClock, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}
