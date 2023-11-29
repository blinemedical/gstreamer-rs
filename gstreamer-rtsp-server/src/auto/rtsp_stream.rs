// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{
    RTSPAddress, RTSPAddressPool, RTSPFilterResult, RTSPPublishClockMode, RTSPStreamTransport,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstRTSPStream")]
    pub struct RTSPStream(Object<ffi::GstRTSPStream, ffi::GstRTSPStreamClass>);

    match fn {
        type_ => || ffi::gst_rtsp_stream_get_type(),
    }
}

impl RTSPStream {
    pub const NONE: Option<&'static RTSPStream> = None;

    #[doc(alias = "gst_rtsp_stream_new")]
    pub fn new(
        idx: u32,
        payloader: &impl IsA<gst::Element>,
        pad: &impl IsA<gst::Pad>,
    ) -> RTSPStream {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_new(
                idx,
                payloader.as_ref().to_glib_none().0,
                pad.as_ref().to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for RTSPStream {}
unsafe impl Sync for RTSPStream {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTSPStream>> Sealed for T {}
}

pub trait RTSPStreamExt: IsA<RTSPStream> + sealed::Sealed + 'static {
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_add_multicast_client_address")]
    fn add_multicast_client_address(
        &self,
        destination: &str,
        rtp_port: u32,
        rtcp_port: u32,
        family: gio::SocketFamily,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_add_multicast_client_address(
                self.as_ref().to_glib_none().0,
                destination.to_glib_none().0,
                rtp_port,
                rtcp_port,
                family.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_add_transport")]
    fn add_transport(
        &self,
        trans: &impl IsA<RTSPStreamTransport>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_stream_add_transport(
                    self.as_ref().to_glib_none().0,
                    trans.as_ref().to_glib_none().0
                ),
                "Failed to add transport"
            )
        }
    }

    //#[doc(alias = "gst_rtsp_stream_allocate_udp_sockets")]
    //fn allocate_udp_sockets(&self, family: gio::SocketFamily, transport: /*Ignored*/&mut gst_rtsp::RTSPTransport, use_client_settings: bool) -> bool {
    //    unsafe { TODO: call ffi:gst_rtsp_stream_allocate_udp_sockets() }
    //}

    //#[doc(alias = "gst_rtsp_stream_complete_stream")]
    //fn complete_stream(&self, transport: /*Ignored*/&gst_rtsp::RTSPTransport) -> bool {
    //    unsafe { TODO: call ffi:gst_rtsp_stream_complete_stream() }
    //}

    #[doc(alias = "gst_rtsp_stream_get_address_pool")]
    #[doc(alias = "get_address_pool")]
    fn address_pool(&self) -> Option<RTSPAddressPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_address_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_buffer_size")]
    #[doc(alias = "get_buffer_size")]
    fn buffer_size(&self) -> u32 {
        unsafe { ffi::gst_rtsp_stream_get_buffer_size(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_stream_get_caps")]
    #[doc(alias = "get_caps")]
    fn caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_control")]
    #[doc(alias = "get_control")]
    fn control(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_control(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_current_seqnum")]
    #[doc(alias = "get_current_seqnum")]
    fn current_seqnum(&self) -> u16 {
        unsafe { ffi::gst_rtsp_stream_get_current_seqnum(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_stream_get_dscp_qos")]
    #[doc(alias = "get_dscp_qos")]
    fn dscp_qos(&self) -> i32 {
        unsafe { ffi::gst_rtsp_stream_get_dscp_qos(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_stream_get_index")]
    #[doc(alias = "get_index")]
    fn index(&self) -> u32 {
        unsafe { ffi::gst_rtsp_stream_get_index(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_stream_get_joined_bin")]
    #[doc(alias = "get_joined_bin")]
    fn joined_bin(&self) -> Option<gst::Bin> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_joined_bin(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_get_max_mcast_ttl")]
    #[doc(alias = "get_max_mcast_ttl")]
    fn max_mcast_ttl(&self) -> u32 {
        unsafe { ffi::gst_rtsp_stream_get_max_mcast_ttl(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_stream_get_mtu")]
    #[doc(alias = "get_mtu")]
    fn mtu(&self) -> u32 {
        unsafe { ffi::gst_rtsp_stream_get_mtu(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_stream_get_multicast_address")]
    #[doc(alias = "get_multicast_address")]
    fn multicast_address(&self, family: gio::SocketFamily) -> Option<RTSPAddress> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_multicast_address(
                self.as_ref().to_glib_none().0,
                family.into_glib(),
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_get_multicast_client_addresses")]
    #[doc(alias = "get_multicast_client_addresses")]
    fn multicast_client_addresses(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_multicast_client_addresses(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_multicast_iface")]
    #[doc(alias = "get_multicast_iface")]
    fn multicast_iface(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_multicast_iface(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_profiles")]
    #[doc(alias = "get_profiles")]
    fn profiles(&self) -> gst_rtsp::RTSPProfile {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_get_profiles(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_protocols")]
    #[doc(alias = "get_protocols")]
    fn protocols(&self) -> gst_rtsp::RTSPLowerTrans {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_get_protocols(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_pt")]
    #[doc(alias = "get_pt")]
    fn pt(&self) -> u32 {
        unsafe { ffi::gst_rtsp_stream_get_pt(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_stream_get_publish_clock_mode")]
    #[doc(alias = "get_publish_clock_mode")]
    fn publish_clock_mode(&self) -> RTSPPublishClockMode {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_get_publish_clock_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_stream_get_rate_control")]
    #[doc(alias = "get_rate_control")]
    fn is_rate_control(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_get_rate_control(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_stream_get_rates")]
    #[doc(alias = "get_rates")]
    fn rates(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut rate = std::mem::MaybeUninit::uninit();
            let mut applied_rate = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_rtsp_stream_get_rates(
                self.as_ref().to_glib_none().0,
                rate.as_mut_ptr(),
                applied_rate.as_mut_ptr(),
            ));
            if ret {
                Some((rate.assume_init(), applied_rate.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_retransmission_pt")]
    #[doc(alias = "get_retransmission_pt")]
    fn retransmission_pt(&self) -> u32 {
        unsafe { ffi::gst_rtsp_stream_get_retransmission_pt(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_stream_get_retransmission_time")]
    #[doc(alias = "get_retransmission_time")]
    fn retransmission_time(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_get_retransmission_time(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_rtcp_multicast_socket")]
    #[doc(alias = "get_rtcp_multicast_socket")]
    fn rtcp_multicast_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_rtcp_multicast_socket(
                self.as_ref().to_glib_none().0,
                family.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_rtcp_socket")]
    #[doc(alias = "get_rtcp_socket")]
    fn rtcp_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_rtcp_socket(
                self.as_ref().to_glib_none().0,
                family.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_rtp_multicast_socket")]
    #[doc(alias = "get_rtp_multicast_socket")]
    fn rtp_multicast_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_rtp_multicast_socket(
                self.as_ref().to_glib_none().0,
                family.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_rtp_socket")]
    #[doc(alias = "get_rtp_socket")]
    fn rtp_socket(&self, family: gio::SocketFamily) -> Option<gio::Socket> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_rtp_socket(
                self.as_ref().to_glib_none().0,
                family.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_rtpinfo")]
    #[doc(alias = "get_rtpinfo")]
    fn rtpinfo(&self) -> Option<(u32, u32, u32, Option<gst::ClockTime>)> {
        unsafe {
            let mut rtptime = std::mem::MaybeUninit::uninit();
            let mut seq = std::mem::MaybeUninit::uninit();
            let mut clock_rate = std::mem::MaybeUninit::uninit();
            let mut running_time = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_rtsp_stream_get_rtpinfo(
                self.as_ref().to_glib_none().0,
                rtptime.as_mut_ptr(),
                seq.as_mut_ptr(),
                clock_rate.as_mut_ptr(),
                running_time.as_mut_ptr(),
            ));
            if ret {
                Some((
                    rtptime.assume_init(),
                    seq.assume_init(),
                    clock_rate.assume_init(),
                    from_glib(running_time.assume_init()),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_rtpsession")]
    #[doc(alias = "get_rtpsession")]
    fn rtpsession(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_rtpsession(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_rtsp_stream_get_server_port")]
    //#[doc(alias = "get_server_port")]
    //fn server_port(&self, server_port: /*Ignored*/gst_rtsp::RTSPRange, family: gio::SocketFamily) {
    //    unsafe { TODO: call ffi:gst_rtsp_stream_get_server_port() }
    //}

    #[doc(alias = "gst_rtsp_stream_get_sinkpad")]
    #[doc(alias = "get_sinkpad")]
    fn sinkpad(&self) -> Option<gst::Pad> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_sinkpad(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_srcpad")]
    #[doc(alias = "get_srcpad")]
    fn srcpad(&self) -> Option<gst::Pad> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_srcpad(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_srtp_encoder")]
    #[doc(alias = "get_srtp_encoder")]
    fn srtp_encoder(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_get_srtp_encoder(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_ssrc")]
    #[doc(alias = "get_ssrc")]
    fn ssrc(&self) -> u32 {
        unsafe {
            let mut ssrc = std::mem::MaybeUninit::uninit();
            ffi::gst_rtsp_stream_get_ssrc(self.as_ref().to_glib_none().0, ssrc.as_mut_ptr());
            ssrc.assume_init()
        }
    }

    #[doc(alias = "gst_rtsp_stream_get_ulpfec_enabled")]
    #[doc(alias = "get_ulpfec_enabled")]
    fn is_ulpfec_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_get_ulpfec_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_get_ulpfec_percentage")]
    #[doc(alias = "get_ulpfec_percentage")]
    fn ulpfec_percentage(&self) -> u32 {
        unsafe { ffi::gst_rtsp_stream_get_ulpfec_percentage(self.as_ref().to_glib_none().0) }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_get_ulpfec_pt")]
    #[doc(alias = "get_ulpfec_pt")]
    fn ulpfec_pt(&self) -> u32 {
        unsafe { ffi::gst_rtsp_stream_get_ulpfec_pt(self.as_ref().to_glib_none().0) }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_handle_keymgmt")]
    fn handle_keymgmt(&self, keymgmt: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_handle_keymgmt(
                self.as_ref().to_glib_none().0,
                keymgmt.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_has_control")]
    fn has_control(&self, control: Option<&str>) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_has_control(
                self.as_ref().to_glib_none().0,
                control.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_is_bind_mcast_address")]
    fn is_bind_mcast_address(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_is_bind_mcast_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_is_blocking")]
    fn is_blocking(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_is_blocking(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_is_client_side")]
    fn is_client_side(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_is_client_side(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_is_complete")]
    fn is_complete(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_is_complete(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_is_receiver")]
    fn is_receiver(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_is_receiver(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_is_sender")]
    fn is_sender(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_is_sender(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_rtsp_stream_is_transport_supported")]
    //fn is_transport_supported(&self, transport: /*Ignored*/&mut gst_rtsp::RTSPTransport) -> bool {
    //    unsafe { TODO: call ffi:gst_rtsp_stream_is_transport_supported() }
    //}

    #[doc(alias = "gst_rtsp_stream_join_bin")]
    fn join_bin(
        &self,
        bin: &impl IsA<gst::Bin>,
        rtpbin: &impl IsA<gst::Element>,
        state: gst::State,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_stream_join_bin(
                    self.as_ref().to_glib_none().0,
                    bin.as_ref().to_glib_none().0,
                    rtpbin.as_ref().to_glib_none().0,
                    state.into_glib()
                ),
                "Failed to join bin"
            )
        }
    }

    #[doc(alias = "gst_rtsp_stream_leave_bin")]
    fn leave_bin(
        &self,
        bin: &impl IsA<gst::Bin>,
        rtpbin: &impl IsA<gst::Element>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_stream_leave_bin(
                    self.as_ref().to_glib_none().0,
                    bin.as_ref().to_glib_none().0,
                    rtpbin.as_ref().to_glib_none().0
                ),
                "Failed to leave bin"
            )
        }
    }

    #[doc(alias = "gst_rtsp_stream_recv_rtcp")]
    fn recv_rtcp(&self, buffer: gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_rtsp_stream_recv_rtcp(
                self.as_ref().to_glib_none().0,
                buffer.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_recv_rtp")]
    fn recv_rtp(&self, buffer: gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_rtsp_stream_recv_rtp(
                self.as_ref().to_glib_none().0,
                buffer.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_remove_transport")]
    fn remove_transport(
        &self,
        trans: &impl IsA<RTSPStreamTransport>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_stream_remove_transport(
                    self.as_ref().to_glib_none().0,
                    trans.as_ref().to_glib_none().0
                ),
                "Failed to remove transport"
            )
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_request_aux_receiver")]
    fn request_aux_receiver(&self, sessid: u32) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_request_aux_receiver(
                self.as_ref().to_glib_none().0,
                sessid,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_request_aux_sender")]
    fn request_aux_sender(&self, sessid: u32) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_request_aux_sender(
                self.as_ref().to_glib_none().0,
                sessid,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_request_ulpfec_decoder")]
    fn request_ulpfec_decoder(
        &self,
        rtpbin: &impl IsA<gst::Element>,
        sessid: u32,
    ) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_request_ulpfec_decoder(
                self.as_ref().to_glib_none().0,
                rtpbin.as_ref().to_glib_none().0,
                sessid,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_request_ulpfec_encoder")]
    fn request_ulpfec_encoder(&self, sessid: u32) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_request_ulpfec_encoder(
                self.as_ref().to_glib_none().0,
                sessid,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_reserve_address")]
    fn reserve_address(
        &self,
        address: &str,
        port: u32,
        n_ports: u32,
        ttl: u32,
    ) -> Option<RTSPAddress> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_stream_reserve_address(
                self.as_ref().to_glib_none().0,
                address.to_glib_none().0,
                port,
                n_ports,
                ttl,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_seekable")]
    fn seekable(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_seekable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_address_pool")]
    fn set_address_pool(&self, pool: Option<&impl IsA<RTSPAddressPool>>) {
        unsafe {
            ffi::gst_rtsp_stream_set_address_pool(
                self.as_ref().to_glib_none().0,
                pool.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_set_bind_mcast_address")]
    fn set_bind_mcast_address(&self, bind_mcast_addr: bool) {
        unsafe {
            ffi::gst_rtsp_stream_set_bind_mcast_address(
                self.as_ref().to_glib_none().0,
                bind_mcast_addr.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_blocked")]
    fn set_blocked(&self, blocked: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_stream_set_blocked(
                    self.as_ref().to_glib_none().0,
                    blocked.into_glib()
                ),
                "Failed to block/unblock the dataflow"
            )
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_buffer_size")]
    fn set_buffer_size(&self, size: u32) {
        unsafe {
            ffi::gst_rtsp_stream_set_buffer_size(self.as_ref().to_glib_none().0, size);
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_client_side")]
    fn set_client_side(&self, client_side: bool) {
        unsafe {
            ffi::gst_rtsp_stream_set_client_side(
                self.as_ref().to_glib_none().0,
                client_side.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_control")]
    fn set_control(&self, control: Option<&str>) {
        unsafe {
            ffi::gst_rtsp_stream_set_control(
                self.as_ref().to_glib_none().0,
                control.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_dscp_qos")]
    fn set_dscp_qos(&self, dscp_qos: i32) {
        unsafe {
            ffi::gst_rtsp_stream_set_dscp_qos(self.as_ref().to_glib_none().0, dscp_qos);
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_set_max_mcast_ttl")]
    fn set_max_mcast_ttl(&self, ttl: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_set_max_mcast_ttl(
                self.as_ref().to_glib_none().0,
                ttl,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_mtu")]
    fn set_mtu(&self, mtu: u32) {
        unsafe {
            ffi::gst_rtsp_stream_set_mtu(self.as_ref().to_glib_none().0, mtu);
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_multicast_iface")]
    fn set_multicast_iface(&self, multicast_iface: Option<&str>) {
        unsafe {
            ffi::gst_rtsp_stream_set_multicast_iface(
                self.as_ref().to_glib_none().0,
                multicast_iface.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_profiles")]
    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile) {
        unsafe {
            ffi::gst_rtsp_stream_set_profiles(self.as_ref().to_glib_none().0, profiles.into_glib());
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_protocols")]
    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans) {
        unsafe {
            ffi::gst_rtsp_stream_set_protocols(
                self.as_ref().to_glib_none().0,
                protocols.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_pt_map")]
    fn set_pt_map(&self, pt: u32, caps: &gst::Caps) {
        unsafe {
            ffi::gst_rtsp_stream_set_pt_map(
                self.as_ref().to_glib_none().0,
                pt,
                caps.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_publish_clock_mode")]
    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode) {
        unsafe {
            ffi::gst_rtsp_stream_set_publish_clock_mode(
                self.as_ref().to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_stream_set_rate_control")]
    fn set_rate_control(&self, enabled: bool) {
        unsafe {
            ffi::gst_rtsp_stream_set_rate_control(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_retransmission_pt")]
    fn set_retransmission_pt(&self, rtx_pt: u32) {
        unsafe {
            ffi::gst_rtsp_stream_set_retransmission_pt(self.as_ref().to_glib_none().0, rtx_pt);
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_retransmission_time")]
    fn set_retransmission_time(&self, time: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            ffi::gst_rtsp_stream_set_retransmission_time(
                self.as_ref().to_glib_none().0,
                time.into().into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_stream_set_seqnum_offset")]
    fn set_seqnum_offset(&self, seqnum: u16) {
        unsafe {
            ffi::gst_rtsp_stream_set_seqnum_offset(self.as_ref().to_glib_none().0, seqnum);
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_set_ulpfec_percentage")]
    fn set_ulpfec_percentage(&self, percentage: u32) {
        unsafe {
            ffi::gst_rtsp_stream_set_ulpfec_percentage(self.as_ref().to_glib_none().0, percentage);
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_set_ulpfec_pt")]
    fn set_ulpfec_pt(&self, pt: u32) {
        unsafe {
            ffi::gst_rtsp_stream_set_ulpfec_pt(self.as_ref().to_glib_none().0, pt);
        }
    }

    #[doc(alias = "gst_rtsp_stream_transport_filter")]
    fn transport_filter(
        &self,
        func: Option<&mut dyn (FnMut(&RTSPStream, &RTSPStreamTransport) -> RTSPFilterResult)>,
    ) -> Vec<RTSPStreamTransport> {
        let func_data: Option<
            &mut dyn (FnMut(&RTSPStream, &RTSPStreamTransport) -> RTSPFilterResult),
        > = func;
        unsafe extern "C" fn func_func(
            stream: *mut ffi::GstRTSPStream,
            trans: *mut ffi::GstRTSPStreamTransport,
            user_data: glib::ffi::gpointer,
        ) -> ffi::GstRTSPFilterResult {
            let stream = from_glib_borrow(stream);
            let trans = from_glib_borrow(trans);
            let callback = user_data
                as *mut Option<
                    &mut dyn (FnMut(&RTSPStream, &RTSPStreamTransport) -> RTSPFilterResult),
                >;
            if let Some(ref mut callback) = *callback {
                callback(&stream, &trans)
            } else {
                panic!("cannot get closure...")
            }
            .into_glib()
        }
        let func = if func_data.is_some() {
            Some(func_func as _)
        } else {
            None
        };
        let super_callback0: &Option<
            &mut dyn (FnMut(&RTSPStream, &RTSPStreamTransport) -> RTSPFilterResult),
        > = &func_data;
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_rtsp_stream_transport_filter(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as *mut _,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_stream_unblock_linked")]
    fn unblock_linked(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_stream_unblock_linked(self.as_ref().to_glib_none().0),
                "Failed to unblock the dataflow"
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_rtsp_stream_unblock_rtcp")]
    fn unblock_rtcp(&self) {
        unsafe {
            ffi::gst_rtsp_stream_unblock_rtcp(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_rtsp_stream_update_crypto")]
    fn update_crypto(
        &self,
        ssrc: u32,
        crypto: Option<&gst::Caps>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_stream_update_crypto(
                    self.as_ref().to_glib_none().0,
                    ssrc,
                    crypto.to_glib_none().0
                ),
                "Failed to update crypto"
            )
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_stream_verify_mcast_ttl")]
    fn verify_mcast_ttl(&self, ttl: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_stream_verify_mcast_ttl(
                self.as_ref().to_glib_none().0,
                ttl,
            ))
        }
    }

    #[doc(alias = "new-rtcp-encoder")]
    fn connect_new_rtcp_encoder<F: Fn(&Self, &gst::Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn new_rtcp_encoder_trampoline<
            P: IsA<RTSPStream>,
            F: Fn(&P, &gst::Element) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPStream,
            object: *mut gst::ffi::GstElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPStream::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"new-rtcp-encoder\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    new_rtcp_encoder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "new-rtp-encoder")]
    fn connect_new_rtp_encoder<F: Fn(&Self, &gst::Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn new_rtp_encoder_trampoline<
            P: IsA<RTSPStream>,
            F: Fn(&P, &gst::Element) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPStream,
            object: *mut gst::ffi::GstElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPStream::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"new-rtp-encoder\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    new_rtp_encoder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "new-rtp-rtcp-decoder")]
    fn connect_new_rtp_rtcp_decoder<F: Fn(&Self, &gst::Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn new_rtp_rtcp_decoder_trampoline<
            P: IsA<RTSPStream>,
            F: Fn(&P, &gst::Element) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPStream,
            object: *mut gst::ffi::GstElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPStream::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"new-rtp-rtcp-decoder\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    new_rtp_rtcp_decoder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "control")]
    fn connect_control_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_control_trampoline<
            P: IsA<RTSPStream>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::control\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_control_trampoline::<Self, F> as *const (),
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
            P: IsA<RTSPStream>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::profiles\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
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
            P: IsA<RTSPStream>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::protocols\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_protocols_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<RTSPStream>> RTSPStreamExt for O {}
