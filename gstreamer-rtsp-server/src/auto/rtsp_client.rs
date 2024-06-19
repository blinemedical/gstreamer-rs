// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
use crate::RTSPStreamTransport;
use crate::{
    RTSPAuth, RTSPContext, RTSPFilterResult, RTSPMountPoints, RTSPSession, RTSPSessionPool,
    RTSPThreadPool,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstRTSPClient")]
    pub struct RTSPClient(Object<ffi::GstRTSPClient, ffi::GstRTSPClientClass>);

    match fn {
        type_ => || ffi::gst_rtsp_client_get_type(),
    }
}

impl RTSPClient {
    pub const NONE: Option<&'static RTSPClient> = None;

    #[doc(alias = "gst_rtsp_client_new")]
    pub fn new() -> RTSPClient {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_client_new()) }
    }
}

impl Default for RTSPClient {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPClient {}
unsafe impl Sync for RTSPClient {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTSPClient>> Sealed for T {}
}

pub trait RTSPClientExt: IsA<RTSPClient> + sealed::Sealed + 'static {
    #[doc(alias = "gst_rtsp_client_close")]
    fn close(&self) {
        unsafe {
            ffi::gst_rtsp_client_close(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_rtsp_client_get_auth")]
    #[doc(alias = "get_auth")]
    fn auth(&self) -> Option<RTSPAuth> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_client_get_auth(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_rtsp_client_get_connection")]
    //#[doc(alias = "get_connection")]
    //fn connection(&self) -> /*Ignored*/Option<gst_rtsp::RTSPConnection> {
    //    unsafe { TODO: call ffi:gst_rtsp_client_get_connection() }
    //}

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_client_get_content_length_limit")]
    #[doc(alias = "get_content_length_limit")]
    fn content_length_limit(&self) -> u32 {
        unsafe { ffi::gst_rtsp_client_get_content_length_limit(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_client_get_mount_points")]
    #[doc(alias = "get_mount_points")]
    fn mount_points(&self) -> Option<RTSPMountPoints> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_client_get_mount_points(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_client_get_session_pool")]
    #[doc(alias = "get_session_pool")]
    fn session_pool(&self) -> Option<RTSPSessionPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_client_get_session_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_client_get_stream_transport")]
    #[doc(alias = "get_stream_transport")]
    fn stream_transport(&self, channel: u8) -> Option<RTSPStreamTransport> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_client_get_stream_transport(
                self.as_ref().to_glib_none().0,
                channel,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_client_get_thread_pool")]
    #[doc(alias = "get_thread_pool")]
    fn thread_pool(&self) -> Option<RTSPThreadPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_client_get_thread_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_rtsp_client_handle_message")]
    //fn handle_message(&self, message: /*Ignored*/&mut gst_rtsp::RTSPMessage) -> gst_rtsp::RTSPResult {
    //    unsafe { TODO: call ffi:gst_rtsp_client_handle_message() }
    //}

    //#[doc(alias = "gst_rtsp_client_send_message")]
    //fn send_message(&self, session: Option<&impl IsA<RTSPSession>>, message: /*Ignored*/&mut gst_rtsp::RTSPMessage) -> gst_rtsp::RTSPResult {
    //    unsafe { TODO: call ffi:gst_rtsp_client_send_message() }
    //}

    #[doc(alias = "gst_rtsp_client_session_filter")]
    fn session_filter(
        &self,
        func: Option<&mut dyn (FnMut(&RTSPClient, &RTSPSession) -> RTSPFilterResult)>,
    ) -> Vec<RTSPSession> {
        let func_data: Option<&mut dyn (FnMut(&RTSPClient, &RTSPSession) -> RTSPFilterResult)> =
            func;
        unsafe extern "C" fn func_func(
            client: *mut ffi::GstRTSPClient,
            sess: *mut ffi::GstRTSPSession,
            user_data: glib::ffi::gpointer,
        ) -> ffi::GstRTSPFilterResult {
            let client = from_glib_borrow(client);
            let sess = from_glib_borrow(sess);
            let callback = user_data
                as *mut Option<&mut dyn (FnMut(&RTSPClient, &RTSPSession) -> RTSPFilterResult)>;
            if let Some(ref mut callback) = *callback {
                callback(&client, &sess)
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
            &mut dyn (FnMut(&RTSPClient, &RTSPSession) -> RTSPFilterResult),
        > = &func_data;
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_rtsp_client_session_filter(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as *mut _,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_client_set_auth")]
    fn set_auth(&self, auth: Option<&impl IsA<RTSPAuth>>) {
        unsafe {
            ffi::gst_rtsp_client_set_auth(
                self.as_ref().to_glib_none().0,
                auth.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "gst_rtsp_client_set_connection")]
    //fn set_connection(&self, conn: /*Ignored*/gst_rtsp::RTSPConnection) -> bool {
    //    unsafe { TODO: call ffi:gst_rtsp_client_set_connection() }
    //}

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_client_set_content_length_limit")]
    fn set_content_length_limit(&self, limit: u32) {
        unsafe {
            ffi::gst_rtsp_client_set_content_length_limit(self.as_ref().to_glib_none().0, limit);
        }
    }

    #[doc(alias = "gst_rtsp_client_set_mount_points")]
    fn set_mount_points(&self, mounts: Option<&impl IsA<RTSPMountPoints>>) {
        unsafe {
            ffi::gst_rtsp_client_set_mount_points(
                self.as_ref().to_glib_none().0,
                mounts.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    //#[cfg(feature = "v1_16")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    //#[doc(alias = "gst_rtsp_client_set_send_messages_func")]
    //fn set_send_messages_func(&self, func: /*Unimplemented*/Fn(&RTSPClient, /*Ignored*/gst_rtsp::RTSPMessage, u32, bool) -> bool, user_data: /*Unimplemented*/Option<Basic: Pointer>) {
    //    unsafe { TODO: call ffi:gst_rtsp_client_set_send_messages_func() }
    //}

    #[doc(alias = "gst_rtsp_client_set_session_pool")]
    fn set_session_pool(&self, pool: Option<&impl IsA<RTSPSessionPool>>) {
        unsafe {
            ffi::gst_rtsp_client_set_session_pool(
                self.as_ref().to_glib_none().0,
                pool.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_rtsp_client_set_thread_pool")]
    fn set_thread_pool(&self, pool: Option<&impl IsA<RTSPThreadPool>>) {
        unsafe {
            ffi::gst_rtsp_client_set_thread_pool(
                self.as_ref().to_glib_none().0,
                pool.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "drop-backlog")]
    fn is_drop_backlog(&self) -> bool {
        ObjectExt::property(self.as_ref(), "drop-backlog")
    }

    #[doc(alias = "drop-backlog")]
    fn set_drop_backlog(&self, drop_backlog: bool) {
        ObjectExt::set_property(self.as_ref(), "drop-backlog", drop_backlog)
    }

    #[doc(alias = "post-session-timeout")]
    fn post_session_timeout(&self) -> i32 {
        ObjectExt::property(self.as_ref(), "post-session-timeout")
    }

    #[doc(alias = "post-session-timeout")]
    fn set_post_session_timeout(&self, post_session_timeout: i32) {
        ObjectExt::set_property(self.as_ref(), "post-session-timeout", post_session_timeout)
    }

    #[doc(alias = "announce-request")]
    fn connect_announce_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn announce_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"announce-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    announce_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //#[doc(alias = "check-requirements")]
    //fn connect_check_requirements<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype arr: *.CArray TypeId { ns_id: 0, id: 28 }
    //}

    #[doc(alias = "closed")]
    fn connect_closed<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    closed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "describe-request")]
    fn connect_describe_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn describe_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"describe-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    describe_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "get-parameter-request")]
    fn connect_get_parameter_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn get_parameter_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"get-parameter-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    get_parameter_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "handle-response")]
    fn connect_handle_response<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn handle_response_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"handle-response\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    handle_response_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "new-session")]
    fn connect_new_session<F: Fn(&Self, &RTSPSession) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn new_session_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPSession) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            object: *mut ffi::GstRTSPSession,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"new-session\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    new_session_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "options-request")]
    fn connect_options_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn options_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"options-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    options_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pause-request")]
    fn connect_pause_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pause_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pause-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pause_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "play-request")]
    fn connect_play_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn play_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"play-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    play_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pre-announce-request")]
    fn connect_pre_announce_request<
        F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pre_announce_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) -> gst_rtsp::ffi::GstRTSPStatusCode {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-announce-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pre_announce_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pre-describe-request")]
    fn connect_pre_describe_request<
        F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pre_describe_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) -> gst_rtsp::ffi::GstRTSPStatusCode {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-describe-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pre_describe_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pre-get-parameter-request")]
    fn connect_pre_get_parameter_request<
        F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pre_get_parameter_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) -> gst_rtsp::ffi::GstRTSPStatusCode {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-get-parameter-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pre_get_parameter_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pre-options-request")]
    fn connect_pre_options_request<
        F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pre_options_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) -> gst_rtsp::ffi::GstRTSPStatusCode {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-options-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pre_options_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pre-pause-request")]
    fn connect_pre_pause_request<
        F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pre_pause_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) -> gst_rtsp::ffi::GstRTSPStatusCode {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-pause-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pre_pause_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pre-play-request")]
    fn connect_pre_play_request<
        F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pre_play_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) -> gst_rtsp::ffi::GstRTSPStatusCode {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-play-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pre_play_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pre-record-request")]
    fn connect_pre_record_request<
        F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pre_record_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) -> gst_rtsp::ffi::GstRTSPStatusCode {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-record-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pre_record_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pre-set-parameter-request")]
    fn connect_pre_set_parameter_request<
        F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pre_set_parameter_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) -> gst_rtsp::ffi::GstRTSPStatusCode {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-set-parameter-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pre_set_parameter_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pre-setup-request")]
    fn connect_pre_setup_request<
        F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pre_setup_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) -> gst_rtsp::ffi::GstRTSPStatusCode {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-setup-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pre_setup_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pre-teardown-request")]
    fn connect_pre_teardown_request<
        F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pre_teardown_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) -> gst_rtsp::ffi::GstRTSPStatusCode {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-teardown-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    pre_teardown_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "record-request")]
    fn connect_record_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn record_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"record-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    record_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //#[doc(alias = "send-message")]
    //fn connect_send_message<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored message: GstRtsp.RTSPMessage
    //}

    #[doc(alias = "set-parameter-request")]
    fn connect_set_parameter_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn set_parameter_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"set-parameter-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    set_parameter_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "setup-request")]
    fn connect_setup_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn setup_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"setup-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    setup_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "teardown-request")]
    fn connect_teardown_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn teardown_request_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P, &RTSPContext) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            ctx: *mut ffi::GstRTSPContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPClient::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(ctx),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"teardown-request\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    teardown_request_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "drop-backlog")]
    fn connect_drop_backlog_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_drop_backlog_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::drop-backlog\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_drop_backlog_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mount-points")]
    fn connect_mount_points_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mount_points_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mount-points\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_mount_points_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "post-session-timeout")]
    fn connect_post_session_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_post_session_timeout_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::post-session-timeout\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_post_session_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "session-pool")]
    fn connect_session_pool_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_session_pool_trampoline<
            P: IsA<RTSPClient>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPClient,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPClient::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::session-pool\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_session_pool_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<RTSPClient>> RTSPClientExt for O {}
