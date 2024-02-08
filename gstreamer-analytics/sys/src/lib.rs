// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use glib_sys as glib;
use gstreamer_sys as gst;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Aliases
pub type GstAnalyticsMtdType = uintptr_t;

// Constants
pub const GST_AN_RELATION_META_TAG: &[u8] = b"GST-ANALYSIS-RELATION-META-TAG\0";
pub const GST_INF_RELATION_SPAN: c_int = -1;
pub const GST_ANALYTICS_MTD_TYPE_ANY: c_int = 0;

// Flags
pub type GstAnalyticsRelTypes = c_uint;
pub const GST_ANALYTICS_REL_TYPE_NONE: GstAnalyticsRelTypes = 0;
pub const GST_ANALYTICS_REL_TYPE_IS_PART_OF: GstAnalyticsRelTypes = 2;
pub const GST_ANALYTICS_REL_TYPE_CONTAIN: GstAnalyticsRelTypes = 4;
pub const GST_ANALYTICS_REL_TYPE_RELATE_TO: GstAnalyticsRelTypes = 8;
pub const GST_ANALYTICS_REL_TYPE_LAST: GstAnalyticsRelTypes = 16;
pub const GST_ANALYTICS_REL_TYPE_ANY: GstAnalyticsRelTypes = 2147483647;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAnalyticsClsMtd {
    pub id: c_uint,
    pub meta: *mut GstAnalyticsRelationMeta,
}

impl ::std::fmt::Debug for GstAnalyticsClsMtd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAnalyticsClsMtd @ {self:p}"))
            .field("id", &self.id)
            .field("meta", &self.meta)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAnalyticsMtd {
    pub id: c_uint,
    pub meta: *mut GstAnalyticsRelationMeta,
}

impl ::std::fmt::Debug for GstAnalyticsMtd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAnalyticsMtd @ {self:p}"))
            .field("id", &self.id)
            .field("meta", &self.meta)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAnalyticsMtdImpl {
    pub name: *const c_char,
    pub mtd_meta_transform: Option<
        unsafe extern "C" fn(
            *mut gst::GstBuffer,
            *mut GstAnalyticsMtd,
            *mut gst::GstBuffer,
            glib::GQuark,
            gpointer,
        ) -> gboolean,
    >,
    pub _reserved: [gpointer; 20],
}

impl ::std::fmt::Debug for GstAnalyticsMtdImpl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAnalyticsMtdImpl @ {self:p}"))
            .field("name", &self.name)
            .field("mtd_meta_transform", &self.mtd_meta_transform)
            .field("_reserved", &self._reserved)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAnalyticsODMtd {
    pub id: c_uint,
    pub meta: *mut GstAnalyticsRelationMeta,
}

impl ::std::fmt::Debug for GstAnalyticsODMtd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAnalyticsODMtd @ {self:p}"))
            .field("id", &self.id)
            .field("meta", &self.meta)
            .finish()
    }
}

#[repr(C)]
pub struct _GstAnalyticsRelationMeta {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstAnalyticsRelationMeta = _GstAnalyticsRelationMeta;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAnalyticsRelationMetaInitParams {
    pub initial_relation_order: size_t,
    pub initial_buf_size: size_t,
}

impl ::std::fmt::Debug for GstAnalyticsRelationMetaInitParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAnalyticsRelationMetaInitParams @ {self:p}"))
            .field("initial_relation_order", &self.initial_relation_order)
            .field("initial_buf_size", &self.initial_buf_size)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstAnalyticsTrackingMtd {
    pub id: c_uint,
    pub meta: *mut GstAnalyticsRelationMeta,
}

impl ::std::fmt::Debug for GstAnalyticsTrackingMtd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstAnalyticsTrackingMtd @ {self:p}"))
            .field("id", &self.id)
            .field("meta", &self.meta)
            .finish()
    }
}

