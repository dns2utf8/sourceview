// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_14", feature = "dox"))]
use gio;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v3_18", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v3_18", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v3_18", feature = "dox"))]
use gobject_sys;
use gtk_source_sys;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use CompressionType;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Encoding;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use NewlineType;

glib_wrapper! {
    pub struct File(Object<gtk_source_sys::GtkSourceFile, gtk_source_sys::GtkSourceFileClass, FileClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_file_get_type(),
    }
}

impl File {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new() -> File {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_source_sys::gtk_source_file_new()) }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_FILE: Option<&File> = None;

pub trait FileExt: 'static {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn check_file_on_disk(&self);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_compression_type(&self) -> CompressionType;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_encoding(&self) -> Option<Encoding>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_location(&self) -> Option<gio::File>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_newline_type(&self) -> NewlineType;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_deleted(&self) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_externally_modified(&self) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_local(&self) -> bool;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_readonly(&self) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_location<P: IsA<gio::File>>(&self, location: Option<&P>);

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn set_mount_operation_factory(&self, callback: /*Unimplemented*/Fn(&File, /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Ignored*/gio::MountOperation, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_property_read_only(&self) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<File>> FileExt for O {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn check_file_on_disk(&self) {
        unsafe {
            gtk_source_sys::gtk_source_file_check_file_on_disk(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_get_compression_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_get_encoding(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_file_get_location(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_get_newline_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_deleted(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_is_deleted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_externally_modified(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_is_externally_modified(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_local(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_is_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn is_readonly(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_file_is_readonly(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_location<P: IsA<gio::File>>(&self, location: Option<&P>) {
        unsafe {
            gtk_source_sys::gtk_source_file_set_location(
                self.as_ref().to_glib_none().0,
                location.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    //#[cfg(any(feature = "v3_14", feature = "dox"))]
    //fn set_mount_operation_factory(&self, callback: /*Unimplemented*/Fn(&File, /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Ignored*/gio::MountOperation, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call gtk_source_sys:gtk_source_file_set_mount_operation_factory() }
    //}

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_property_read_only(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"read-only\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_compression_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceFile,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<File>,
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::compression-type\0".as_ptr() as *const _,
                Some(transmute(
                    notify_compression_type_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_encoding_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceFile,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<File>,
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::encoding\0".as_ptr() as *const _,
                Some(transmute(notify_encoding_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_location_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceFile,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<File>,
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::location\0".as_ptr() as *const _,
                Some(transmute(notify_location_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_newline_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceFile,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<File>,
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::newline-type\0".as_ptr() as *const _,
                Some(transmute(
                    notify_newline_type_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_read_only_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceFile,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<File>,
        {
            let f: &F = &*(f as *const F);
            f(&File::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::read-only\0".as_ptr() as *const _,
                Some(transmute(notify_read_only_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File")
    }
}
