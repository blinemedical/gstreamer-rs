// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RTSPAuth;
use crate::RTSPClient;
use crate::RTSPFilterResult;
use crate::RTSPMountPoints;
use crate::RTSPSessionPool;
use crate::RTSPThreadPool;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib::glib_wrapper! {
    pub struct RTSPServer(Object<ffi::GstRTSPServer, ffi::GstRTSPServerClass>);

    match fn {
        get_type => || ffi::gst_rtsp_server_get_type(),
    }
}

impl RTSPServer {
    #[doc(alias = "gst_rtsp_server_new")]
    pub fn new() -> RTSPServer {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_server_new()) }
    }

    #[doc(alias = "gst_rtsp_server_io_func")]
    pub fn io_func<P: IsA<gio::Socket>, Q: IsA<RTSPServer>>(
        socket: &P,
        condition: glib::IOCondition,
        server: &Q,
    ) -> Result<(), glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::gst_rtsp_server_io_func(
                    socket.as_ref().to_glib_none().0,
                    condition.to_glib(),
                    server.as_ref().to_glib_none().0
                ),
                "Failed to connect the source"
            )
        }
    }
}

impl Default for RTSPServer {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPServer {}
unsafe impl Sync for RTSPServer {}

pub const NONE_RTSP_SERVER: Option<&RTSPServer> = None;

pub trait RTSPServerExt: 'static {
    #[doc(alias = "gst_rtsp_server_client_filter")]
    fn client_filter(
        &self,
        func: Option<&mut dyn (FnMut(&RTSPServer, &RTSPClient) -> RTSPFilterResult)>,
    ) -> Vec<RTSPClient>;

    #[doc(alias = "gst_rtsp_server_create_socket")]
    fn create_socket<P: IsA<gio::Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<gio::Socket, glib::Error>;

    #[doc(alias = "gst_rtsp_server_create_source")]
    fn create_source<P: IsA<gio::Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<glib::Source, glib::Error>;

    #[doc(alias = "gst_rtsp_server_get_address")]
    fn get_address(&self) -> Option<glib::GString>;

    #[doc(alias = "gst_rtsp_server_get_auth")]
    fn get_auth(&self) -> Option<RTSPAuth>;

    #[doc(alias = "gst_rtsp_server_get_backlog")]
    fn get_backlog(&self) -> i32;

    #[doc(alias = "gst_rtsp_server_get_bound_port")]
    fn get_bound_port(&self) -> i32;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_server_get_content_length_limit")]
    fn get_content_length_limit(&self) -> u32;

    #[doc(alias = "gst_rtsp_server_get_mount_points")]
    fn get_mount_points(&self) -> Option<RTSPMountPoints>;

    #[doc(alias = "gst_rtsp_server_get_service")]
    fn get_service(&self) -> Option<glib::GString>;

    #[doc(alias = "gst_rtsp_server_get_session_pool")]
    fn get_session_pool(&self) -> Option<RTSPSessionPool>;

    #[doc(alias = "gst_rtsp_server_get_thread_pool")]
    fn get_thread_pool(&self) -> Option<RTSPThreadPool>;

    #[doc(alias = "gst_rtsp_server_set_address")]
    fn set_address(&self, address: &str);

    #[doc(alias = "gst_rtsp_server_set_auth")]
    fn set_auth<P: IsA<RTSPAuth>>(&self, auth: Option<&P>);

    #[doc(alias = "gst_rtsp_server_set_backlog")]
    fn set_backlog(&self, backlog: i32);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_rtsp_server_set_content_length_limit")]
    fn set_content_length_limit(&self, limit: u32);

    #[doc(alias = "gst_rtsp_server_set_mount_points")]
    fn set_mount_points<P: IsA<RTSPMountPoints>>(&self, mounts: Option<&P>);

    #[doc(alias = "gst_rtsp_server_set_service")]
    fn set_service(&self, service: &str);

    #[doc(alias = "gst_rtsp_server_set_session_pool")]
    fn set_session_pool<P: IsA<RTSPSessionPool>>(&self, pool: Option<&P>);

    #[doc(alias = "gst_rtsp_server_set_thread_pool")]
    fn set_thread_pool<P: IsA<RTSPThreadPool>>(&self, pool: Option<&P>);

    #[doc(alias = "gst_rtsp_server_transfer_connection")]
    fn transfer_connection<P: IsA<gio::Socket>>(
        &self,
        socket: &P,
        ip: &str,
        port: i32,
        initial_buffer: Option<&str>,
    ) -> Result<(), glib::error::BoolError>;

    fn get_property_content_length_limit(&self) -> u32;

    fn set_property_content_length_limit(&self, content_length_limit: u32);

