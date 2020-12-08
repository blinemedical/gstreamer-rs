// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Bus;
use crate::Device;
use crate::DeviceProviderFactory;
use crate::Object;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct DeviceProvider(Object<ffi::GstDeviceProvider, ffi::GstDeviceProviderClass>) @extends Object;

    match fn {
        get_type => || ffi::gst_device_provider_get_type(),
    }
}

unsafe impl Send for DeviceProvider {}
unsafe impl Sync for DeviceProvider {}

pub const NONE_DEVICE_PROVIDER: Option<&DeviceProvider> = None;

pub trait DeviceProviderExt: 'static {
    #[doc(alias = "gst_device_provider_can_monitor")]
    fn can_monitor(&self) -> bool;

    #[doc(alias = "gst_device_provider_device_add")]
    fn device_add<P: IsA<Device>>(&self, device: &P);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_device_provider_device_changed")]
    fn device_changed<P: IsA<Device>, Q: IsA<Device>>(&self, device: &P, changed_device: &Q);

    #[doc(alias = "gst_device_provider_device_remove")]
    fn device_remove<P: IsA<Device>>(&self, device: &P);

    #[doc(alias = "gst_device_provider_get_bus")]
    fn get_bus(&self) -> Bus;

    #[doc(alias = "gst_device_provider_get_devices")]
    fn get_devices(&self) -> Vec<Device>;

    #[doc(alias = "gst_device_provider_get_factory")]
    fn get_factory(&self) -> Option<DeviceProviderFactory>;

    #[doc(alias = "gst_device_provider_get_hidden_providers")]
    fn get_hidden_providers(&self) -> Vec<glib::GString>;

    #[doc(alias = "gst_device_provider_hide_provider")]
    fn hide_provider(&self, name: &str);

    #[doc(alias = "gst_device_provider_start")]
    fn start(&self) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_device_provider_stop")]
    fn stop(&self);

    #[doc(alias = "gst_device_provider_unhide_provider")]
    fn unhide_provider(&self, name: &str);

    fn connect_provider_hidden<F: Fn(&Self, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_provider_unhidden<F: Fn(&Self, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DeviceProvider>> DeviceProviderExt for O {
    fn can_monitor(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_device_provider_can_monitor(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn device_add<P: IsA<Device>>(&self, device: &P) {
        unsafe {
            ffi::gst_device_provider_device_add(
                self.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn device_changed<P: IsA<Device>, Q: IsA<Device>>(&self, device: &P, changed_device: &Q) {
        unsafe {
            ffi::gst_device_provider_device_changed(
                self.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
                changed_device.as_ref().to_glib_none().0,
            );
        }
    }

    fn device_remove<P: IsA<Device>>(&self, device: &P) {
        unsafe {
            ffi::gst_device_provider_device_remove(
                self.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_bus(&self) -> Bus {
        unsafe {
            from_glib_full(ffi::gst_device_provider_get_bus(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_provider_get_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_factory(&self) -> Option<DeviceProviderFactory> {
        unsafe {
            from_glib_none(ffi::gst_device_provider_get_factory(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_hidden_providers(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_device_provider_get_hidden_providers(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn hide_provider(&self, name: &str) {
        unsafe {
            ffi::gst_device_provider_hide_provider(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn start(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::gst_device_provider_start(self.as_ref().to_glib_none().0),
                "Failed to start"
            )
        }
    }

    fn stop(&self) {
        unsafe {
            ffi::gst_device_provider_stop(self.as_ref().to_glib_none().0);
        }
    }

    fn unhide_provider(&self, name: &str) {
        unsafe {
            ffi::gst_device_provider_unhide_provider(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn connect_provider_hidden<F: Fn(&Self, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn provider_hidden_trampoline<
            P,
            F: Fn(&P, &str) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDeviceProvider,
            object: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DeviceProvider>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DeviceProvider::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"provider-hidden\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    provider_hidden_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_provider_unhidden<F: Fn(&Self, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn provider_unhidden_trampoline<
            P,
            F: Fn(&P, &str) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDeviceProvider,
            object: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DeviceProvider>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DeviceProvider::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"provider-unhidden\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    provider_unhidden_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
