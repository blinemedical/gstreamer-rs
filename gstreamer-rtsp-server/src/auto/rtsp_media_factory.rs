// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{RTSPAddressPool, RTSPMedia, RTSPPublishClockMode, RTSPSuspendMode, RTSPTransportMode};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstRTSPMediaFactory")]
    pub struct RTSPMediaFactory(Object<ffi::GstRTSPMediaFactory, ffi::GstRTSPMediaFactoryClass>);

    match fn {
        type_ => || ffi::gst_rtsp_media_factory_get_type(),
    }
}

impl RTSPMediaFactory {
    pub const NONE: Option<&'static RTSPMediaFactory> = None;

    #[doc(alias = "gst_rtsp_media_factory_new")]
    pub fn new() -> RTSPMediaFactory {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_media_factory_new()) }
    }
}

impl Default for RTSPMediaFactory {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPMediaFactory {}
unsafe impl Sync for RTSPMediaFactory {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTSPMediaFactory>> Sealed for T {}
}

pub trait RTSPMediaFactoryExt: IsA<RTSPMediaFactory> + sealed::Sealed + 'static {
    //#[doc(alias = "gst_rtsp_media_factory_add_role")]
    //fn add_role(&self, role: &str, fieldname: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gst_rtsp_media_factory_add_role() }
    //}

