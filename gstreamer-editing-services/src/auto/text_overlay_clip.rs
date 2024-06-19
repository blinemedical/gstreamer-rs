// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{
    Clip, Container, Extractable, MetaContainer, OperationClip, OverlayClip, TextHAlign,
    TextVAlign, TimelineElement,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GESTextOverlayClip")]
    pub struct TextOverlayClip(Object<ffi::GESTextOverlayClip, ffi::GESTextOverlayClipClass>) @extends OverlayClip, OperationClip, Clip, Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_text_overlay_clip_get_type(),
    }
}

impl TextOverlayClip {
    pub const NONE: Option<&'static TextOverlayClip> = None;

    #[doc(alias = "ges_text_overlay_clip_new")]
    pub fn new() -> Option<TextOverlayClip> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_text_overlay_clip_new()) }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::TextOverlayClip>> Sealed for T {}
}

pub trait TextOverlayClipExt: IsA<TextOverlayClip> + sealed::Sealed + 'static {
    #[doc(alias = "ges_text_overlay_clip_get_color")]
    #[doc(alias = "get_color")]
    fn color(&self) -> u32 {
        unsafe { ffi::ges_text_overlay_clip_get_color(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_text_overlay_clip_get_font_desc")]
    #[doc(alias = "get_font_desc")]
    fn font_desc(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ges_text_overlay_clip_get_font_desc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_text_overlay_clip_get_halignment")]
    #[doc(alias = "get_halignment")]
    fn halignment(&self) -> TextHAlign {
        unsafe {
            from_glib(ffi::ges_text_overlay_clip_get_halignment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_text_overlay_clip_get_text")]
    #[doc(alias = "get_text")]
    fn text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ges_text_overlay_clip_get_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_text_overlay_clip_get_valignment")]
    #[doc(alias = "get_valignment")]
    fn valignment(&self) -> TextVAlign {
        unsafe {
            from_glib(ffi::ges_text_overlay_clip_get_valignment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_text_overlay_clip_get_xpos")]
    #[doc(alias = "get_xpos")]
    fn xpos(&self) -> f64 {
        unsafe { ffi::ges_text_overlay_clip_get_xpos(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_text_overlay_clip_get_ypos")]
    #[doc(alias = "get_ypos")]
    fn ypos(&self) -> f64 {
        unsafe { ffi::ges_text_overlay_clip_get_ypos(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_text_overlay_clip_set_color")]
    fn set_color(&self, color: u32) {
        unsafe {
            ffi::ges_text_overlay_clip_set_color(self.as_ref().to_glib_none().0, color);
        }
    }

    #[doc(alias = "ges_text_overlay_clip_set_font_desc")]
    fn set_font_desc(&self, font_desc: Option<&str>) {
        unsafe {
            ffi::ges_text_overlay_clip_set_font_desc(
                self.as_ref().to_glib_none().0,
                font_desc.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "ges_text_overlay_clip_set_halign")]
    fn set_halign(&self, halign: TextHAlign) {
        unsafe {
            ffi::ges_text_overlay_clip_set_halign(
                self.as_ref().to_glib_none().0,
                halign.into_glib(),
            );
        }
    }

    #[doc(alias = "ges_text_overlay_clip_set_text")]
    fn set_text(&self, text: Option<&str>) {
        unsafe {
            ffi::ges_text_overlay_clip_set_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "ges_text_overlay_clip_set_valign")]
    fn set_valign(&self, valign: TextVAlign) {
        unsafe {
            ffi::ges_text_overlay_clip_set_valign(
                self.as_ref().to_glib_none().0,
                valign.into_glib(),
            );
        }
    }

    #[doc(alias = "ges_text_overlay_clip_set_xpos")]
    fn set_xpos(&self, position: f64) {
        unsafe {
            ffi::ges_text_overlay_clip_set_xpos(self.as_ref().to_glib_none().0, position);
        }
    }

    #[doc(alias = "ges_text_overlay_clip_set_ypos")]
    fn set_ypos(&self, position: f64) {
        unsafe {
            ffi::ges_text_overlay_clip_set_ypos(self.as_ref().to_glib_none().0, position);
        }
    }

    fn set_halignment(&self, halignment: TextHAlign) {
        ObjectExt::set_property(self.as_ref(), "halignment", halignment)
    }

    fn set_valignment(&self, valignment: TextVAlign) {
        ObjectExt::set_property(self.as_ref(), "valignment", valignment)
    }

    #[doc(alias = "color")]
    fn connect_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_color_trampoline<
            P: IsA<TextOverlayClip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTextOverlayClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TextOverlayClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::color\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_color_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "font-desc")]
    fn connect_font_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_desc_trampoline<
            P: IsA<TextOverlayClip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTextOverlayClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TextOverlayClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::font-desc\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_font_desc_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "halignment")]
    fn connect_halignment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_halignment_trampoline<
            P: IsA<TextOverlayClip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTextOverlayClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TextOverlayClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::halignment\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_halignment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "text")]
    fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<
            P: IsA<TextOverlayClip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTextOverlayClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TextOverlayClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "valignment")]
    fn connect_valignment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_valignment_trampoline<
            P: IsA<TextOverlayClip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTextOverlayClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TextOverlayClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::valignment\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_valignment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "xpos")]
    fn connect_xpos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xpos_trampoline<
            P: IsA<TextOverlayClip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTextOverlayClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TextOverlayClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xpos\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_xpos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "ypos")]
    fn connect_ypos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ypos_trampoline<
            P: IsA<TextOverlayClip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTextOverlayClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TextOverlayClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ypos\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_ypos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<TextOverlayClip>> TextOverlayClipExt for O {}
