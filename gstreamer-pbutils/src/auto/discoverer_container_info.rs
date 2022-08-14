// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::DiscovererStreamInfo;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstDiscovererContainerInfo")]
    pub struct DiscovererContainerInfo(Object<ffi::GstDiscovererContainerInfo>) @extends DiscovererStreamInfo;

    match fn {
        type_ => || ffi::gst_discoverer_container_info_get_type(),
    }
}

impl DiscovererContainerInfo {
    #[doc(alias = "gst_discoverer_container_info_get_streams")]
    #[doc(alias = "get_streams")]
    pub fn streams(&self) -> Vec<DiscovererStreamInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_discoverer_container_info_get_streams(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_discoverer_container_info_get_tags")]
    #[doc(alias = "get_tags")]
    pub fn tags(&self) -> Option<gst::TagList> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_container_info_get_tags(
                self.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for DiscovererContainerInfo {}
unsafe impl Sync for DiscovererContainerInfo {}