    fn connect_client_connected<F: Fn(&Self, &RTSPClient) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_address_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_backlog_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_bound_port_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_content_length_limit_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_mount_points_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_service_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_session_pool_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<RTSPServer>> RTSPServerExt for O {
    fn client_filter(
        &self,
        func: Option<&mut dyn (FnMut(&RTSPServer, &RTSPClient) -> RTSPFilterResult)>,
    ) -> Vec<RTSPClient> {
        let func_data: Option<&mut dyn (FnMut(&RTSPServer, &RTSPClient) -> RTSPFilterResult)> =
            func;
        unsafe extern "C" fn func_func(
            server: *mut ffi::GstRTSPServer,
            client: *mut ffi::GstRTSPClient,
            user_data: glib::ffi::gpointer,
        ) -> ffi::GstRTSPFilterResult {
            let server = from_glib_borrow(server);
            let client = from_glib_borrow(client);
            let callback: *mut Option<
                &mut dyn (FnMut(&RTSPServer, &RTSPClient) -> RTSPFilterResult),
            > = user_data as *const _ as usize
                as *mut Option<&mut dyn (FnMut(&RTSPServer, &RTSPClient) -> RTSPFilterResult)>;
            let res = if let Some(ref mut callback) = *callback {
                callback(&server, &client)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let func = if func_data.is_some() {
            Some(func_func as _)
        } else {
            None
        };
        let super_callback0: &Option<
            &mut dyn (FnMut(&RTSPServer, &RTSPClient) -> RTSPFilterResult),
        > = &func_data;
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_rtsp_server_client_filter(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    fn create_socket<P: IsA<gio::Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<gio::Socket, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_rtsp_server_create_socket(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_source<P: IsA<gio::Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<glib::Source, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_rtsp_server_create_source(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_address(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_auth(&self) -> Option<RTSPAuth> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_auth(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_backlog(&self) -> i32 {
        unsafe { ffi::gst_rtsp_server_get_backlog(self.as_ref().to_glib_none().0) }
    }

    fn get_bound_port(&self) -> i32 {
        unsafe { ffi::gst_rtsp_server_get_bound_port(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_content_length_limit(&self) -> u32 {
        unsafe { ffi::gst_rtsp_server_get_content_length_limit(self.as_ref().to_glib_none().0) }
    }

    fn get_mount_points(&self) -> Option<RTSPMountPoints> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_mount_points(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_service(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_service(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_session_pool(&self) -> Option<RTSPSessionPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_session_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_thread_pool(&self) -> Option<RTSPThreadPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_server_get_thread_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_address(&self, address: &str) {
        unsafe {
            ffi::gst_rtsp_server_set_address(
                self.as_ref().to_glib_none().0,
                address.to_glib_none().0,
            );
        }
    }

    fn set_auth<P: IsA<RTSPAuth>>(&self, auth: Option<&P>) {
        unsafe {
            ffi::gst_rtsp_server_set_auth(
                self.as_ref().to_glib_none().0,
                auth.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_backlog(&self, backlog: i32) {
        unsafe {
            ffi::gst_rtsp_server_set_backlog(self.as_ref().to_glib_none().0, backlog);
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_content_length_limit(&self, limit: u32) {
        unsafe {
            ffi::gst_rtsp_server_set_content_length_limit(self.as_ref().to_glib_none().0, limit);
        }
    }

    fn set_mount_points<P: IsA<RTSPMountPoints>>(&self, mounts: Option<&P>) {
        unsafe {
            ffi::gst_rtsp_server_set_mount_points(
                self.as_ref().to_glib_none().0,
                mounts.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_service(&self, service: &str) {
        unsafe {
            ffi::gst_rtsp_server_set_service(
                self.as_ref().to_glib_none().0,
                service.to_glib_none().0,
            );
        }
    }

    fn set_session_pool<P: IsA<RTSPSessionPool>>(&self, pool: Option<&P>) {
        unsafe {
            ffi::gst_rtsp_server_set_session_pool(
                self.as_ref().to_glib_none().0,
                pool.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_thread_pool<P: IsA<RTSPThreadPool>>(&self, pool: Option<&P>) {
        unsafe {
            ffi::gst_rtsp_server_set_thread_pool(
                self.as_ref().to_glib_none().0,
                pool.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn transfer_connection<P: IsA<gio::Socket>>(
        &self,
        socket: &P,
        ip: &str,
        port: i32,
        initial_buffer: Option<&str>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                ffi::gst_rtsp_server_transfer_connection(
                    self.as_ref().to_glib_none().0,
                    socket.as_ref().to_glib_full(),
                    ip.to_glib_none().0,
                    port,
                    initial_buffer.to_glib_none().0
                ),
                "Failed to transfer to the connection"
            )
        }
    }

    fn get_property_content_length_limit(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"content-length-limit\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `content-length-limit` getter")
                .unwrap()
        }
    }

    fn set_property_content_length_limit(&self, content_length_limit: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"content-length-limit\0".as_ptr() as *const _,
                glib::Value::from(&content_length_limit).to_glib_none().0,
            );
        }
    }

    fn connect_client_connected<F: Fn(&Self, &RTSPClient) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn client_connected_trampoline<
            P,
            F: Fn(&P, &RTSPClient) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPServer,
            object: *mut ffi::GstRTSPClient,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RTSPServer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &RTSPServer::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"client-connected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    client_connected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_address_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_address_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstRTSPServer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RTSPServer>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPServer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_backlog_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_backlog_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstRTSPServer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RTSPServer>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPServer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::backlog\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_backlog_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_bound_port_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_bound_port_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstRTSPServer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RTSPServer>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPServer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bound-port\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bound_port_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_content_length_limit_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_length_limit_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPServer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RTSPServer>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPServer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::content-length-limit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_content_length_limit_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mount_points_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mount_points_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstRTSPServer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RTSPServer>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPServer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mount-points\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mount_points_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_service_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_service_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstRTSPServer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RTSPServer>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPServer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::service\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_service_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_session_pool_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_session_pool_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstRTSPServer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<RTSPServer>,
        {
            let f: &F = &*(f as *const F);
            f(&RTSPServer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::session-pool\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_session_pool_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
