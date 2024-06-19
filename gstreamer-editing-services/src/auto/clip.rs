// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
use crate::FrameNumber;
use crate::{
    Asset, BaseEffect, Container, Extractable, Layer, MetaContainer, TimelineElement, Track,
    TrackElement, TrackType,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GESClip")]
    pub struct Clip(Object<ffi::GESClip, ffi::GESClipClass>) @extends Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_clip_get_type(),
    }
}

impl Clip {
    pub const NONE: Option<&'static Clip> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Clip>> Sealed for T {}
}

pub trait ClipExt: IsA<Clip> + sealed::Sealed + 'static {
    #[doc(alias = "ges_clip_add_asset")]
    fn add_asset(&self, asset: &impl IsA<Asset>) -> Result<TrackElement, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::ges_clip_add_asset(
                self.as_ref().to_glib_none().0,
                asset.as_ref().to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to add asset"))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_add_child_to_track")]
    fn add_child_to_track(
        &self,
        child: &impl IsA<TrackElement>,
        track: &impl IsA<Track>,
    ) -> Result<TrackElement, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::ges_clip_add_child_to_track(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                track.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_add_top_effect")]
    fn add_top_effect(&self, effect: &impl IsA<BaseEffect>, index: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_clip_add_top_effect(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
                index,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "ges_clip_find_track_element")]
    fn find_track_element(
        &self,
        track: Option<&impl IsA<Track>>,
        type_: glib::types::Type,
    ) -> Option<TrackElement> {
        unsafe {
            from_glib_full(ffi::ges_clip_find_track_element(
                self.as_ref().to_glib_none().0,
                track.map(|p| p.as_ref()).to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_clip_find_track_elements")]
    fn find_track_elements(
        &self,
        track: Option<&impl IsA<Track>>,
        track_type: TrackType,
        type_: glib::types::Type,
    ) -> Vec<TrackElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_clip_find_track_elements(
                self.as_ref().to_glib_none().0,
                track.map(|p| p.as_ref()).to_glib_none().0,
                track_type.into_glib(),
                type_.into_glib(),
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_get_duration_limit")]
    #[doc(alias = "get_duration_limit")]
    fn duration_limit(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::ges_clip_get_duration_limit(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_get_internal_time_from_timeline_time")]
    #[doc(alias = "get_internal_time_from_timeline_time")]
    fn internal_time_from_timeline_time(
        &self,
        child: &impl IsA<TrackElement>,
        timeline_time: impl Into<Option<gst::ClockTime>>,
    ) -> Result<Option<gst::ClockTime>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::ges_clip_get_internal_time_from_timeline_time(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                timeline_time.into().into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "ges_clip_get_layer")]
    #[doc(alias = "get_layer")]
    fn layer(&self) -> Option<Layer> {
        unsafe { from_glib_full(ffi::ges_clip_get_layer(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "ges_clip_get_supported_formats")]
    #[doc(alias = "get_supported_formats")]
    fn supported_formats(&self) -> TrackType {
        unsafe {
            from_glib(ffi::ges_clip_get_supported_formats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_get_timeline_time_from_internal_time")]
    #[doc(alias = "get_timeline_time_from_internal_time")]
    fn timeline_time_from_internal_time(
        &self,
        child: &impl IsA<TrackElement>,
        internal_time: impl Into<Option<gst::ClockTime>>,
    ) -> Result<Option<gst::ClockTime>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::ges_clip_get_timeline_time_from_internal_time(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                internal_time.into().into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_get_timeline_time_from_source_frame")]
    #[doc(alias = "get_timeline_time_from_source_frame")]
    fn timeline_time_from_source_frame(
        &self,
        frame_number: FrameNumber,
    ) -> Result<Option<gst::ClockTime>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::ges_clip_get_timeline_time_from_source_frame(
                self.as_ref().to_glib_none().0,
                frame_number,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "ges_clip_get_top_effect_index")]
    #[doc(alias = "get_top_effect_index")]
    fn top_effect_index(&self, effect: &impl IsA<BaseEffect>) -> i32 {
        unsafe {
            ffi::ges_clip_get_top_effect_index(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
            )
        }
    }

    #[doc(alias = "ges_clip_get_top_effect_position")]
    #[doc(alias = "get_top_effect_position")]
    fn top_effect_position(&self, effect: &impl IsA<BaseEffect>) -> i32 {
        unsafe {
            ffi::ges_clip_get_top_effect_position(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
            )
        }
    }

    #[doc(alias = "ges_clip_get_top_effects")]
    #[doc(alias = "get_top_effects")]
    fn top_effects(&self) -> Vec<TrackElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_clip_get_top_effects(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_clip_move_to_layer")]
    fn move_to_layer(&self, layer: &impl IsA<Layer>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_clip_move_to_layer(
                    self.as_ref().to_glib_none().0,
                    layer.as_ref().to_glib_none().0
                ),
                "Failed to move clip to specified layer"
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_move_to_layer_full")]
    fn move_to_layer_full(&self, layer: &impl IsA<Layer>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_clip_move_to_layer_full(
                self.as_ref().to_glib_none().0,
                layer.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_remove_top_effect")]
    fn remove_top_effect(&self, effect: &impl IsA<BaseEffect>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_clip_remove_top_effect(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "ges_clip_set_supported_formats")]
    fn set_supported_formats(&self, supportedformats: TrackType) {
        unsafe {
            ffi::ges_clip_set_supported_formats(
                self.as_ref().to_glib_none().0,
                supportedformats.into_glib(),
            );
        }
    }

    #[doc(alias = "ges_clip_set_top_effect_index")]
    fn set_top_effect_index(
        &self,
        effect: &impl IsA<BaseEffect>,
        newindex: u32,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_clip_set_top_effect_index(
                    self.as_ref().to_glib_none().0,
                    effect.as_ref().to_glib_none().0,
                    newindex
                ),
                "Failed to move effect"
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_set_top_effect_index_full")]
    fn set_top_effect_index_full(
        &self,
        effect: &impl IsA<BaseEffect>,
        newindex: u32,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_clip_set_top_effect_index_full(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
                newindex,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "ges_clip_set_top_effect_priority")]
    fn set_top_effect_priority(
        &self,
        effect: &impl IsA<BaseEffect>,
        newpriority: u32,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_clip_set_top_effect_priority(
                    self.as_ref().to_glib_none().0,
                    effect.as_ref().to_glib_none().0,
                    newpriority
                ),
                "Failed to the set top effect priority"
            )
        }
    }

    #[doc(alias = "ges_clip_split")]
    fn split(&self, position: u64) -> Result<Clip, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::ges_clip_split(
                self.as_ref().to_glib_none().0,
                position,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to split clip"))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_clip_split_full")]
    fn split_full(&self, position: u64) -> Result<Option<Clip>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::ges_clip_split_full(self.as_ref().to_glib_none().0, position, &mut error);
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "duration-limit")]
    fn connect_duration_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_limit_trampoline<P: IsA<Clip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Clip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration-limit\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_duration_limit_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "layer")]
    fn connect_layer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_layer_trampoline<P: IsA<Clip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Clip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::layer\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_layer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "supported-formats")]
    fn connect_supported_formats_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_supported_formats_trampoline<
            P: IsA<Clip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Clip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::supported-formats\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_supported_formats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Clip>> ClipExt for O {}
