// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
use crate::WebRTCKind;
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
use crate::WebRTCRTPTransceiverDirection;
use crate::{WebRTCRTPReceiver, WebRTCRTPSender};
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::{prelude::*, translate::*};
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstWebRTCRTPTransceiver")]
    pub struct WebRTCRTPTransceiver(Object<ffi::GstWebRTCRTPTransceiver, ffi::GstWebRTCRTPTransceiverClass>);

    match fn {
        type_ => || ffi::gst_webrtc_rtp_transceiver_get_type(),
    }
}

impl WebRTCRTPTransceiver {
    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "codec-preferences")]
    pub fn codec_preferences(&self) -> Option<gst::Caps> {
        ObjectExt::property(self, "codec-preferences")
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "codec-preferences")]
    pub fn set_codec_preferences(&self, codec_preferences: Option<&gst::Caps>) {
        ObjectExt::set_property(self, "codec-preferences", codec_preferences)
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "current-direction")]
    pub fn current_direction(&self) -> WebRTCRTPTransceiverDirection {
        ObjectExt::property(self, "current-direction")
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    pub fn direction(&self) -> WebRTCRTPTransceiverDirection {
        ObjectExt::property(self, "direction")
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    pub fn set_direction(&self, direction: WebRTCRTPTransceiverDirection) {
        ObjectExt::set_property(self, "direction", direction)
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub fn kind(&self) -> WebRTCKind {
        ObjectExt::property(self, "kind")
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub fn mid(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "mid")
    }

    pub fn mlineindex(&self) -> u32 {
        ObjectExt::property(self, "mlineindex")
    }

    pub fn receiver(&self) -> Option<WebRTCRTPReceiver> {
        ObjectExt::property(self, "receiver")
    }

    pub fn sender(&self) -> Option<WebRTCRTPSender> {
        ObjectExt::property(self, "sender")
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "codec-preferences")]
    pub fn connect_codec_preferences_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_codec_preferences_trampoline<
            F: Fn(&WebRTCRTPTransceiver) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCRTPTransceiver,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::codec-preferences\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_codec_preferences_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "current-direction")]
    pub fn connect_current_direction_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_direction_trampoline<
            F: Fn(&WebRTCRTPTransceiver) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCRTPTransceiver,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::current-direction\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_current_direction_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "direction")]
    pub fn connect_direction_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<
            F: Fn(&WebRTCRTPTransceiver) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCRTPTransceiver,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::direction\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_direction_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "kind")]
    pub fn connect_kind_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_kind_trampoline<
            F: Fn(&WebRTCRTPTransceiver) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCRTPTransceiver,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::kind\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_kind_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "mid")]
    pub fn connect_mid_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mid_trampoline<
            F: Fn(&WebRTCRTPTransceiver) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCRTPTransceiver,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mid\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_mid_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for WebRTCRTPTransceiver {}
unsafe impl Sync for WebRTCRTPTransceiver {}