    #[doc(alias = "gst_rtsp_media_factory_construct")]
    fn construct(&self, url: &gst_rtsp::RTSPUrl) -> Result<RTSPMedia, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_rtsp_media_factory_construct(
                self.as_ref().to_glib_none().0,
                url.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to construct media"))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_create_element")]
    fn create_element(&self, url: &gst_rtsp::RTSPUrl) -> Result<gst::Element, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::gst_rtsp_media_factory_create_element(
                self.as_ref().to_glib_none().0,
                url.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create media element"))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_address_pool")]
    #[doc(alias = "get_address_pool")]
    fn address_pool(&self) -> Option<RTSPAddressPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_get_address_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_buffer_size")]
    #[doc(alias = "get_buffer_size")]
    fn buffer_size(&self) -> u32 {
        unsafe { ffi::gst_rtsp_media_factory_get_buffer_size(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_clock")]
    #[doc(alias = "get_clock")]
    fn clock(&self) -> Option<gst::Clock> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_get_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_media_factory_get_do_retransmission")]
    #[doc(alias = "get_do_retransmission")]
    fn does_retransmission(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_do_retransmission(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_media_factory_get_dscp_qos")]
    #[doc(alias = "get_dscp_qos")]
    fn dscp_qos(&self) -> i32 {
        unsafe { ffi::gst_rtsp_media_factory_get_dscp_qos(self.as_ref().to_glib_none().0) }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_rtsp_media_factory_get_ensure_keyunit_on_start")]
    #[doc(alias = "get_ensure_keyunit_on_start")]
    fn is_ensure_keyunit_on_start(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_ensure_keyunit_on_start(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_rtsp_media_factory_get_ensure_keyunit_on_start_timeout")]
    #[doc(alias = "get_ensure_keyunit_on_start_timeout")]
    fn ensure_keyunit_on_start_timeout(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_media_factory_get_ensure_keyunit_on_start_timeout(
                self.as_ref().to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_latency")]
    #[doc(alias = "get_latency")]
    fn latency(&self) -> u32 {
        unsafe { ffi::gst_rtsp_media_factory_get_latency(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_launch")]
    #[doc(alias = "get_launch")]
    fn launch(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_get_launch(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_media_factory_get_max_mcast_ttl")]
    #[doc(alias = "get_max_mcast_ttl")]
    fn max_mcast_ttl(&self) -> u32 {
        unsafe { ffi::gst_rtsp_media_factory_get_max_mcast_ttl(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_media_gtype")]
    #[doc(alias = "get_media_gtype")]
    fn media_gtype(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_media_gtype(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_multicast_iface")]
    #[doc(alias = "get_multicast_iface")]
    fn multicast_iface(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_get_multicast_iface(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_rtsp_media_factory_get_permissions")]
    //#[doc(alias = "get_permissions")]
    //fn permissions(&self) -> /*Ignored*/Option<RTSPPermissions> {
    //    unsafe { TODO: call ffi:gst_rtsp_media_factory_get_permissions() }
    //}

    #[doc(alias = "gst_rtsp_media_factory_get_profiles")]
    #[doc(alias = "get_profiles")]
    fn profiles(&self) -> gst_rtsp::RTSPProfile {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_profiles(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_protocols")]
    #[doc(alias = "get_protocols")]
    fn protocols(&self) -> gst_rtsp::RTSPLowerTrans {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_protocols(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_publish_clock_mode")]
    #[doc(alias = "get_publish_clock_mode")]
    fn publish_clock_mode(&self) -> RTSPPublishClockMode {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_publish_clock_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_retransmission_time")]
    #[doc(alias = "get_retransmission_time")]
    fn retransmission_time(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_retransmission_time(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_suspend_mode")]
    #[doc(alias = "get_suspend_mode")]
    fn suspend_mode(&self) -> RTSPSuspendMode {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_suspend_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_get_transport_mode")]
    #[doc(alias = "get_transport_mode")]
    fn transport_mode(&self) -> RTSPTransportMode {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_transport_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_media_factory_is_bind_mcast_address")]
    fn is_bind_mcast_address(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_is_bind_mcast_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_rtsp_media_factory_is_enable_rtcp")]
    fn is_enable_rtcp(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_is_enable_rtcp(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_is_eos_shutdown")]
    fn is_eos_shutdown(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_is_eos_shutdown(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_is_shared")]
    fn is_shared(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_is_shared(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_is_stop_on_disonnect")]
    fn is_stop_on_disonnect(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_is_stop_on_disonnect(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_address_pool")]
    fn set_address_pool(&self, pool: Option<&impl IsA<RTSPAddressPool>>) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_address_pool(
                self.as_ref().to_glib_none().0,
                pool.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_media_factory_set_bind_mcast_address")]
    fn set_bind_mcast_address(&self, bind_mcast_addr: bool) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_bind_mcast_address(
                self.as_ref().to_glib_none().0,
                bind_mcast_addr.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_buffer_size")]
    fn set_buffer_size(&self, size: u32) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_buffer_size(self.as_ref().to_glib_none().0, size);
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_clock")]
    fn set_clock(&self, clock: Option<&impl IsA<gst::Clock>>) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_clock(
                self.as_ref().to_glib_none().0,
                clock.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_media_factory_set_do_retransmission")]
    fn set_do_retransmission(&self, do_retransmission: bool) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_do_retransmission(
                self.as_ref().to_glib_none().0,
                do_retransmission.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_media_factory_set_dscp_qos")]
    fn set_dscp_qos(&self, dscp_qos: i32) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_dscp_qos(self.as_ref().to_glib_none().0, dscp_qos);
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_rtsp_media_factory_set_enable_rtcp")]
    fn set_enable_rtcp(&self, enable: bool) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_enable_rtcp(
                self.as_ref().to_glib_none().0,
                enable.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_rtsp_media_factory_set_ensure_keyunit_on_start")]
    fn set_ensure_keyunit_on_start(&self, ensure_keyunit_on_start: bool) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_ensure_keyunit_on_start(
                self.as_ref().to_glib_none().0,
                ensure_keyunit_on_start.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_rtsp_media_factory_set_ensure_keyunit_on_start_timeout")]
    fn set_ensure_keyunit_on_start_timeout(&self, timeout: u32) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_ensure_keyunit_on_start_timeout(
                self.as_ref().to_glib_none().0,
                timeout,
            );
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_eos_shutdown")]
    fn set_eos_shutdown(&self, eos_shutdown: bool) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_eos_shutdown(
                self.as_ref().to_glib_none().0,
                eos_shutdown.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_latency")]
    fn set_latency(&self, latency: u32) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_latency(self.as_ref().to_glib_none().0, latency);
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_launch")]
    fn set_launch(&self, launch: &str) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_launch(
                self.as_ref().to_glib_none().0,
                launch.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_media_factory_set_max_mcast_ttl")]
    fn set_max_mcast_ttl(&self, ttl: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_set_max_mcast_ttl(
                self.as_ref().to_glib_none().0,
                ttl,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_media_gtype")]
    fn set_media_gtype(&self, media_gtype: glib::types::Type) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_media_gtype(
                self.as_ref().to_glib_none().0,
                media_gtype.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_multicast_iface")]
    fn set_multicast_iface(&self, multicast_iface: Option<&str>) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_multicast_iface(
                self.as_ref().to_glib_none().0,
                multicast_iface.to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "gst_rtsp_media_factory_set_permissions")]
    //fn set_permissions(&self, permissions: /*Ignored*/Option<&mut RTSPPermissions>) {
    //    unsafe { TODO: call ffi:gst_rtsp_media_factory_set_permissions() }
    //}

    #[doc(alias = "gst_rtsp_media_factory_set_profiles")]
    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_profiles(
                self.as_ref().to_glib_none().0,
                profiles.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_protocols")]
    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_protocols(
                self.as_ref().to_glib_none().0,
                protocols.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_publish_clock_mode")]
    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_publish_clock_mode(
                self.as_ref().to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_retransmission_time")]
    fn set_retransmission_time(&self, time: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_retransmission_time(
                self.as_ref().to_glib_none().0,
                time.into().into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_shared")]
    fn set_shared(&self, shared: bool) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_shared(
                self.as_ref().to_glib_none().0,
                shared.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_stop_on_disconnect")]
    fn set_stop_on_disconnect(&self, stop_on_disconnect: bool) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_stop_on_disconnect(
                self.as_ref().to_glib_none().0,
                stop_on_disconnect.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_suspend_mode")]
    fn set_suspend_mode(&self, mode: RTSPSuspendMode) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_suspend_mode(
                self.as_ref().to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_media_factory_set_transport_mode")]
    fn set_transport_mode(&self, mode: RTSPTransportMode) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_transport_mode(
                self.as_ref().to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "bind-mcast-address")]
    fn get_property_bind_mcast_address(&self) -> bool {
        ObjectExt::property(self.as_ref(), "bind-mcast-address")
    }

    #[doc(alias = "bind-mcast-address")]
    fn set_property_bind_mcast_address(&self, bind_mcast_address: bool) {
        ObjectExt::set_property(self.as_ref(), "bind-mcast-address", bind_mcast_address)
    }

    #[doc(alias = "dscp-qos")]
    fn get_property_dscp_qos(&self) -> i32 {
        ObjectExt::property(self.as_ref(), "dscp-qos")
    }

    #[doc(alias = "dscp-qos")]
    fn set_property_dscp_qos(&self, dscp_qos: i32) {
        ObjectExt::set_property(self.as_ref(), "dscp-qos", dscp_qos)
    }

    #[doc(alias = "max-mcast-ttl")]
    fn get_property_max_mcast_ttl(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "max-mcast-ttl")
    }

    #[doc(alias = "max-mcast-ttl")]
    fn set_property_max_mcast_ttl(&self, max_mcast_ttl: u32) {
        ObjectExt::set_property(self.as_ref(), "max-mcast-ttl", max_mcast_ttl)
    }

    #[doc(alias = "stop-on-disconnect")]
    fn is_stop_on_disconnect(&self) -> bool {
        ObjectExt::property(self.as_ref(), "stop-on-disconnect")
    }

    #[doc(alias = "media-configure")]
    fn connect_media_configure<F: Fn(&Self, &RTSPMedia) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn media_configure_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P, &RTSPMedia) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            object: *mut ffi::GstRTSPMedia,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"media-configure\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    media_configure_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "media-constructed")]
    fn connect_media_constructed<F: Fn(&Self, &RTSPMedia) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn media_constructed_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P, &RTSPMedia) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            object: *mut ffi::GstRTSPMedia,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"media-constructed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    media_constructed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "bind-mcast-address")]
    fn connect_bind_mcast_address_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_bind_mcast_address_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bind-mcast-address\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_bind_mcast_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "buffer-size")]
    fn connect_buffer_size_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffer_size_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::buffer-size\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_buffer_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "clock")]
    fn connect_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_clock_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::clock\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_clock_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "dscp-qos")]
    fn connect_dscp_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_dscp_qos_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dscp-qos\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_dscp_qos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "enable-rtcp")]
    fn connect_enable_rtcp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_rtcp_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-rtcp\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_enable_rtcp_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "ensure-keyunit-on-start")]
    fn connect_ensure_keyunit_on_start_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ensure_keyunit_on_start_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ensure-keyunit-on-start\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_ensure_keyunit_on_start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "ensure-keyunit-on-start-timeout")]
    fn connect_ensure_keyunit_on_start_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ensure_keyunit_on_start_timeout_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ensure-keyunit-on-start-timeout\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_ensure_keyunit_on_start_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "eos-shutdown")]
    fn connect_eos_shutdown_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_eos_shutdown_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::eos-shutdown\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_eos_shutdown_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "latency")]
    fn connect_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_latency_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::latency\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_latency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "launch")]
    fn connect_launch_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_launch_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::launch\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_launch_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-mcast-ttl")]
    fn connect_max_mcast_ttl_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_mcast_ttl_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-mcast-ttl\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_mcast_ttl_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "profiles")]
    fn connect_profiles_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_profiles_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::profiles\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_profiles_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "protocols")]
    fn connect_protocols_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_protocols_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::protocols\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_protocols_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "shared")]
    fn connect_shared_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shared_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shared\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_shared_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "stop-on-disconnect")]
    fn connect_stop_on_disconnect_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_stop_on_disconnect_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stop-on-disconnect\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_stop_on_disconnect_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "suspend-mode")]
    fn connect_suspend_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_suspend_mode_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::suspend-mode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_suspend_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transport-mode")]
    fn connect_transport_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transport_mode_trampoline<
            P: IsA<RTSPMediaFactory>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactory,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactory::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transport-mode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_transport_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<RTSPMediaFactory>> RTSPMediaFactoryExt for O {}
