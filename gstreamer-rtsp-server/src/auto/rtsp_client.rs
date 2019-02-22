// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use RTSPAuth;
use RTSPContext;
use RTSPMountPoints;
use RTSPSession;
use RTSPSessionPool;
use RTSPThreadPool;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use gst_rtsp;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use gst_rtsp_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct RTSPClient(Object<ffi::GstRTSPClient, ffi::GstRTSPClientClass, RTSPClientClass>);

    match fn {
        get_type => || ffi::gst_rtsp_client_get_type(),
    }
}

impl RTSPClient {
    pub fn new() -> RTSPClient {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_client_new())
        }
    }
}

impl Default for RTSPClient {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPClient {}
unsafe impl Sync for RTSPClient {}

pub const NONE_RTSP_CLIENT: Option<&RTSPClient> = None;

pub trait RTSPClientExt: 'static {
    fn close(&self);

    fn get_auth(&self) -> Option<RTSPAuth>;

    //fn get_connection(&self) -> /*Ignored*/Option<gst_rtsp::RTSPConnection>;

    fn get_mount_points(&self) -> Option<RTSPMountPoints>;

    fn get_session_pool(&self) -> Option<RTSPSessionPool>;

    fn get_thread_pool(&self) -> Option<RTSPThreadPool>;

    //fn handle_message(&self, message: /*Ignored*/&mut gst_rtsp::RTSPMessage) -> gst_rtsp::RTSPResult;

    //fn send_message<'a, P: IsA<RTSPSession> + 'a, Q: Into<Option<&'a P>>>(&self, session: Q, message: /*Ignored*/&mut gst_rtsp::RTSPMessage) -> gst_rtsp::RTSPResult;

    //fn session_filter(&self, func: /*Unimplemented*/FnMut(&RTSPClient, &RTSPSession) -> /*Ignored*/RTSPFilterResult, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> Vec<RTSPSession>;

    fn set_auth<'a, P: IsA<RTSPAuth> + 'a, Q: Into<Option<&'a P>>>(&self, auth: Q);

    //fn set_connection(&self, conn: /*Ignored*/&mut gst_rtsp::RTSPConnection) -> bool;

    fn set_mount_points<'a, P: IsA<RTSPMountPoints> + 'a, Q: Into<Option<&'a P>>>(&self, mounts: Q);

    //fn set_send_func(&self, func: /*Unimplemented*/Fn(&RTSPClient, /*Ignored*/gst_rtsp::RTSPMessage, bool) -> bool, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    fn set_session_pool<'a, P: IsA<RTSPSessionPool> + 'a, Q: Into<Option<&'a P>>>(&self, pool: Q);

    fn set_thread_pool<'a, P: IsA<RTSPThreadPool> + 'a, Q: Into<Option<&'a P>>>(&self, pool: Q);

    fn get_property_drop_backlog(&self) -> bool;

    fn set_property_drop_backlog(&self, drop_backlog: bool);

