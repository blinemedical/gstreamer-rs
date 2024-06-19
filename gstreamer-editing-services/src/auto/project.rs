// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
use crate::Formatter;
use crate::{Asset, MetaContainer, Timeline};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GESProject")]
    pub struct Project(Object<ffi::GESProject, ffi::GESProjectClass>) @extends Asset, @implements MetaContainer;

    match fn {
        type_ => || ffi::ges_project_get_type(),
    }
}

impl Project {
    pub const NONE: Option<&'static Project> = None;

    #[doc(alias = "ges_project_new")]
    pub fn new(uri: Option<&str>) -> Project {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::ges_project_new(uri.to_glib_none().0)) }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Project>> Sealed for T {}
}

pub trait ProjectExt: IsA<Project> + sealed::Sealed + 'static {
    #[doc(alias = "ges_project_add_asset")]
    fn add_asset(&self, asset: &impl IsA<Asset>) -> bool {
        unsafe {
            from_glib(ffi::ges_project_add_asset(
                self.as_ref().to_glib_none().0,
                asset.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_project_add_encoding_profile")]
    fn add_encoding_profile(
        &self,
        profile: &impl IsA<gst_pbutils::EncodingProfile>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_project_add_encoding_profile(
                    self.as_ref().to_glib_none().0,
                    profile.as_ref().to_glib_none().0
                ),
                "Failed to add profile"
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_project_add_formatter")]
    fn add_formatter(&self, formatter: &impl IsA<Formatter>) {
        unsafe {
            ffi::ges_project_add_formatter(
                self.as_ref().to_glib_none().0,
                formatter.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "ges_project_create_asset")]
    fn create_asset(&self, id: Option<&str>, extractable_type: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::ges_project_create_asset(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                extractable_type.into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_project_create_asset_sync")]
    fn create_asset_sync(
        &self,
        id: Option<&str>,
        extractable_type: glib::types::Type,
    ) -> Result<Option<Asset>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::ges_project_create_asset_sync(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                extractable_type.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "ges_project_get_asset")]
    #[doc(alias = "get_asset")]
    fn asset(&self, id: &str, extractable_type: glib::types::Type) -> Option<Asset> {
        unsafe {
            from_glib_full(ffi::ges_project_get_asset(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                extractable_type.into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_project_get_loading_assets")]
    #[doc(alias = "get_loading_assets")]
    fn loading_assets(&self) -> Vec<Asset> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_project_get_loading_assets(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_project_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::ges_project_get_uri(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "ges_project_list_assets")]
    fn list_assets(&self, filter: glib::types::Type) -> Vec<Asset> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_project_list_assets(
                self.as_ref().to_glib_none().0,
                filter.into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_project_list_encoding_profiles")]
    fn list_encoding_profiles(&self) -> Vec<gst_pbutils::EncodingProfile> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::ges_project_list_encoding_profiles(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_project_load")]
    fn load(&self, timeline: &impl IsA<Timeline>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_project_load(
                self.as_ref().to_glib_none().0,
                timeline.as_ref().to_glib_none().0,
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

    #[doc(alias = "ges_project_remove_asset")]
    fn remove_asset(&self, asset: &impl IsA<Asset>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_project_remove_asset(
                    self.as_ref().to_glib_none().0,
                    asset.as_ref().to_glib_none().0
                ),
                "Failed to remove asset"
            )
        }
    }

    #[doc(alias = "ges_project_save")]
    fn save(
        &self,
        timeline: &impl IsA<Timeline>,
        uri: &str,
        formatter_asset: Option<impl IsA<Asset>>,
        overwrite: bool,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_project_save(
                self.as_ref().to_glib_none().0,
                timeline.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                formatter_asset.map(|p| p.upcast()).into_glib_ptr(),
                overwrite.into_glib(),
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

    #[doc(alias = "asset-added")]
    fn connect_asset_added<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn asset_added_trampoline<
            P: IsA<Project>,
            F: Fn(&P, &Asset) + 'static,
        >(
            this: *mut ffi::GESProject,
            asset: *mut ffi::GESAsset,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Project::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(asset),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"asset-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    asset_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "asset-loading")]
    fn connect_asset_loading<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn asset_loading_trampoline<
            P: IsA<Project>,
            F: Fn(&P, &Asset) + 'static,
        >(
            this: *mut ffi::GESProject,
            asset: *mut ffi::GESAsset,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Project::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(asset),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"asset-loading\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    asset_loading_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "asset-removed")]
    fn connect_asset_removed<F: Fn(&Self, &Asset) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn asset_removed_trampoline<
            P: IsA<Project>,
            F: Fn(&P, &Asset) + 'static,
        >(
            this: *mut ffi::GESProject,
            asset: *mut ffi::GESAsset,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Project::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(asset),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"asset-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    asset_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "error-loading")]
    fn connect_error_loading<F: Fn(&Self, &Timeline, &glib::Error) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn error_loading_trampoline<
            P: IsA<Project>,
            F: Fn(&P, &Timeline, &glib::Error) + 'static,
        >(
            this: *mut ffi::GESProject,
            timeline: *mut ffi::GESTimeline,
            error: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Project::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(timeline),
                &from_glib_borrow(error),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"error-loading\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    error_loading_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "error-loading-asset")]
    fn connect_error_loading_asset<
        F: Fn(&Self, &glib::Error, &str, glib::types::Type) + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn error_loading_asset_trampoline<
            P: IsA<Project>,
            F: Fn(&P, &glib::Error, &str, glib::types::Type) + 'static,
        >(
            this: *mut ffi::GESProject,
            error: *mut glib::ffi::GError,
            id: *mut libc::c_char,
            extractable_type: glib::ffi::GType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Project::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(error),
                &glib::GString::from_glib_borrow(id),
                from_glib(extractable_type),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"error-loading-asset\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    error_loading_asset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "loaded")]
    fn connect_loaded<F: Fn(&Self, &Timeline) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn loaded_trampoline<P: IsA<Project>, F: Fn(&P, &Timeline) + 'static>(
            this: *mut ffi::GESProject,
            timeline: *mut ffi::GESTimeline,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Project::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(timeline),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"loaded\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    loaded_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "loading")]
    fn connect_loading<F: Fn(&Self, &Timeline) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn loading_trampoline<P: IsA<Project>, F: Fn(&P, &Timeline) + 'static>(
            this: *mut ffi::GESProject,
            timeline: *mut ffi::GESTimeline,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Project::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(timeline),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"loading\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    loading_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "missing-uri")]
    fn connect_missing_uri<
        F: Fn(&Self, &glib::Error, &Asset) -> Option<glib::GString> + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn missing_uri_trampoline<
            P: IsA<Project>,
            F: Fn(&P, &glib::Error, &Asset) -> Option<glib::GString> + 'static,
        >(
            this: *mut ffi::GESProject,
            error: *mut glib::ffi::GError,
            wrong_asset: *mut ffi::GESAsset,
            f: glib::ffi::gpointer,
        ) -> *mut libc::c_char {
            let f: &F = &*(f as *const F);
            f(
                Project::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(error),
                &from_glib_borrow(wrong_asset),
            )
            .to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"missing-uri\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    missing_uri_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Project>> ProjectExt for O {}
