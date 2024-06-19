// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::AudioInfo;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstAudioDecoder")]
    pub struct AudioDecoder(Object<ffi::GstAudioDecoder, ffi::GstAudioDecoderClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_audio_decoder_get_type(),
    }
}

impl AudioDecoder {
    pub const NONE: Option<&'static AudioDecoder> = None;
}

unsafe impl Send for AudioDecoder {}
unsafe impl Sync for AudioDecoder {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AudioDecoder>> Sealed for T {}
}

pub trait AudioDecoderExt: IsA<AudioDecoder> + sealed::Sealed + 'static {
    #[doc(alias = "gst_audio_decoder_allocate_output_buffer")]
    fn allocate_output_buffer(&self, size: usize) -> gst::Buffer {
        unsafe {
            from_glib_full(ffi::gst_audio_decoder_allocate_output_buffer(
                self.as_ref().to_glib_none().0,
                size,
            ))
        }
    }

    #[doc(alias = "gst_audio_decoder_finish_frame")]
    fn finish_frame(
        &self,
        buf: Option<gst::Buffer>,
        frames: i32,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_audio_decoder_finish_frame(
                self.as_ref().to_glib_none().0,
                buf.into_glib_ptr(),
                frames,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_audio_decoder_finish_subframe")]
    fn finish_subframe(
        &self,
        buf: Option<gst::Buffer>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_audio_decoder_finish_subframe(
                self.as_ref().to_glib_none().0,
                buf.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_audio_decoder_get_audio_info")]
    #[doc(alias = "get_audio_info")]
    fn audio_info(&self) -> AudioInfo {
        unsafe {
            from_glib_none(ffi::gst_audio_decoder_get_audio_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_audio_decoder_get_delay")]
    #[doc(alias = "get_delay")]
    fn delay(&self) -> i32 {
        unsafe { ffi::gst_audio_decoder_get_delay(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_audio_decoder_get_drainable")]
    #[doc(alias = "get_drainable")]
    fn is_drainable(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_decoder_get_drainable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_audio_decoder_get_estimate_rate")]
    #[doc(alias = "get_estimate_rate")]
    fn estimate_rate(&self) -> i32 {
        unsafe { ffi::gst_audio_decoder_get_estimate_rate(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_audio_decoder_get_latency")]
    #[doc(alias = "get_latency")]
    fn latency(&self) -> (gst::ClockTime, Option<gst::ClockTime>) {
        unsafe {
            let mut min = std::mem::MaybeUninit::uninit();
            let mut max = std::mem::MaybeUninit::uninit();
            ffi::gst_audio_decoder_get_latency(
                self.as_ref().to_glib_none().0,
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            (
                try_from_glib(min.assume_init()).expect("mandatory glib value is None"),
                from_glib(max.assume_init()),
            )
        }
    }

    #[doc(alias = "gst_audio_decoder_get_max_errors")]
    #[doc(alias = "get_max_errors")]
    fn max_errors(&self) -> i32 {
        unsafe { ffi::gst_audio_decoder_get_max_errors(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_audio_decoder_get_min_latency")]
    #[doc(alias = "get_min_latency")]
    fn min_latency(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::gst_audio_decoder_get_min_latency(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    #[doc(alias = "gst_audio_decoder_get_needs_format")]
    #[doc(alias = "get_needs_format")]
    fn needs_format(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_decoder_get_needs_format(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_audio_decoder_get_parse_state")]
    #[doc(alias = "get_parse_state")]
    fn parse_state(&self) -> (bool, bool) {
        unsafe {
            let mut sync = std::mem::MaybeUninit::uninit();
            let mut eos = std::mem::MaybeUninit::uninit();
            ffi::gst_audio_decoder_get_parse_state(
                self.as_ref().to_glib_none().0,
                sync.as_mut_ptr(),
                eos.as_mut_ptr(),
            );
            (from_glib(sync.assume_init()), from_glib(eos.assume_init()))
        }
    }

    #[doc(alias = "gst_audio_decoder_get_plc")]
    #[doc(alias = "get_plc")]
    fn is_plc(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_decoder_get_plc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_audio_decoder_get_plc_aware")]
    #[doc(alias = "get_plc_aware")]
    fn plc_aware(&self) -> i32 {
        unsafe { ffi::gst_audio_decoder_get_plc_aware(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_audio_decoder_get_tolerance")]
    #[doc(alias = "get_tolerance")]
    fn tolerance(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::gst_audio_decoder_get_tolerance(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    #[doc(alias = "gst_audio_decoder_merge_tags")]
    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode) {
        unsafe {
            ffi::gst_audio_decoder_merge_tags(
                self.as_ref().to_glib_none().0,
                tags.to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_audio_decoder_proxy_getcaps")]
    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_audio_decoder_proxy_getcaps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_audio_decoder_set_allocation_caps")]
    fn set_allocation_caps(&self, allocation_caps: Option<&gst::Caps>) {
        unsafe {
            ffi::gst_audio_decoder_set_allocation_caps(
                self.as_ref().to_glib_none().0,
                allocation_caps.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_audio_decoder_set_drainable")]
    fn set_drainable(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_drainable(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_audio_decoder_set_estimate_rate")]
    fn set_estimate_rate(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_estimate_rate(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_audio_decoder_set_latency")]
    fn set_latency(&self, min: gst::ClockTime, max: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            ffi::gst_audio_decoder_set_latency(
                self.as_ref().to_glib_none().0,
                min.into_glib(),
                max.into().into_glib(),
            );
        }
    }

    #[doc(alias = "gst_audio_decoder_set_max_errors")]
    fn set_max_errors(&self, num: i32) {
        unsafe {
            ffi::gst_audio_decoder_set_max_errors(self.as_ref().to_glib_none().0, num);
        }
    }

    #[doc(alias = "gst_audio_decoder_set_min_latency")]
    fn set_min_latency(&self, num: gst::ClockTime) {
        unsafe {
            ffi::gst_audio_decoder_set_min_latency(self.as_ref().to_glib_none().0, num.into_glib());
        }
    }

    #[doc(alias = "gst_audio_decoder_set_needs_format")]
    fn set_needs_format(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_needs_format(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_audio_decoder_set_plc")]
    fn set_plc(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_plc(self.as_ref().to_glib_none().0, enabled.into_glib());
        }
    }

    #[doc(alias = "gst_audio_decoder_set_plc_aware")]
    fn set_plc_aware(&self, plc: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_plc_aware(self.as_ref().to_glib_none().0, plc.into_glib());
        }
    }

    #[doc(alias = "gst_audio_decoder_set_tolerance")]
    fn set_tolerance(&self, tolerance: gst::ClockTime) {
        unsafe {
            ffi::gst_audio_decoder_set_tolerance(
                self.as_ref().to_glib_none().0,
                tolerance.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_audio_decoder_set_use_default_pad_acceptcaps")]
    fn set_use_default_pad_acceptcaps(&self, use_: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_use_default_pad_acceptcaps(
                self.as_ref().to_glib_none().0,
                use_.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "max-errors")]
    fn connect_max_errors_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_errors_trampoline<
            P: IsA<AudioDecoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-errors\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_errors_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-latency")]
    fn connect_min_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_latency_trampoline<
            P: IsA<AudioDecoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-latency\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_min_latency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "plc")]
    fn connect_plc_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_plc_trampoline<
            P: IsA<AudioDecoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::plc\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_plc_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tolerance")]
    fn connect_tolerance_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tolerance_trampoline<
            P: IsA<AudioDecoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tolerance\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_tolerance_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<AudioDecoder>> AudioDecoderExt for O {}