    fn connect_announce_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_check_requirements<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_closed<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_describe_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_get_parameter_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_handle_response<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_new_session<F: Fn(&Self, &RTSPSession) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_options_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_pause_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_play_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_announce_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_describe_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_get_parameter_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_options_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_pause_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_play_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_record_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_set_parameter_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_setup_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_teardown_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_record_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_send_message<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_set_parameter_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_setup_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_teardown_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_drop_backlog_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mount_points_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_session_pool_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPClient>> RTSPClientExt for O {
    fn close(&self) {
        unsafe {
            ffi::gst_rtsp_client_close(self.as_ref().to_glib_none().0);
        }
    }

    fn get_auth(&self) -> Option<RTSPAuth> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_client_get_auth(self.as_ref().to_glib_none().0))
        }
    }

    //fn get_connection(&self) -> /*Ignored*/Option<gst_rtsp::RTSPConnection> {
    //    unsafe { TODO: call ffi::gst_rtsp_client_get_connection() }
    //}

    fn get_mount_points(&self) -> Option<RTSPMountPoints> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_client_get_mount_points(self.as_ref().to_glib_none().0))
        }
    }

    fn get_session_pool(&self) -> Option<RTSPSessionPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_client_get_session_pool(self.as_ref().to_glib_none().0))
        }
    }

    fn get_thread_pool(&self) -> Option<RTSPThreadPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_client_get_thread_pool(self.as_ref().to_glib_none().0))
        }
    }

    //fn handle_message(&self, message: /*Ignored*/&mut gst_rtsp::RTSPMessage) -> gst_rtsp::RTSPResult {
    //    unsafe { TODO: call ffi::gst_rtsp_client_handle_message() }
    //}

    //fn send_message<'a, P: IsA<RTSPSession> + 'a, Q: Into<Option<&'a P>>>(&self, session: Q, message: /*Ignored*/&mut gst_rtsp::RTSPMessage) -> gst_rtsp::RTSPResult {
    //    unsafe { TODO: call ffi::gst_rtsp_client_send_message() }
    //}

    //fn session_filter(&self, func: /*Unimplemented*/FnMut(&RTSPClient, &RTSPSession) -> /*Ignored*/RTSPFilterResult, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> Vec<RTSPSession> {
    //    unsafe { TODO: call ffi::gst_rtsp_client_session_filter() }
    //}

    fn set_auth<'a, P: IsA<RTSPAuth> + 'a, Q: Into<Option<&'a P>>>(&self, auth: Q) {
        let auth = auth.into();
        unsafe {
            ffi::gst_rtsp_client_set_auth(self.as_ref().to_glib_none().0, auth.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    //fn set_connection(&self, conn: /*Ignored*/&mut gst_rtsp::RTSPConnection) -> bool {
    //    unsafe { TODO: call ffi::gst_rtsp_client_set_connection() }
    //}

    fn set_mount_points<'a, P: IsA<RTSPMountPoints> + 'a, Q: Into<Option<&'a P>>>(&self, mounts: Q) {
        let mounts = mounts.into();
        unsafe {
            ffi::gst_rtsp_client_set_mount_points(self.as_ref().to_glib_none().0, mounts.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    //fn set_send_func(&self, func: /*Unimplemented*/Fn(&RTSPClient, /*Ignored*/gst_rtsp::RTSPMessage, bool) -> bool, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gst_rtsp_client_set_send_func() }
    //}

    fn set_session_pool<'a, P: IsA<RTSPSessionPool> + 'a, Q: Into<Option<&'a P>>>(&self, pool: Q) {
        let pool = pool.into();
        unsafe {
            ffi::gst_rtsp_client_set_session_pool(self.as_ref().to_glib_none().0, pool.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_thread_pool<'a, P: IsA<RTSPThreadPool> + 'a, Q: Into<Option<&'a P>>>(&self, pool: Q) {
        let pool = pool.into();
        unsafe {
            ffi::gst_rtsp_client_set_thread_pool(self.as_ref().to_glib_none().0, pool.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn get_property_drop_backlog(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"drop-backlog\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_drop_backlog(&self, drop_backlog: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"drop-backlog\0".as_ptr() as *const _, Value::from(&drop_backlog).to_glib_none().0);
        }
    }

    fn connect_announce_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"announce-request\0".as_ptr() as *const _,
                Some(transmute(announce_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    //fn connect_check_requirements<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype arr: *.CArray TypeId { ns_id: 0, id: 28 }
    //}

    fn connect_closed<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"closed\0".as_ptr() as *const _,
                Some(transmute(closed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_describe_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"describe-request\0".as_ptr() as *const _,
                Some(transmute(describe_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_get_parameter_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"get-parameter-request\0".as_ptr() as *const _,
                Some(transmute(get_parameter_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_handle_response<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"handle-response\0".as_ptr() as *const _,
                Some(transmute(handle_response_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_new_session<F: Fn(&Self, &RTSPSession) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"new-session\0".as_ptr() as *const _,
                Some(transmute(new_session_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_options_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"options-request\0".as_ptr() as *const _,
                Some(transmute(options_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_pause_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pause-request\0".as_ptr() as *const _,
                Some(transmute(pause_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_play_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"play-request\0".as_ptr() as *const _,
                Some(transmute(play_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_announce_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pre-announce-request\0".as_ptr() as *const _,
                Some(transmute(pre_announce_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_describe_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pre-describe-request\0".as_ptr() as *const _,
                Some(transmute(pre_describe_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_get_parameter_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pre-get-parameter-request\0".as_ptr() as *const _,
                Some(transmute(pre_get_parameter_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_options_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pre-options-request\0".as_ptr() as *const _,
                Some(transmute(pre_options_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_pause_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pre-pause-request\0".as_ptr() as *const _,
                Some(transmute(pre_pause_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_play_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pre-play-request\0".as_ptr() as *const _,
                Some(transmute(pre_play_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_record_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pre-record-request\0".as_ptr() as *const _,
                Some(transmute(pre_record_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_set_parameter_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pre-set-parameter-request\0".as_ptr() as *const _,
                Some(transmute(pre_set_parameter_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_setup_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pre-setup-request\0".as_ptr() as *const _,
                Some(transmute(pre_setup_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_pre_teardown_request<F: Fn(&Self, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pre-teardown-request\0".as_ptr() as *const _,
                Some(transmute(pre_teardown_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_record_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"record-request\0".as_ptr() as *const _,
                Some(transmute(record_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    //fn connect_send_message<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored message: GstRtsp.RTSPMessage
    //}

    fn connect_set_parameter_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"set-parameter-request\0".as_ptr() as *const _,
                Some(transmute(set_parameter_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_setup_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"setup-request\0".as_ptr() as *const _,
                Some(transmute(setup_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_teardown_request<F: Fn(&Self, &RTSPContext) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"teardown-request\0".as_ptr() as *const _,
                Some(transmute(teardown_request_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_drop_backlog_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::drop-backlog\0".as_ptr() as *const _,
                Some(transmute(notify_drop_backlog_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_mount_points_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::mount-points\0".as_ptr() as *const _,
                Some(transmute(notify_mount_points_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_session_pool_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::session-pool\0".as_ptr() as *const _,
                Some(transmute(notify_session_pool_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn announce_request_trampoline<P, F: Fn(&P, &RTSPContext) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx))
}

unsafe extern "C" fn closed_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn describe_request_trampoline<P, F: Fn(&P, &RTSPContext) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx))
}

unsafe extern "C" fn get_parameter_request_trampoline<P, F: Fn(&P, &RTSPContext) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx))
}

unsafe extern "C" fn handle_response_trampoline<P, F: Fn(&P, &RTSPContext) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx))
}

unsafe extern "C" fn new_session_trampoline<P, F: Fn(&P, &RTSPSession) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, object: *mut ffi::GstRTSPSession, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(object))
}

unsafe extern "C" fn options_request_trampoline<P, F: Fn(&P, &RTSPContext) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx))
}

unsafe extern "C" fn pause_request_trampoline<P, F: Fn(&P, &RTSPContext) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx))
}

unsafe extern "C" fn play_request_trampoline<P, F: Fn(&P, &RTSPContext) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx))
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn pre_announce_request_trampoline<P, F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer) -> gst_rtsp_ffi::GstRTSPStatusCode
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx)).to_glib()
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn pre_describe_request_trampoline<P, F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer) -> gst_rtsp_ffi::GstRTSPStatusCode
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx)).to_glib()
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn pre_get_parameter_request_trampoline<P, F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer) -> gst_rtsp_ffi::GstRTSPStatusCode
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx)).to_glib()
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn pre_options_request_trampoline<P, F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer) -> gst_rtsp_ffi::GstRTSPStatusCode
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx)).to_glib()
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn pre_pause_request_trampoline<P, F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer) -> gst_rtsp_ffi::GstRTSPStatusCode
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx)).to_glib()
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn pre_play_request_trampoline<P, F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer) -> gst_rtsp_ffi::GstRTSPStatusCode
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx)).to_glib()
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn pre_record_request_trampoline<P, F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer) -> gst_rtsp_ffi::GstRTSPStatusCode
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx)).to_glib()
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn pre_set_parameter_request_trampoline<P, F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer) -> gst_rtsp_ffi::GstRTSPStatusCode
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx)).to_glib()
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn pre_setup_request_trampoline<P, F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer) -> gst_rtsp_ffi::GstRTSPStatusCode
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx)).to_glib()
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
unsafe extern "C" fn pre_teardown_request_trampoline<P, F: Fn(&P, &RTSPContext) -> gst_rtsp::RTSPStatusCode + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer) -> gst_rtsp_ffi::GstRTSPStatusCode
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx)).to_glib()
}

unsafe extern "C" fn record_request_trampoline<P, F: Fn(&P, &RTSPContext) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx))
}

unsafe extern "C" fn set_parameter_request_trampoline<P, F: Fn(&P, &RTSPContext) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx))
}

unsafe extern "C" fn setup_request_trampoline<P, F: Fn(&P, &RTSPContext) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx))
}

unsafe extern "C" fn teardown_request_trampoline<P, F: Fn(&P, &RTSPContext) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, ctx: *mut ffi::GstRTSPContext, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(ctx))
}

unsafe extern "C" fn notify_drop_backlog_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_mount_points_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_session_pool_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(this: *mut ffi::GstRTSPClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPClient> {
    let f: &F = &*(f as *const F);
    f(&RTSPClient::from_glib_borrow(this).unsafe_cast())
}
