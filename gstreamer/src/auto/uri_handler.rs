// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::URIType;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

glib::glib_wrapper! {
    pub struct URIHandler(Interface<ffi::GstURIHandler>);

    match fn {
        get_type => || ffi::gst_uri_handler_get_type(),
    }
}

unsafe impl Send for URIHandler {}
unsafe impl Sync for URIHandler {}

pub const NONE_URI_HANDLER: Option<&URIHandler> = None;

pub trait URIHandlerExt: 'static {
    #[doc(alias = "gst_uri_handler_get_protocols")]
    fn get_protocols(&self) -> Vec<glib::GString>;

    #[doc(alias = "gst_uri_handler_get_uri")]
    fn get_uri(&self) -> Option<glib::GString>;

    #[doc(alias = "gst_uri_handler_get_uri_type")]
    fn get_uri_type(&self) -> URIType;

    #[doc(alias = "gst_uri_handler_set_uri")]
    fn set_uri(&self, uri: &str) -> Result<(), glib::Error>;
}

impl<O: IsA<URIHandler>> URIHandlerExt for O {
    fn get_protocols(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_uri_handler_get_protocols(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gst_uri_handler_get_uri(self.as_ref().to_glib_none().0)) }
    }

    fn get_uri_type(&self) -> URIType {
        unsafe {
            from_glib(ffi::gst_uri_handler_get_uri_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_uri(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gst_uri_handler_set_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
