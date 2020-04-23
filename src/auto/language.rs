// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
	pub struct Language(Object<gtk_source_sys::GtkSourceLanguage, gtk_source_sys::GtkSourceLanguageClass, LanguageClass>);

	match fn {
		get_type => || gtk_source_sys::gtk_source_language_get_type(),
	}
}

pub const NONE_LANGUAGE: Option<&Language> = None;

pub trait LanguageExt: 'static
{
	fn get_globs(&self) -> Vec<GString>;

	fn get_hidden(&self) -> bool;

	fn get_id(&self) -> Option<GString>;

	fn get_metadata(&self, name: &str) -> Option<GString>;

	fn get_mime_types(&self) -> Vec<GString>;

	fn get_name(&self) -> Option<GString>;

	fn get_section(&self) -> Option<GString>;

	#[cfg(any(feature = "v3_4", feature = "dox"))]
	fn get_style_fallback(&self, style_id: &str) -> Option<GString>;

	fn get_style_ids(&self) -> Vec<GString>;

	fn get_style_name(&self, style_id: &str) -> Option<GString>;

	fn connect_property_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

	fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

	fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

	fn connect_property_section_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Language>> LanguageExt for O
{
	fn get_globs(&self) -> Vec<GString>
	{
		unsafe {
			FromGlibPtrContainer::from_glib_full(gtk_source_sys::gtk_source_language_get_globs(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn get_hidden(&self) -> bool
	{
		unsafe {
			from_glib(gtk_source_sys::gtk_source_language_get_hidden(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn get_id(&self) -> Option<GString>
	{
		unsafe {
			from_glib_none(gtk_source_sys::gtk_source_language_get_id(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn get_metadata(&self, name: &str) -> Option<GString>
	{
		unsafe {
			from_glib_none(gtk_source_sys::gtk_source_language_get_metadata(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
			))
		}
	}

	fn get_mime_types(&self) -> Vec<GString>
	{
		unsafe {
			FromGlibPtrContainer::from_glib_full(
				gtk_source_sys::gtk_source_language_get_mime_types(self.as_ref().to_glib_none().0),
			)
		}
	}

	fn get_name(&self) -> Option<GString>
	{
		unsafe {
			from_glib_none(gtk_source_sys::gtk_source_language_get_name(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn get_section(&self) -> Option<GString>
	{
		unsafe {
			from_glib_none(gtk_source_sys::gtk_source_language_get_section(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	#[cfg(any(feature = "v3_4", feature = "dox"))]
	fn get_style_fallback(&self, style_id: &str) -> Option<GString>
	{
		unsafe {
			from_glib_none(gtk_source_sys::gtk_source_language_get_style_fallback(
				self.as_ref().to_glib_none().0,
				style_id.to_glib_none().0,
			))
		}
	}

	fn get_style_ids(&self) -> Vec<GString>
	{
		unsafe {
			FromGlibPtrContainer::from_glib_full(gtk_source_sys::gtk_source_language_get_style_ids(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn get_style_name(&self, style_id: &str) -> Option<GString>
	{
		unsafe {
			from_glib_none(gtk_source_sys::gtk_source_language_get_style_name(
				self.as_ref().to_glib_none().0,
				style_id.to_glib_none().0,
			))
		}
	}

	fn connect_property_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId
	{
		unsafe extern "C" fn notify_hidden_trampoline<P, F: Fn(&P) + 'static>(
			this: *mut gtk_source_sys::GtkSourceLanguage,
			_param_spec: glib_sys::gpointer,
			f: glib_sys::gpointer,
		) where
			P: IsA<Language>,
		{
			let f: &F = &*(f as *const F);
			f(&Language::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::hidden\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_hidden_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId
	{
		unsafe extern "C" fn notify_id_trampoline<P, F: Fn(&P) + 'static>(
			this: *mut gtk_source_sys::GtkSourceLanguage,
			_param_spec: glib_sys::gpointer,
			f: glib_sys::gpointer,
		) where
			P: IsA<Language>,
		{
			let f: &F = &*(f as *const F);
			f(&Language::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::id\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_id_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId
	{
		unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
			this: *mut gtk_source_sys::GtkSourceLanguage,
			_param_spec: glib_sys::gpointer,
			f: glib_sys::gpointer,
		) where
			P: IsA<Language>,
		{
			let f: &F = &*(f as *const F);
			f(&Language::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::name\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_name_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_property_section_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId
	{
		unsafe extern "C" fn notify_section_trampoline<P, F: Fn(&P) + 'static>(
			this: *mut gtk_source_sys::GtkSourceLanguage,
			_param_spec: glib_sys::gpointer,
			f: glib_sys::gpointer,
		) where
			P: IsA<Language>,
		{
			let f: &F = &*(f as *const F);
			f(&Language::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::section\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_section_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl fmt::Display for Language
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "Language")
	}
}
