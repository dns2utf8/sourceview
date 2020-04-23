// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_pixbuf;
#[cfg(any(feature = "v3_18", feature = "dox"))]
use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v3_18", feature = "dox"))]
use glib::value::SetValueOptional;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_source_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use CompletionProposal;

glib_wrapper! {
	pub struct CompletionItem(Object<gtk_source_sys::GtkSourceCompletionItem, gtk_source_sys::GtkSourceCompletionItemClass, CompletionItemClass>) @implements CompletionProposal;

	match fn {
		get_type => || gtk_source_sys::gtk_source_completion_item_get_type(),
	}
}

impl CompletionItem
{
	#[cfg_attr(feature = "v3_24", deprecated)]
	pub fn new(
		label: &str,
		text: &str,
		icon: Option<&gdk_pixbuf::Pixbuf>,
		info: Option<&str>,
	) -> CompletionItem
	{
		assert_initialized_main_thread!();
		unsafe {
			from_glib_full(gtk_source_sys::gtk_source_completion_item_new(
				label.to_glib_none().0,
				text.to_glib_none().0,
				icon.to_glib_none().0,
				info.to_glib_none().0,
			))
		}
	}

	#[cfg_attr(feature = "v3_10", deprecated)]
	pub fn new_from_stock(
		label: Option<&str>,
		text: &str,
		stock: &str,
		info: Option<&str>,
	) -> CompletionItem
	{
		assert_initialized_main_thread!();
		unsafe {
			from_glib_full(gtk_source_sys::gtk_source_completion_item_new_from_stock(
				label.to_glib_none().0,
				text.to_glib_none().0,
				stock.to_glib_none().0,
				info.to_glib_none().0,
			))
		}
	}

	#[cfg_attr(feature = "v3_24", deprecated)]
	pub fn new_with_markup(
		markup: &str,
		text: &str,
		icon: Option<&gdk_pixbuf::Pixbuf>,
		info: Option<&str>,
	) -> CompletionItem
	{
		assert_initialized_main_thread!();
		unsafe {
			from_glib_full(gtk_source_sys::gtk_source_completion_item_new_with_markup(
				markup.to_glib_none().0,
				text.to_glib_none().0,
				icon.to_glib_none().0,
				info.to_glib_none().0,
			))
		}
	}

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	pub fn new2() -> Option<CompletionItem>
	{
		assert_initialized_main_thread!();
		unsafe { from_glib_full(gtk_source_sys::gtk_source_completion_item_new2()) }
	}
}

#[derive(Clone, Default)]
pub struct CompletionItemBuilder
{
	#[cfg(any(feature = "v3_18", feature = "dox"))]
	gicon: Option<gio::Icon>,
	icon: Option<gdk_pixbuf::Pixbuf>,
	#[cfg(any(feature = "v3_18", feature = "dox"))]
	icon_name: Option<String>,
	info: Option<String>,
	label: Option<String>,
	markup: Option<String>,
	text: Option<String>,
}

impl CompletionItemBuilder
{
	pub fn new() -> Self
	{
		Self::default()
	}

	pub fn build(self) -> CompletionItem
	{
		let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
		#[cfg(any(feature = "v3_18", feature = "dox"))]
		{
			if let Some(ref gicon) = self.gicon
			{
				properties.push(("gicon", gicon));
			}
		}
		if let Some(ref icon) = self.icon
		{
			properties.push(("icon", icon));
		}
		#[cfg(any(feature = "v3_18", feature = "dox"))]
		{
			if let Some(ref icon_name) = self.icon_name
			{
				properties.push(("icon-name", icon_name));
			}
		}
		if let Some(ref info) = self.info
		{
			properties.push(("info", info));
		}
		if let Some(ref label) = self.label
		{
			properties.push(("label", label));
		}
		if let Some(ref markup) = self.markup
		{
			properties.push(("markup", markup));
		}
		if let Some(ref text) = self.text
		{
			properties.push(("text", text));
		}
		glib::Object::new(CompletionItem::static_type(), &properties)
			.expect("object new")
			.downcast()
			.expect("downcast")
	}

	#[cfg(any(feature = "v3_18", feature = "dox"))]
	pub fn gicon<P: IsA<gio::Icon>>(mut self, gicon: &P) -> Self
	{
		self.gicon = Some(gicon.clone().upcast());
		self
	}

	pub fn icon(mut self, icon: &gdk_pixbuf::Pixbuf) -> Self
	{
		self.icon = Some(icon.clone());
		self
	}

