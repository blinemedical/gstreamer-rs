// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
use crate::Plugin;
use glib::object::IsA;
use glib::translate::*;

glib::glib_wrapper! {
    pub struct PluginFeature(Object<ffi::GstPluginFeature, ffi::GstPluginFeatureClass>) @extends Object;

    match fn {
        get_type => || ffi::gst_plugin_feature_get_type(),
    }
}

unsafe impl Send for PluginFeature {}
unsafe impl Sync for PluginFeature {}

pub const NONE_PLUGIN_FEATURE: Option<&PluginFeature> = None;

pub trait PluginFeatureExt: 'static {
    #[doc(alias = "gst_plugin_feature_check_version")]
    fn check_version(&self, min_major: u32, min_minor: u32, min_micro: u32) -> bool;

    #[doc(alias = "gst_plugin_feature_get_plugin")]
    fn get_plugin(&self) -> Option<Plugin>;

    #[doc(alias = "gst_plugin_feature_get_plugin_name")]
    fn get_plugin_name(&self) -> Option<glib::GString>;

    #[doc(alias = "gst_plugin_feature_load")]
    fn load(&self) -> Result<PluginFeature, glib::BoolError>;
}

impl<O: IsA<PluginFeature>> PluginFeatureExt for O {
    fn check_version(&self, min_major: u32, min_minor: u32, min_micro: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_plugin_feature_check_version(
                self.as_ref().to_glib_none().0,
                min_major,
                min_minor,
                min_micro,
            ))
        }
    }

    fn get_plugin(&self) -> Option<Plugin> {
        unsafe {
            from_glib_full(ffi::gst_plugin_feature_get_plugin(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_plugin_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_plugin_feature_get_plugin_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn load(&self) -> Result<PluginFeature, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_plugin_feature_load(
                self.as_ref().to_glib_none().0,
            ))
            .ok_or_else(|| glib::glib_bool_error!("Failed to load plugin feature"))
        }
    }
}
