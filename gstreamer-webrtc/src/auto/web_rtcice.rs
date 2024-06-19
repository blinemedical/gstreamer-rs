// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{WebRTCICECandidateStats, WebRTCICEComponent, WebRTCICEStream, WebRTCICETransport};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstWebRTCICE")]
    pub struct WebRTCICE(Object<ffi::GstWebRTCICE, ffi::GstWebRTCICEClass>);

    match fn {
        type_ => || ffi::gst_webrtc_ice_get_type(),
    }
}

impl WebRTCICE {
    pub const NONE: Option<&'static WebRTCICE> = None;
}

unsafe impl Send for WebRTCICE {}
unsafe impl Sync for WebRTCICE {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::WebRTCICE>> Sealed for T {}
}

pub trait WebRTCICEExt: IsA<WebRTCICE> + sealed::Sealed + 'static {
    #[doc(alias = "gst_webrtc_ice_add_stream")]
    fn add_stream(&self, session_id: u32) -> Option<WebRTCICEStream> {
        unsafe {
            from_glib_full(ffi::gst_webrtc_ice_add_stream(
                self.as_ref().to_glib_none().0,
                session_id,
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_add_turn_server")]
    fn add_turn_server(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_webrtc_ice_add_turn_server(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_find_transport")]
    fn find_transport(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
        component: WebRTCICEComponent,
    ) -> Option<WebRTCICETransport> {
        unsafe {
            from_glib_full(ffi::gst_webrtc_ice_find_transport(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                component.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_gather_candidates")]
    fn gather_candidates(&self, stream: &impl IsA<WebRTCICEStream>) -> bool {
        unsafe {
            from_glib(ffi::gst_webrtc_ice_gather_candidates(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_get_http_proxy")]
    #[doc(alias = "get_http_proxy")]
    fn http_proxy(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_webrtc_ice_get_http_proxy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_get_is_controller")]
    #[doc(alias = "get_is_controller")]
    fn is_controller(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_webrtc_ice_get_is_controller(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_get_local_candidates")]
    #[doc(alias = "get_local_candidates")]
    fn local_candidates(&self, stream: &impl IsA<WebRTCICEStream>) -> Vec<WebRTCICECandidateStats> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_webrtc_ice_get_local_candidates(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_get_remote_candidates")]
    #[doc(alias = "get_remote_candidates")]
    fn remote_candidates(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
    ) -> Vec<WebRTCICECandidateStats> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_webrtc_ice_get_remote_candidates(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_get_selected_pair")]
    #[doc(alias = "get_selected_pair")]
    fn selected_pair(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
    ) -> Option<(WebRTCICECandidateStats, WebRTCICECandidateStats)> {
        unsafe {
            let mut local_stats = std::ptr::null_mut();
            let mut remote_stats = std::ptr::null_mut();
            let ret = from_glib(ffi::gst_webrtc_ice_get_selected_pair(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                &mut local_stats,
                &mut remote_stats,
            ));
            if ret {
                Some((from_glib_full(local_stats), from_glib_full(remote_stats)))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_webrtc_ice_get_stun_server")]
    #[doc(alias = "get_stun_server")]
    fn stun_server(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_webrtc_ice_get_stun_server(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_get_turn_server")]
    #[doc(alias = "get_turn_server")]
    fn turn_server(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_webrtc_ice_get_turn_server(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_set_force_relay")]
    fn set_force_relay(&self, force_relay: bool) {
        unsafe {
            ffi::gst_webrtc_ice_set_force_relay(
                self.as_ref().to_glib_none().0,
                force_relay.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_webrtc_ice_set_http_proxy")]
    fn set_http_proxy(&self, uri: &str) {
        unsafe {
            ffi::gst_webrtc_ice_set_http_proxy(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_webrtc_ice_set_is_controller")]
    fn set_is_controller(&self, controller: bool) {
        unsafe {
            ffi::gst_webrtc_ice_set_is_controller(
                self.as_ref().to_glib_none().0,
                controller.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_webrtc_ice_set_local_credentials")]
    fn set_local_credentials(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
        ufrag: &str,
        pwd: &str,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_webrtc_ice_set_local_credentials(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                ufrag.to_glib_none().0,
                pwd.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_set_on_ice_candidate")]
    fn set_on_ice_candidate<P: Fn(&WebRTCICE, u32, &str) + Send + Sync + 'static>(&self, func: P) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<P: Fn(&WebRTCICE, u32, &str) + Send + Sync + 'static>(
            ice: *mut ffi::GstWebRTCICE,
            stream_id: libc::c_uint,
            candidate: *const libc::c_char,
            user_data: glib::ffi::gpointer,
        ) {
            let ice = from_glib_borrow(ice);
            let candidate: Borrowed<glib::GString> = from_glib_borrow(candidate);
            let callback = &*(user_data as *mut P);
            (*callback)(&ice, stream_id, candidate.as_str())
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn notify_func<P: Fn(&WebRTCICE, u32, &str) + Send + Sync + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call3 = Some(notify_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gst_webrtc_ice_set_on_ice_candidate(
                self.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gst_webrtc_ice_set_remote_credentials")]
    fn set_remote_credentials(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
        ufrag: &str,
        pwd: &str,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_webrtc_ice_set_remote_credentials(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                ufrag.to_glib_none().0,
                pwd.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_set_stun_server")]
    fn set_stun_server(&self, uri: Option<&str>) {
        unsafe {
            ffi::gst_webrtc_ice_set_stun_server(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_webrtc_ice_set_tos")]
    fn set_tos(&self, stream: &impl IsA<WebRTCICEStream>, tos: u32) {
        unsafe {
            ffi::gst_webrtc_ice_set_tos(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                tos,
            );
        }
    }

    #[doc(alias = "gst_webrtc_ice_set_turn_server")]
    fn set_turn_server(&self, uri: Option<&str>) {
        unsafe {
            ffi::gst_webrtc_ice_set_turn_server(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "max-rtp-port")]
    fn max_rtp_port(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "max-rtp-port")
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "max-rtp-port")]
    fn set_max_rtp_port(&self, max_rtp_port: u32) {
        ObjectExt::set_property(self.as_ref(), "max-rtp-port", max_rtp_port)
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "min-rtp-port")]
    fn min_rtp_port(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "min-rtp-port")
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "min-rtp-port")]
    fn set_min_rtp_port(&self, min_rtp_port: u32) {
        ObjectExt::set_property(self.as_ref(), "min-rtp-port", min_rtp_port)
    }

    #[doc(alias = "add-local-ip-address")]
    fn connect_add_local_ip_address<F: Fn(&Self, &str) -> bool + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn add_local_ip_address_trampoline<
            P: IsA<WebRTCICE>,
            F: Fn(&P, &str) -> bool + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCICE,
            address: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                WebRTCICE::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(address),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"add-local-ip-address\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    add_local_ip_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_add_local_ip_address(&self, address: &str) -> bool {
        self.emit_by_name("add-local-ip-address", &[&address])
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "max-rtp-port")]
    fn connect_max_rtp_port_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_rtp_port_trampoline<
            P: IsA<WebRTCICE>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCICE,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(WebRTCICE::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-rtp-port\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_rtp_port_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "min-rtp-port")]
    fn connect_min_rtp_port_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_rtp_port_trampoline<
            P: IsA<WebRTCICE>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCICE,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(WebRTCICE::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-rtp-port\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_min_rtp_port_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<WebRTCICE>> WebRTCICEExt for O {}
