// Generated by gir (https://github.com/gtk-rs/gir @ 54e116a11822)
// from gir-files (https://github.com/gtk-rs/gir-files @ df20f22974b6)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ e421156aab30)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Constants
pub const GST_GL_DISPLAY_EGL_NAME: *const c_char =
    b"gst.gl.display.egl\0" as *const u8 as *const c_char;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstGLDisplayEGLClass {
    pub object_class: gst_gl::GstGLDisplayClass,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayEGLClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstGLDisplayEGLClass @ {:p}", self))
            .field("object_class", &self.object_class)
            .field("_padding", &self._padding)
            .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstGLDisplayEGL {
    pub parent: gst_gl::GstGLDisplay,
    pub display: gpointer,
    pub foreign_display: gboolean,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayEGL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstGLDisplayEGL @ {:p}", self))
            .field("parent", &self.parent)
            .finish()
    }
}

#[link(name = "gstgl-1.0")]
extern "C" {

    //=========================================================================
    // GstGLDisplayEGL
    //=========================================================================
    pub fn gst_gl_display_egl_get_type() -> GType;
    pub fn gst_gl_display_egl_new() -> *mut GstGLDisplayEGL;
    pub fn gst_gl_display_egl_new_with_egl_display(display: gpointer) -> *mut GstGLDisplayEGL;
    pub fn gst_gl_display_egl_from_gl_display(
        display: *mut gst_gl::GstGLDisplay,
    ) -> *mut GstGLDisplayEGL;
    pub fn gst_gl_display_egl_get_from_native(
        type_: gst_gl::GstGLDisplayType,
        display: uintptr_t,
    ) -> gpointer;

}
