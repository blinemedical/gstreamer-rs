// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use crate::GLContext;
use crate::GLSLProfile;
use glib::error::ErrorDomain;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use glib::object::IsA;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::Quark;
use glib::StaticType;
use glib::Type;
use std::mem;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GLContextError {
    Failed,
    WrongConfig,
    WrongApi,
    OldLibs,
    CreateContext,
    ResourceUnavailable,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for GLContextError {
    type GlibType = ffi::GstGLContextError;

    fn to_glib(&self) -> ffi::GstGLContextError {
        match *self {
            GLContextError::Failed => ffi::GST_GL_CONTEXT_ERROR_FAILED,
            GLContextError::WrongConfig => ffi::GST_GL_CONTEXT_ERROR_WRONG_CONFIG,
            GLContextError::WrongApi => ffi::GST_GL_CONTEXT_ERROR_WRONG_API,
            GLContextError::OldLibs => ffi::GST_GL_CONTEXT_ERROR_OLD_LIBS,
            GLContextError::CreateContext => ffi::GST_GL_CONTEXT_ERROR_CREATE_CONTEXT,
            GLContextError::ResourceUnavailable => ffi::GST_GL_CONTEXT_ERROR_RESOURCE_UNAVAILABLE,
            GLContextError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLContextError> for GLContextError {
    unsafe fn from_glib(value: ffi::GstGLContextError) -> Self {
        skip_assert_initialized!();
        match value {
            0 => GLContextError::Failed,
            1 => GLContextError::WrongConfig,
            2 => GLContextError::WrongApi,
            3 => GLContextError::OldLibs,
            4 => GLContextError::CreateContext,
            5 => GLContextError::ResourceUnavailable,
            value => GLContextError::__Unknown(value),
        }
    }
}

impl ErrorDomain for GLContextError {
    fn domain() -> Quark {
        skip_assert_initialized!();

        unsafe { from_glib(ffi::gst_gl_context_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            0 => Some(GLContextError::Failed),
            1 => Some(GLContextError::WrongConfig),
            2 => Some(GLContextError::WrongApi),
            3 => Some(GLContextError::OldLibs),
            4 => Some(GLContextError::CreateContext),
            5 => Some(GLContextError::ResourceUnavailable),
            _ => Some(GLContextError::Failed),
        }
    }
}

impl StaticType for GLContextError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_context_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GLContextError {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GLContextError {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GLContextError {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GLFormat {
    Luminance,
    Alpha,
    LuminanceAlpha,
    Red,
    R8,
    Rg,
    Rg8,
    Rgb,
    Rgb8,
    Rgb565,
    Rgb16,
    Rgba,
    Rgba8,
    Rgba16,
    DepthComponent16,
    Depth24Stencil8,
    Rgb10A2,
    R16,
    Rg16,
    #[doc(hidden)]
    __Unknown(i32),
}

impl GLFormat {
    //#[doc(alias = "gst_gl_format_from_video_info")]
    //pub fn from_video_info<P: IsA<GLContext>>(context: &P, vinfo: /*Ignored*/&mut gst_video::VideoInfo, plane: u32) -> GLFormat {
    //    unsafe { TODO: call ffi:gst_gl_format_from_video_info() }
    //}

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_gl_format_is_supported")]
    pub fn is_supported<P: IsA<GLContext>>(context: &P, format: GLFormat) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_gl_format_is_supported(
                context.as_ref().to_glib_none().0,
                format.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_gl_format_type_from_sized_gl_format")]
    pub fn type_from_sized_gl_format(self) -> (GLFormat, u32) {
        assert_initialized_main_thread!();
        unsafe {
            let mut unsized_format = mem::MaybeUninit::uninit();
            let mut gl_type = mem::MaybeUninit::uninit();
            ffi::gst_gl_format_type_from_sized_gl_format(
                self.to_glib(),
                unsized_format.as_mut_ptr(),
                gl_type.as_mut_ptr(),
            );
            let unsized_format = unsized_format.assume_init();
            let gl_type = gl_type.assume_init();
            (from_glib(unsized_format), gl_type)
        }
    }

    #[doc(alias = "gst_gl_format_type_n_bytes")]
    pub fn type_n_bytes(format: u32, type_: u32) -> u32 {
        assert_initialized_main_thread!();
        unsafe { ffi::gst_gl_format_type_n_bytes(format, type_) }
    }
}

#[doc(hidden)]
impl ToGlib for GLFormat {
    type GlibType = ffi::GstGLFormat;

    fn to_glib(&self) -> ffi::GstGLFormat {
        match *self {
            GLFormat::Luminance => ffi::GST_GL_LUMINANCE,
            GLFormat::Alpha => ffi::GST_GL_ALPHA,
            GLFormat::LuminanceAlpha => ffi::GST_GL_LUMINANCE_ALPHA,
            GLFormat::Red => ffi::GST_GL_RED,
            GLFormat::R8 => ffi::GST_GL_R8,
            GLFormat::Rg => ffi::GST_GL_RG,
            GLFormat::Rg8 => ffi::GST_GL_RG8,
            GLFormat::Rgb => ffi::GST_GL_RGB,
            GLFormat::Rgb8 => ffi::GST_GL_RGB8,
            GLFormat::Rgb565 => ffi::GST_GL_RGB565,
            GLFormat::Rgb16 => ffi::GST_GL_RGB16,
            GLFormat::Rgba => ffi::GST_GL_RGBA,
            GLFormat::Rgba8 => ffi::GST_GL_RGBA8,
            GLFormat::Rgba16 => ffi::GST_GL_RGBA16,
            GLFormat::DepthComponent16 => ffi::GST_GL_DEPTH_COMPONENT16,
            GLFormat::Depth24Stencil8 => ffi::GST_GL_DEPTH24_STENCIL8,
            GLFormat::Rgb10A2 => ffi::GST_GL_RGB10_A2,
            GLFormat::R16 => ffi::GST_GL_R16,
            GLFormat::Rg16 => ffi::GST_GL_RG16,
            GLFormat::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLFormat> for GLFormat {
    unsafe fn from_glib(value: ffi::GstGLFormat) -> Self {
        skip_assert_initialized!();
        match value {
            6409 => GLFormat::Luminance,
            6406 => GLFormat::Alpha,
            6410 => GLFormat::LuminanceAlpha,
            6403 => GLFormat::Red,
            33321 => GLFormat::R8,
            33319 => GLFormat::Rg,
            33323 => GLFormat::Rg8,
            6407 => GLFormat::Rgb,
            32849 => GLFormat::Rgb8,
            36194 => GLFormat::Rgb565,
            32852 => GLFormat::Rgb16,
            6408 => GLFormat::Rgba,
            32856 => GLFormat::Rgba8,
            32859 => GLFormat::Rgba16,
            33189 => GLFormat::DepthComponent16,
            35056 => GLFormat::Depth24Stencil8,
            32857 => GLFormat::Rgb10A2,
            33322 => GLFormat::R16,
            33324 => GLFormat::Rg16,
            value => GLFormat::__Unknown(value),
        }
    }
}

impl StaticType for GLFormat {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_format_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GLFormat {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GLFormat {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GLFormat {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GLQueryType {
    None,
    TimeElapsed,
    Timestamp,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for GLQueryType {
    type GlibType = ffi::GstGLQueryType;

    fn to_glib(&self) -> ffi::GstGLQueryType {
        match *self {
            GLQueryType::None => ffi::GST_GL_QUERY_NONE,
            GLQueryType::TimeElapsed => ffi::GST_GL_QUERY_TIME_ELAPSED,
            GLQueryType::Timestamp => ffi::GST_GL_QUERY_TIMESTAMP,
            GLQueryType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLQueryType> for GLQueryType {
    unsafe fn from_glib(value: ffi::GstGLQueryType) -> Self {
        skip_assert_initialized!();
        match value {
            0 => GLQueryType::None,
            1 => GLQueryType::TimeElapsed,
            2 => GLQueryType::Timestamp,
            value => GLQueryType::__Unknown(value),
        }
    }
}

impl StaticType for GLQueryType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_query_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GLQueryType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GLQueryType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GLQueryType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GLSLError {
    Compile,
    Link,
    Program,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for GLSLError {
    type GlibType = ffi::GstGLSLError;

    fn to_glib(&self) -> ffi::GstGLSLError {
        match *self {
            GLSLError::Compile => ffi::GST_GLSL_ERROR_COMPILE,
            GLSLError::Link => ffi::GST_GLSL_ERROR_LINK,
            GLSLError::Program => ffi::GST_GLSL_ERROR_PROGRAM,
            GLSLError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLSLError> for GLSLError {
    unsafe fn from_glib(value: ffi::GstGLSLError) -> Self {
        skip_assert_initialized!();
        match value {
            0 => GLSLError::Compile,
            1 => GLSLError::Link,
            2 => GLSLError::Program,
            value => GLSLError::__Unknown(value),
        }
    }
}

impl ErrorDomain for GLSLError {
    fn domain() -> Quark {
        skip_assert_initialized!();

        unsafe { from_glib(ffi::gst_glsl_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            0 => Some(GLSLError::Compile),
            1 => Some(GLSLError::Link),
            2 => Some(GLSLError::Program),
            value => Some(GLSLError::__Unknown(value)),
        }
    }
}

impl StaticType for GLSLError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_glsl_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GLSLError {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GLSLError {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GLSLError {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GLSLVersion {
    None,
    _100,
    _110,
    _120,
    _130,
    _140,
    _150,
    _300,
    _310,
    _320,
    _330,
    _400,
    _410,
    _420,
    _430,
    _440,
    _450,
    #[doc(hidden)]
    __Unknown(i32),
}

impl GLSLVersion {
    #[doc(alias = "gst_glsl_version_from_string")]
    pub fn from_string(string: &str) -> GLSLVersion {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_glsl_version_from_string(string.to_glib_none().0)) }
    }

    #[doc(alias = "gst_glsl_version_profile_from_string")]
    pub fn profile_from_string(string: &str) -> Option<(GLSLVersion, GLSLProfile)> {
        assert_initialized_main_thread!();
        unsafe {
            let mut version_ret = mem::MaybeUninit::uninit();
            let mut profile_ret = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_glsl_version_profile_from_string(
                string.to_glib_none().0,
                version_ret.as_mut_ptr(),
                profile_ret.as_mut_ptr(),
            ));
            let version_ret = version_ret.assume_init();
            let profile_ret = profile_ret.assume_init();
            if ret {
                Some((from_glib(version_ret), from_glib(profile_ret)))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_glsl_version_profile_to_string")]
    pub fn profile_to_string(self, profile: GLSLProfile) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_glsl_version_profile_to_string(
                self.to_glib(),
                profile.to_glib(),
            ))
        }
    }

    #[doc(alias = "gst_glsl_version_to_string")]
    pub fn to_str(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_glsl_version_to_string(self.to_glib())) }
    }
}

#[doc(hidden)]
impl ToGlib for GLSLVersion {
    type GlibType = ffi::GstGLSLVersion;

    fn to_glib(&self) -> ffi::GstGLSLVersion {
        match *self {
            GLSLVersion::None => ffi::GST_GLSL_VERSION_NONE,
            GLSLVersion::_100 => ffi::GST_GLSL_VERSION_100,
            GLSLVersion::_110 => ffi::GST_GLSL_VERSION_110,
            GLSLVersion::_120 => ffi::GST_GLSL_VERSION_120,
            GLSLVersion::_130 => ffi::GST_GLSL_VERSION_130,
            GLSLVersion::_140 => ffi::GST_GLSL_VERSION_140,
            GLSLVersion::_150 => ffi::GST_GLSL_VERSION_150,
            GLSLVersion::_300 => ffi::GST_GLSL_VERSION_300,
            GLSLVersion::_310 => ffi::GST_GLSL_VERSION_310,
            GLSLVersion::_320 => ffi::GST_GLSL_VERSION_320,
            GLSLVersion::_330 => ffi::GST_GLSL_VERSION_330,
            GLSLVersion::_400 => ffi::GST_GLSL_VERSION_400,
            GLSLVersion::_410 => ffi::GST_GLSL_VERSION_410,
            GLSLVersion::_420 => ffi::GST_GLSL_VERSION_420,
            GLSLVersion::_430 => ffi::GST_GLSL_VERSION_430,
            GLSLVersion::_440 => ffi::GST_GLSL_VERSION_440,
            GLSLVersion::_450 => ffi::GST_GLSL_VERSION_450,
            GLSLVersion::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLSLVersion> for GLSLVersion {
    unsafe fn from_glib(value: ffi::GstGLSLVersion) -> Self {
        skip_assert_initialized!();
        match value {
            0 => GLSLVersion::None,
            100 => GLSLVersion::_100,
            110 => GLSLVersion::_110,
            120 => GLSLVersion::_120,
            130 => GLSLVersion::_130,
            140 => GLSLVersion::_140,
            150 => GLSLVersion::_150,
            300 => GLSLVersion::_300,
            310 => GLSLVersion::_310,
            320 => GLSLVersion::_320,
            330 => GLSLVersion::_330,
            400 => GLSLVersion::_400,
            410 => GLSLVersion::_410,
            420 => GLSLVersion::_420,
            430 => GLSLVersion::_430,
            440 => GLSLVersion::_440,
            450 => GLSLVersion::_450,
            value => GLSLVersion::__Unknown(value),
        }
    }
}

impl StaticType for GLSLVersion {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_glsl_version_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GLSLVersion {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GLSLVersion {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GLSLVersion {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GLStereoDownmix {
    GreenMagentaDubois,
    RedCyanDubois,
    AmberBlueDubois,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for GLStereoDownmix {
    type GlibType = ffi::GstGLStereoDownmix;

    fn to_glib(&self) -> ffi::GstGLStereoDownmix {
        match *self {
            GLStereoDownmix::GreenMagentaDubois => {
                ffi::GST_GL_STEREO_DOWNMIX_ANAGLYPH_GREEN_MAGENTA_DUBOIS
            }
            GLStereoDownmix::RedCyanDubois => ffi::GST_GL_STEREO_DOWNMIX_ANAGLYPH_RED_CYAN_DUBOIS,
            GLStereoDownmix::AmberBlueDubois => {
                ffi::GST_GL_STEREO_DOWNMIX_ANAGLYPH_AMBER_BLUE_DUBOIS
            }
            GLStereoDownmix::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLStereoDownmix> for GLStereoDownmix {
    unsafe fn from_glib(value: ffi::GstGLStereoDownmix) -> Self {
        skip_assert_initialized!();
        match value {
            0 => GLStereoDownmix::GreenMagentaDubois,
            1 => GLStereoDownmix::RedCyanDubois,
            2 => GLStereoDownmix::AmberBlueDubois,
            value => GLStereoDownmix::__Unknown(value),
        }
    }
}

impl StaticType for GLStereoDownmix {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_stereo_downmix_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GLStereoDownmix {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GLStereoDownmix {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GLStereoDownmix {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GLTextureTarget {
    None,
    _2d,
    Rectangle,
    ExternalOes,
    #[doc(hidden)]
    __Unknown(i32),
}

impl GLTextureTarget {
    #[doc(alias = "gst_gl_texture_target_from_gl")]
    pub fn from_gl(target: u32) -> GLTextureTarget {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_gl_texture_target_from_gl(target)) }
    }

    #[doc(alias = "gst_gl_texture_target_from_string")]
    pub fn from_string(str: &str) -> GLTextureTarget {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_gl_texture_target_from_string(str.to_glib_none().0)) }
    }

    #[doc(alias = "gst_gl_texture_target_to_buffer_pool_option")]
    pub fn to_buffer_pool_option(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_gl_texture_target_to_buffer_pool_option(
                self.to_glib(),
            ))
        }
    }

    #[doc(alias = "gst_gl_texture_target_to_gl")]
    pub fn to_gl(self) -> u32 {
        assert_initialized_main_thread!();
        unsafe { ffi::gst_gl_texture_target_to_gl(self.to_glib()) }
    }

    #[doc(alias = "gst_gl_texture_target_to_string")]
    pub fn to_str(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_gl_texture_target_to_string(self.to_glib())) }
    }
}

#[doc(hidden)]
impl ToGlib for GLTextureTarget {
    type GlibType = ffi::GstGLTextureTarget;

    fn to_glib(&self) -> ffi::GstGLTextureTarget {
        match *self {
            GLTextureTarget::None => ffi::GST_GL_TEXTURE_TARGET_NONE,
            GLTextureTarget::_2d => ffi::GST_GL_TEXTURE_TARGET_2D,
            GLTextureTarget::Rectangle => ffi::GST_GL_TEXTURE_TARGET_RECTANGLE,
            GLTextureTarget::ExternalOes => ffi::GST_GL_TEXTURE_TARGET_EXTERNAL_OES,
            GLTextureTarget::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLTextureTarget> for GLTextureTarget {
    unsafe fn from_glib(value: ffi::GstGLTextureTarget) -> Self {
        skip_assert_initialized!();
        match value {
            0 => GLTextureTarget::None,
            1 => GLTextureTarget::_2d,
            2 => GLTextureTarget::Rectangle,
            3 => GLTextureTarget::ExternalOes,
            value => GLTextureTarget::__Unknown(value),
        }
    }
}

impl StaticType for GLTextureTarget {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_texture_target_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GLTextureTarget {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GLTextureTarget {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GLTextureTarget {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GLUploadReturn {
    Done,
    Error,
    Unsupported,
    Reconfigure,
    UnsharedGlContext,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for GLUploadReturn {
    type GlibType = ffi::GstGLUploadReturn;

    fn to_glib(&self) -> ffi::GstGLUploadReturn {
        match *self {
            GLUploadReturn::Done => ffi::GST_GL_UPLOAD_DONE,
            GLUploadReturn::Error => ffi::GST_GL_UPLOAD_ERROR,
            GLUploadReturn::Unsupported => ffi::GST_GL_UPLOAD_UNSUPPORTED,
            GLUploadReturn::Reconfigure => ffi::GST_GL_UPLOAD_RECONFIGURE,
            GLUploadReturn::UnsharedGlContext => ffi::GST_GL_UPLOAD_UNSHARED_GL_CONTEXT,
            GLUploadReturn::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLUploadReturn> for GLUploadReturn {
    unsafe fn from_glib(value: ffi::GstGLUploadReturn) -> Self {
        skip_assert_initialized!();
        match value {
            1 => GLUploadReturn::Done,
            -1 => GLUploadReturn::Error,
            -2 => GLUploadReturn::Unsupported,
            -3 => GLUploadReturn::Reconfigure,
            -100 => GLUploadReturn::UnsharedGlContext,
            value => GLUploadReturn::__Unknown(value),
        }
    }
}

impl StaticType for GLUploadReturn {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_upload_return_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GLUploadReturn {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GLUploadReturn {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GLUploadReturn {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GLWindowError {
    Failed,
    OldLibs,
    ResourceUnavailable,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for GLWindowError {
    type GlibType = ffi::GstGLWindowError;

    fn to_glib(&self) -> ffi::GstGLWindowError {
        match *self {
            GLWindowError::Failed => ffi::GST_GL_WINDOW_ERROR_FAILED,
            GLWindowError::OldLibs => ffi::GST_GL_WINDOW_ERROR_OLD_LIBS,
            GLWindowError::ResourceUnavailable => ffi::GST_GL_WINDOW_ERROR_RESOURCE_UNAVAILABLE,
            GLWindowError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLWindowError> for GLWindowError {
    unsafe fn from_glib(value: ffi::GstGLWindowError) -> Self {
        skip_assert_initialized!();
        match value {
            0 => GLWindowError::Failed,
            1 => GLWindowError::OldLibs,
            2 => GLWindowError::ResourceUnavailable,
            value => GLWindowError::__Unknown(value),
        }
    }
}

impl ErrorDomain for GLWindowError {
    fn domain() -> Quark {
        skip_assert_initialized!();

        unsafe { from_glib(ffi::gst_gl_window_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            0 => Some(GLWindowError::Failed),
            1 => Some(GLWindowError::OldLibs),
            2 => Some(GLWindowError::ResourceUnavailable),
            _ => Some(GLWindowError::Failed),
        }
    }
}

impl StaticType for GLWindowError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_window_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GLWindowError {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GLWindowError {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GLWindowError {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
