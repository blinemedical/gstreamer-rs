// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct AggregatorPad(Object<ffi::GstAggregatorPad, ffi::GstAggregatorPadClass>) @extends gst::Pad, gst::Object;

    match fn {
        get_type => || ffi::gst_aggregator_pad_get_type(),
    }
}

unsafe impl Send for AggregatorPad {}
unsafe impl Sync for AggregatorPad {}

pub const NONE_AGGREGATOR_PAD: Option<&AggregatorPad> = None;

pub trait AggregatorPadExt: 'static {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "gst_aggregator_pad_drop_buffer")]
    fn drop_buffer(&self) -> bool;

    #[cfg(any(feature = "v1_14_1", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14_1")))]
    #[doc(alias = "gst_aggregator_pad_has_buffer")]
    fn has_buffer(&self) -> bool;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "gst_aggregator_pad_is_eos")]
    fn is_eos(&self) -> bool;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "gst_aggregator_pad_peek_buffer")]
    fn peek_buffer(&self) -> Option<gst::Buffer>;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "gst_aggregator_pad_pop_buffer")]
    fn pop_buffer(&self) -> Option<gst::Buffer>;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn get_property_emit_signals(&self) -> bool;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn set_property_emit_signals(&self, emit_signals: bool);

    fn connect_buffer_consumed<F: Fn(&Self, &gst::Buffer) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn connect_property_emit_signals_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<AggregatorPad>> AggregatorPadExt for O {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn drop_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_aggregator_pad_drop_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14_1", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14_1")))]
    fn has_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_aggregator_pad_has_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn is_eos(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_aggregator_pad_is_eos(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn peek_buffer(&self) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(ffi::gst_aggregator_pad_peek_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn pop_buffer(&self) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(ffi::gst_aggregator_pad_pop_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn get_property_emit_signals(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"emit-signals\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `emit-signals` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn set_property_emit_signals(&self, emit_signals: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"emit-signals\0".as_ptr() as *const _,
                glib::Value::from(&emit_signals).to_glib_none().0,
            );
        }
    }

    fn connect_buffer_consumed<F: Fn(&Self, &gst::Buffer) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn buffer_consumed_trampoline<
            P,
            F: Fn(&P, &gst::Buffer) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAggregatorPad,
            object: *mut gst::ffi::GstBuffer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<AggregatorPad>,
        {
            let f: &F = &*(f as *const F);
            f(
                &AggregatorPad::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"buffer-consumed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    buffer_consumed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    fn connect_property_emit_signals_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_emit_signals_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstAggregatorPad,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<AggregatorPad>,
        {
            let f: &F = &*(f as *const F);
            f(&AggregatorPad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::emit-signals\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_emit_signals_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