#[link(name = "gstanalytics-1.0")]
extern "C" {

    //=========================================================================
    // GstAnalyticsClsMtd
    //=========================================================================
    pub fn gst_analytics_cls_mtd_get_index_by_quark(
        handle: *mut GstAnalyticsClsMtd,
        quark: glib::GQuark,
    ) -> c_int;
    pub fn gst_analytics_cls_mtd_get_length(handle: *mut GstAnalyticsClsMtd) -> size_t;
    pub fn gst_analytics_cls_mtd_get_level(
        handle: *mut GstAnalyticsClsMtd,
        index: size_t,
    ) -> c_float;
    pub fn gst_analytics_cls_mtd_get_quark(
        handle: *mut GstAnalyticsClsMtd,
        index: size_t,
    ) -> glib::GQuark;
    pub fn gst_analytics_cls_mtd_get_mtd_type() -> GstAnalyticsMtdType;

    //=========================================================================
    // GstAnalyticsMtd
    //=========================================================================
    pub fn gst_analytics_mtd_get_id(instance: *mut GstAnalyticsMtd) -> c_uint;
    pub fn gst_analytics_mtd_get_mtd_type(instance: *mut GstAnalyticsMtd) -> GstAnalyticsMtdType;
    pub fn gst_analytics_mtd_get_size(instance: *mut GstAnalyticsMtd) -> size_t;
    pub fn gst_analytics_mtd_type_get_name(type_: GstAnalyticsMtdType) -> *const c_char;

    //=========================================================================
    // GstAnalyticsODMtd
    //=========================================================================
    pub fn gst_analytics_od_mtd_get_confidence_lvl(
        instance: *mut GstAnalyticsODMtd,
        loc_conf_lvl: *mut c_float,
    ) -> gboolean;
    pub fn gst_analytics_od_mtd_get_location(
        instance: *mut GstAnalyticsODMtd,
        x: *mut c_int,
        y: *mut c_int,
        w: *mut c_int,
        h: *mut c_int,
        loc_conf_lvl: *mut c_float,
    ) -> gboolean;
    pub fn gst_analytics_od_mtd_get_obj_type(handle: *mut GstAnalyticsODMtd) -> glib::GQuark;
    pub fn gst_analytics_od_mtd_get_mtd_type() -> GstAnalyticsMtdType;

    //=========================================================================
    // GstAnalyticsRelationMeta
    //=========================================================================
    pub fn gst_analytics_relation_meta_add_cls_mtd(
        instance: *mut GstAnalyticsRelationMeta,
        length: size_t,
        confidence_levels: *mut c_float,
        class_quarks: *mut glib::GQuark,
        cls_mtd: *mut GstAnalyticsClsMtd,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_add_mtd(
        meta: *mut GstAnalyticsRelationMeta,
        impl_: *const GstAnalyticsMtdImpl,
        size: size_t,
        rlt_mtd: *mut GstAnalyticsMtd,
    ) -> gpointer;
    pub fn gst_analytics_relation_meta_add_od_mtd(
        instance: *mut GstAnalyticsRelationMeta,
        type_: glib::GQuark,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        loc_conf_lvl: c_float,
        od_mtd: *mut GstAnalyticsODMtd,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_add_one_cls_mtd(
        instance: *mut GstAnalyticsRelationMeta,
        confidence_level: c_float,
        class_quark: glib::GQuark,
        cls_mtd: *mut GstAnalyticsClsMtd,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_add_tracking_mtd(
        instance: *mut GstAnalyticsRelationMeta,
        tracking_id: u64,
        tracking_first_seen: gst::GstClockTime,
        trk_mtd: *mut GstAnalyticsTrackingMtd,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_exist(
        rmeta: *mut GstAnalyticsRelationMeta,
        an_meta_first_id: c_uint,
        an_meta_second_id: c_uint,
        max_relation_span: c_int,
        cond_types: GstAnalyticsRelTypes,
        relations_path: *mut *mut glib::GArray,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_get_cls_mtd(
        meta: *mut GstAnalyticsRelationMeta,
        an_meta_id: c_uint,
        rlt: *mut GstAnalyticsClsMtd,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_get_direct_related(
        meta: *mut GstAnalyticsRelationMeta,
        an_meta_id: c_uint,
        relation_type: GstAnalyticsRelTypes,
        type_: GstAnalyticsMtdType,
        state: *mut gpointer,
        rlt_mtd: *mut GstAnalyticsMtd,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_get_mtd(
        meta: *mut GstAnalyticsRelationMeta,
        an_meta_id: c_uint,
        type_: GstAnalyticsMtdType,
        rlt: *mut GstAnalyticsMtd,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_get_mtd_data(
        meta: *mut GstAnalyticsRelationMeta,
        an_meta_id: c_uint,
    ) -> gpointer;
    pub fn gst_analytics_relation_meta_get_od_mtd(
        meta: *mut GstAnalyticsRelationMeta,
        an_meta_id: c_uint,
        rlt: *mut GstAnalyticsODMtd,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_get_relation(
        meta: *mut GstAnalyticsRelationMeta,
        an_meta_first_id: c_uint,
        an_meta_second_id: c_uint,
    ) -> GstAnalyticsRelTypes;
    pub fn gst_analytics_relation_meta_get_tracking_mtd(
        meta: *mut GstAnalyticsRelationMeta,
        an_meta_id: c_uint,
        rlt: *mut GstAnalyticsTrackingMtd,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_iterate(
        meta: *mut GstAnalyticsRelationMeta,
        state: *mut gpointer,
        type_: GstAnalyticsMtdType,
        rlt_mtd: *mut GstAnalyticsMtd,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_set_relation(
        meta: *mut GstAnalyticsRelationMeta,
        type_: GstAnalyticsRelTypes,
        an_meta_first_id: c_uint,
        an_meta_second_id: c_uint,
    ) -> gboolean;
    pub fn gst_analytics_relation_meta_get_info() -> *const gst::GstMetaInfo;

    //=========================================================================
    // GstAnalyticsTrackingMtd
    //=========================================================================
    pub fn gst_analytics_tracking_mtd_get_info(
        instance: *mut GstAnalyticsTrackingMtd,
        tracking_id: *mut u64,
        tracking_first_seen: *mut gst::GstClockTime,
        tracking_last_seen: *mut gst::GstClockTime,
        tracking_lost: *mut gboolean,
    ) -> gboolean;
    pub fn gst_analytics_tracking_mtd_set_lost(instance: *mut GstAnalyticsTrackingMtd) -> gboolean;
    pub fn gst_analytics_tracking_mtd_update_last_seen(
        instance: *mut GstAnalyticsTrackingMtd,
        last_seen: gst::GstClockTime,
    ) -> gboolean;
    pub fn gst_analytics_tracking_mtd_get_mtd_type() -> GstAnalyticsMtdType;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gst_buffer_add_analytics_relation_meta(
        buffer: *mut gst::GstBuffer,
    ) -> *mut GstAnalyticsRelationMeta;
    pub fn gst_buffer_add_analytics_relation_meta_full(
        buffer: *mut gst::GstBuffer,
        init_params: *mut GstAnalyticsRelationMetaInitParams,
    ) -> *mut GstAnalyticsRelationMeta;
    pub fn gst_buffer_get_analytics_relation_meta(
        buffer: *mut gst::GstBuffer,
    ) -> *mut GstAnalyticsRelationMeta;
    pub fn gst_analytics_relation_get_length(instance: *mut GstAnalyticsRelationMeta) -> size_t;
    pub fn gst_analytics_relation_meta_api_get_type() -> GType;

}