// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
use crate::PluginDependencyFlags;
use glib::translate::*;
use std::ptr;

glib::glib_wrapper! {
    pub struct Plugin(Object<ffi::GstPlugin, ffi::GstPluginClass>) @extends Object;

    match fn {
        get_type => || ffi::gst_plugin_get_type(),
    }
}

impl Plugin {
    #[doc(alias = "gst_plugin_add_dependency")]
    pub fn add_dependency(
        &self,
        env_vars: &[&str],
        paths: &[&str],
        names: &[&str],
        flags: PluginDependencyFlags,
    ) {
        unsafe {
            ffi::gst_plugin_add_dependency(
                self.to_glib_none().0,
                env_vars.to_glib_none().0,
                paths.to_glib_none().0,
                names.to_glib_none().0,
                flags.to_glib(),
            );
        }
    }

    #[doc(alias = "gst_plugin_add_dependency_simple")]
    pub fn add_dependency_simple(
        &self,
        env_vars: Option<&str>,
        paths: Option<&str>,
        names: Option<&str>,
        flags: PluginDependencyFlags,
    ) {
        unsafe {
            ffi::gst_plugin_add_dependency_simple(
                self.to_glib_none().0,
                env_vars.to_glib_none().0,
                paths.to_glib_none().0,
                names.to_glib_none().0,
                flags.to_glib(),
            );
        }
    }

    #[doc(alias = "gst_plugin_get_description")]
    pub fn get_description(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gst_plugin_get_description(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_plugin_get_filename")]
    pub fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe { from_glib_none(ffi::gst_plugin_get_filename(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_plugin_get_license")]
    pub fn get_license(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gst_plugin_get_license(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_plugin_get_origin")]
    pub fn get_origin(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gst_plugin_get_origin(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_plugin_get_package")]
    pub fn get_package(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gst_plugin_get_package(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_plugin_get_release_date_string")]
    pub fn get_release_date_string(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_plugin_get_release_date_string(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_plugin_get_source")]
    pub fn get_source(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gst_plugin_get_source(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_plugin_get_version")]
    pub fn get_version(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gst_plugin_get_version(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_plugin_is_loaded")]
    pub fn is_loaded(&self) -> bool {
        unsafe { from_glib(ffi::gst_plugin_is_loaded(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_plugin_load")]
    pub fn load(&self) -> Result<Plugin, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_plugin_load(self.to_glib_none().0))
                .ok_or_else(|| glib::glib_bool_error!("Failed to load plugin"))
        }
    }

    #[doc(alias = "gst_plugin_load_by_name")]
    pub fn load_by_name(name: &str) -> Result<Plugin, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_plugin_load_by_name(name.to_glib_none().0))
                .ok_or_else(|| glib::glib_bool_error!("Failed to load plugin"))
        }
    }

    #[doc(alias = "gst_plugin_load_file")]
    pub fn load_file<P: AsRef<std::path::Path>>(filename: P) -> Result<Plugin, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_plugin_load_file(filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl Send for Plugin {}
unsafe impl Sync for Plugin {}