	#[cfg(any(feature = "v3_18", feature = "dox"))]
	pub fn icon_name(mut self, icon_name: &str) -> Self
	{
		self.icon_name = Some(icon_name.to_string());
		self
	}

	pub fn info(mut self, info: &str) -> Self
	{
		self.info = Some(info.to_string());
		self
	}

	pub fn label(mut self, label: &str) -> Self
	{
		self.label = Some(label.to_string());
		self
	}

	pub fn markup(mut self, markup: &str) -> Self
	{
		self.markup = Some(markup.to_string());
		self
	}

	pub fn text(mut self, text: &str) -> Self
	{
		self.text = Some(text.to_string());
		self
	}
}

pub const NONE_COMPLETION_ITEM: Option<&CompletionItem> = None;

pub trait CompletionItemExt: 'static
{
	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: Option<&P>);

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>);

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_icon_name(&self, icon_name: Option<&str>);

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_info(&self, info: Option<&str>);

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_label(&self, label: Option<&str>);

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_markup(&self, markup: Option<&str>);

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_text(&self, text: Option<&str>);

	#[cfg(any(feature = "v3_18", feature = "dox"))]
	fn set_property_gicon<P: IsA<gio::Icon> + SetValueOptional>(&self, gicon: Option<&P>);

	fn set_property_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>);

	#[cfg(any(feature = "v3_18", feature = "dox"))]
	fn set_property_icon_name(&self, icon_name: Option<&str>);

	fn set_property_info(&self, info: Option<&str>);

	fn set_property_label(&self, label: Option<&str>);

	fn set_property_markup(&self, markup: Option<&str>);

	fn set_property_text(&self, text: Option<&str>);

	#[cfg(any(feature = "v3_18", feature = "dox"))]
	fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

	fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

	#[cfg(any(feature = "v3_18", feature = "dox"))]
	fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

	fn connect_property_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

	fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

	fn connect_property_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

	fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CompletionItem>> CompletionItemExt for O
{
	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: Option<&P>)
	{
		unsafe {
			gtk_source_sys::gtk_source_completion_item_set_gicon(
				self.as_ref().to_glib_none().0,
				gicon.map(|p| p.as_ref()).to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>)
	{
		unsafe {
			gtk_source_sys::gtk_source_completion_item_set_icon(
				self.as_ref().to_glib_none().0,
				icon.to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_icon_name(&self, icon_name: Option<&str>)
	{
		unsafe {
			gtk_source_sys::gtk_source_completion_item_set_icon_name(
				self.as_ref().to_glib_none().0,
				icon_name.to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_info(&self, info: Option<&str>)
	{
		unsafe {
			gtk_source_sys::gtk_source_completion_item_set_info(
				self.as_ref().to_glib_none().0,
				info.to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_label(&self, label: Option<&str>)
	{
		unsafe {
			gtk_source_sys::gtk_source_completion_item_set_label(
				self.as_ref().to_glib_none().0,
				label.to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_markup(&self, markup: Option<&str>)
	{
		unsafe {
			gtk_source_sys::gtk_source_completion_item_set_markup(
				self.as_ref().to_glib_none().0,
				markup.to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v3_24", feature = "dox"))]
	fn set_text(&self, text: Option<&str>)
	{
		unsafe {
			gtk_source_sys::gtk_source_completion_item_set_text(
				self.as_ref().to_glib_none().0,
				text.to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v3_18", feature = "dox"))]
	fn set_property_gicon<P: IsA<gio::Icon> + SetValueOptional>(&self, gicon: Option<&P>)
	{
		unsafe {
			gobject_sys::g_object_set_property(
				self.to_glib_none().0 as *mut gobject_sys::GObject,
				b"gicon\0".as_ptr() as *const _,
				Value::from(gicon).to_glib_none().0,
			);
		}
	}

	fn set_property_icon(&self, icon: Option<&gdk_pixbuf::Pixbuf>)
	{
		unsafe {
			gobject_sys::g_object_set_property(
				self.to_glib_none().0 as *mut gobject_sys::GObject,
				b"icon\0".as_ptr() as *const _,
				Value::from(icon).to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v3_18", feature = "dox"))]
	fn set_property_icon_name(&self, icon_name: Option<&str>)
	{
		unsafe {
			gobject_sys::g_object_set_property(
				self.to_glib_none().0 as *mut gobject_sys::GObject,
				b"icon-name\0".as_ptr() as *const _,
				Value::from(icon_name).to_glib_none().0,
			);
		}
	}

	fn set_property_info(&self, info: Option<&str>)
	{
		unsafe {
			gobject_sys::g_object_set_property(
				self.to_glib_none().0 as *mut gobject_sys::GObject,
				b"info\0".as_ptr() as *const _,
				Value::from(info).to_glib_none().0,
			);
		}
	}

	fn set_property_label(&self, label: Option<&str>)
	{
		unsafe {
			gobject_sys::g_object_set_property(
				self.to_glib_none().0 as *mut gobject_sys::GObject,
				b"label\0".as_ptr() as *const _,
				Value::from(label).to_glib_none().0,
			);
		}
	}

	fn set_property_markup(&self, markup: Option<&str>)
	{
		unsafe {
			gobject_sys::g_object_set_property(
				self.to_glib_none().0 as *mut gobject_sys::GObject,
				b"markup\0".as_ptr() as *const _,
				Value::from(markup).to_glib_none().0,
			);
		}
	}

	fn set_property_text(&self, text: Option<&str>)
	{
		unsafe {
			gobject_sys::g_object_set_property(
				self.to_glib_none().0 as *mut gobject_sys::GObject,
				b"text\0".as_ptr() as *const _,
				Value::from(text).to_glib_none().0,
			);
		}
	}

	#[cfg(any(feature = "v3_18", feature = "dox"))]
	fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId
	{
		unsafe extern "C" fn notify_gicon_trampoline<P, F: Fn(&P) + 'static>(
			this: *mut gtk_source_sys::GtkSourceCompletionItem,
			_param_spec: glib_sys::gpointer,
			f: glib_sys::gpointer,
		) where
			P: IsA<CompletionItem>,
		{
			let f: &F = &*(f as *const F);
			f(&CompletionItem::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::gicon\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_gicon_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_property_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId
	{
		unsafe extern "C" fn notify_icon_trampoline<P, F: Fn(&P) + 'static>(
			this: *mut gtk_source_sys::GtkSourceCompletionItem,
			_param_spec: glib_sys::gpointer,
			f: glib_sys::gpointer,
		) where
			P: IsA<CompletionItem>,
		{
			let f: &F = &*(f as *const F);
			f(&CompletionItem::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::icon\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_icon_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	#[cfg(any(feature = "v3_18", feature = "dox"))]
	fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId
	{
		unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
			this: *mut gtk_source_sys::GtkSourceCompletionItem,
			_param_spec: glib_sys::gpointer,
			f: glib_sys::gpointer,
		) where
			P: IsA<CompletionItem>,
		{
			let f: &F = &*(f as *const F);
			f(&CompletionItem::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::icon-name\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_icon_name_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_property_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId
	{
		unsafe extern "C" fn notify_info_trampoline<P, F: Fn(&P) + 'static>(
			this: *mut gtk_source_sys::GtkSourceCompletionItem,
			_param_spec: glib_sys::gpointer,
			f: glib_sys::gpointer,
		) where
			P: IsA<CompletionItem>,
		{
			let f: &F = &*(f as *const F);
			f(&CompletionItem::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::info\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_info_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId
	{
		unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(
			this: *mut gtk_source_sys::GtkSourceCompletionItem,
			_param_spec: glib_sys::gpointer,
			f: glib_sys::gpointer,
		) where
			P: IsA<CompletionItem>,
		{
			let f: &F = &*(f as *const F);
			f(&CompletionItem::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::label\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_label_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_property_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId
	{
		unsafe extern "C" fn notify_markup_trampoline<P, F: Fn(&P) + 'static>(
			this: *mut gtk_source_sys::GtkSourceCompletionItem,
			_param_spec: glib_sys::gpointer,
			f: glib_sys::gpointer,
		) where
			P: IsA<CompletionItem>,
		{
			let f: &F = &*(f as *const F);
			f(&CompletionItem::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::markup\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_markup_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}

	fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId
	{
		unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
			this: *mut gtk_source_sys::GtkSourceCompletionItem,
			_param_spec: glib_sys::gpointer,
			f: glib_sys::gpointer,
		) where
			P: IsA<CompletionItem>,
		{
			let f: &F = &*(f as *const F);
			f(&CompletionItem::from_glib_borrow(this).unsafe_cast_ref())
		}
		unsafe {
			let f: Box_<F> = Box_::new(f);
			connect_raw(
				self.as_ptr() as *mut _,
				b"notify::text\0".as_ptr() as *const _,
				Some(transmute::<_, unsafe extern "C" fn()>(
					notify_text_trampoline::<Self, F> as *const (),
				)),
				Box_::into_raw(f),
			)
		}
	}
}

impl fmt::Display for CompletionItem
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "CompletionItem")
	}
}
