//! Some wrappers around the generated code to simplify use.

use std::convert::TryInto;
use std::marker::PhantomData;

use super::connection::Connection;
use super::cookie::VoidCookie;
use super::errors::{ConnectionError, ReplyError, ReplyOrIdError};
use super::protocol::xproto::{self, Atom, ConnectionExt as XProtoConnectionExt, PropMode, Window};
use super::x11_utils::TryParse;

/// Iterator implementation used by `GetPropertyReply`.
///
/// This is the actual type returned by `GetPropertyReply::value8`, `GetPropertyReply::value16`,
/// and `GetPropertyReply::value32`. This type needs to be public due to Rust's visibility rules.
#[derive(Debug, Clone)]
pub struct PropertyIterator<'a, T>(&'a [u8], PhantomData<T>);

impl<'a, T> PropertyIterator<'a, T> {
    pub(crate) fn new(value: &'a [u8]) -> Self {
        PropertyIterator(value, PhantomData)
    }
}

impl<T> Iterator for PropertyIterator<'_, T>
where
    T: TryParse,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match T::try_parse(self.0) {
            Ok((value, remaining)) => {
                self.0 = remaining;
                Some(value)
            }
            Err(_) => {
                self.0 = &[];
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.0.len() / std::mem::size_of::<T>();
        (size, Some(size))
    }
}

impl<T: TryParse> std::iter::FusedIterator for PropertyIterator<'_, T> {}

/// Extension trait that simplifies API use
pub trait ConnectionExt: XProtoConnectionExt {
    /// Change a property on a window with format 8.
    fn change_property8<A, B>(
        &self,
        mode: PropMode,
        window: Window,
        property: A,
        type_: B,
        data: &[u8],
    ) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<Atom>,
        B: Into<Atom>,
    {
        self.change_property(
            mode,
            window,
            property,
            type_,
            8,
            data.len().try_into().expect("`data` has too many elements"),
            data,
        )
    }

    /// Change a property on a window with format 16.
    fn change_property16<A, B>(
        &self,
        mode: PropMode,
        window: Window,
        property: A,
        type_: B,
        data: &[u16],
    ) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<Atom>,
        B: Into<Atom>,
    {
        let mut data_u8 = Vec::with_capacity(data.len() * 2);
        for item in data {
            data_u8.extend(&item.to_ne_bytes());
        }
        self.change_property(
            mode,
            window,
            property,
            type_,
            16,
            data.len().try_into().expect("`data` has too many elements"),
            &data_u8,
        )
    }

    /// Change a property on a window with format 32.
    fn change_property32<A, B>(
        &self,
        mode: PropMode,
        window: Window,
        property: A,
        type_: B,
        data: &[u32],
    ) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<Atom>,
        B: Into<Atom>,
    {
        let mut data_u8 = Vec::with_capacity(data.len() * 4);
        for item in data {
            data_u8.extend(&item.to_ne_bytes());
        }
        self.change_property(
            mode,
            window,
            property,
            type_,
            32,
            data.len().try_into().expect("`data` has too many elements"),
            &data_u8,
        )
    }

    /// Synchronise with the X11 server.
    ///
    /// This function synchronises with the X11 server. This means that all requests that are still
    /// in the output buffer are sent to the server. Then, we wait until the X11 server processed
    /// all requests.
    fn sync(&self) -> Result<(), ReplyError> {
        // When a new request is generated, it is appended to the output buffer. Thus, this causes
        // all previous requests to be sent.
        // The X11 server is single-threaded and processes requests in-order. Thus, it will only
        // reply to our GetInputFocus after everything before was processed.
        self.get_input_focus()?.reply().and(Ok(()))
    }
}
impl<C: XProtoConnectionExt + ?Sized> ConnectionExt for C {}

macro_rules! resource_wrapper {
    {
        $(#[$meta:meta])*
        pub struct $name:ident: $inner:ty,
        wrap: $wrapper:ident,
        get: $getter:ident,
        consume: $consumer:ident,
        free: $freer:ident,
    } => {
        $(#[$meta])*
        ///
        /// Any errors during `Drop` are silently ignored. Most likely an error here means that your
        /// X11 connection is broken and later requests will also fail.
        #[derive(Debug)]
        pub struct $name<'c, C: Connection>(&'c C, $inner);

        impl<'c, C: Connection> $name<'c, C>
        {
            /// Assume ownership of the given resource and destroy it in `Drop`.
            pub fn $wrapper(conn: &'c C, id: $inner) -> Self {
                $name(conn, id)
            }

            /// Get the XID of the wrapped resource
            pub fn $getter(&self) -> $inner {
                self.1
            }

            /// Assume ownership of the XID of the wrapped resource
            ///
            /// This function destroys this wrapper without freeing the underlying resource.
            pub fn $consumer(self) -> $inner {
                let id = self.1;
                std::mem::forget(self);
                id
            }
        }

        impl<C: Connection> From<&$name<'_, C>> for $inner {
            fn from(from: &$name<'_, C>) -> Self {
                from.1
            }
        }
        impl<'c, C: Connection> Drop for $name<'c, C> {
            fn drop(&mut self) {
                let _ = (self.0).$freer(self.1);
            }
        }
    }
}

resource_wrapper! {
    /// A RAII-like wrapper around a [xproto::Gcontext].
    ///
    /// Instances of this struct represent a graphics context that is freed in `Drop`.
    pub struct GcontextWrapper: xproto::Gcontext,
    wrap: for_gc,
    get: gc,
    consume: into_gc,
    free: free_gc,
}

impl<'c, C: Connection> GcontextWrapper<'c, C> {
    /// Create a new graphics context and return a graphics context wrapper and a cookie.
    ///
    /// This is a thin wrapper around [xproto::create_gc] that allocates a id for the graphics
    /// context. This function returns the resulting `GcontextWrapper` that owns the created
    /// graphics context and frees it in `Drop`. This also returns a `VoidCookie` that comes from
    /// the call to [xproto::create_gc].
    ///
    /// Errors can come from the call to [Connection::generate_id] or [xproto::create_gc].
    pub fn create_gc_and_get_cookie<'input>(
        conn: &'c C,
        drawable: xproto::Drawable,
        value_list: &'input xproto::CreateGCAux,
    ) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError> {
        let id = conn.generate_id()?;
        let cookie = conn.create_gc(id, drawable, value_list)?;
        Ok((Self::for_gc(conn, id), cookie))
    }

    /// Create a new graphics context and return a graphics context wrapper
    ///
    /// This is a thin wrapper around [xproto::create_gc] that allocates a id for the graphics
    /// context. This function returns the resulting `GcontextWrapper` that owns the created
    /// graphics context and frees it in `Drop`.
    ///
    /// Errors can come from the call to [Connection::generate_id] or [xproto::create_gc].
    pub fn create_gc<'input>(
        conn: &'c C,
        drawable: xproto::Drawable,
        value_list: &'input xproto::CreateGCAux,
    ) -> Result<Self, ReplyOrIdError> {
        Ok(Self::create_gc_and_get_cookie(conn, drawable, value_list)?.0)
    }
}

resource_wrapper! {
    /// A RAII-like wrapper around a [xproto::Colormap].
    ///
    /// Instances of this struct represent a colormap that is freed in `Drop`.
    pub struct ColormapWrapper: xproto::Colormap,
    wrap: for_colormap,
    get: colormap,
    consume: into_colormap,
    free: free_colormap,
}

impl<'c, C: Connection> ColormapWrapper<'c, C> {
    /// Create a new colormap and return a colormap wrapper and a cookie.
    ///
    /// This is a thin wrapper around [xproto::create_colormap] that allocates a id for the
    /// colormap. This function returns the resulting `ColormapWrapper` that owns the created
    /// colormap and frees it in `Drop`. This also returns a `VoidCookie` that comes from the call
    /// to [xproto::create_colormap].
    ///
    /// Errors can come from the call to [Connection::generate_id] or [xproto::create_colormap].
    pub fn create_colormap_and_get_cookie(
        conn: &'c C,
        alloc: xproto::ColormapAlloc,
        window: Window,
        visual: xproto::Visualid,
    ) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError> {
        let id = conn.generate_id()?;
        let cookie = conn.create_colormap(alloc, id, window, visual)?;
        Ok((Self::for_colormap(conn, id), cookie))
    }

    /// Create a new colormap and return a colormap wrapper
    ///
    /// This is a thin wrapper around [xproto::create_colormap] that allocates a id for the
    /// colormap. This function returns the resulting `ColormapWrapper` that owns the created
    /// colormap and frees it in `Drop`.
    ///
    /// Errors can come from the call to [Connection::generate_id] or [xproto::create_colormap].
    pub fn create_colormap(
        conn: &'c C,
        alloc: xproto::ColormapAlloc,
        window: Window,
        visual: xproto::Visualid,
    ) -> Result<Self, ReplyOrIdError> {
        Ok(Self::create_colormap_and_get_cookie(conn, alloc, window, visual)?.0)
    }
}

resource_wrapper! {
    /// A RAII-like wrapper around a [xproto::Cursor].
    ///
    /// Instances of this struct represent a graphics context that is freed in `Drop`.
    pub struct CursorWrapper: xproto::Cursor,
    wrap: for_cursor,
    get: cursor,
    consume: into_cursor,
    free: free_cursor,
}

impl<'c, C: Connection> CursorWrapper<'c, C> {
    /// Create a new cursor and return a cursor wrapper and a cookie.
    ///
    /// This is a thin wrapper around [xproto::create_cursor] that allocates a id for the cursor.
    /// This function returns the resulting `CursorWrapper` that owns the created cursor and frees
    /// it in `Drop`. This also returns a `VoidCookie` that comes from the call to
    /// [xproto::create_cursor].
    ///
    /// Errors can come from the call to [Connection::generate_id] or [xproto::create_cursor].
    #[allow(clippy::too_many_arguments)]
    pub fn create_cursor_and_get_cookie<A: Into<xproto::Pixmap>>(
        conn: &'c C,
        source: xproto::Pixmap,
        mask: A,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        x: u16,
        y: u16,
    ) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError> {
        let id = conn.generate_id()?;
        let cookie = conn.create_cursor(
            id, source, mask, fore_red, fore_green, fore_blue, back_red, back_green, back_blue, x,
            y,
        )?;
        Ok((Self::for_cursor(conn, id), cookie))
    }

    /// Create a new cursor and return a cursor wrapper
    ///
    /// This is a thin wrapper around [xproto::create_cursor] that allocates a id for the cursor.
    /// This function returns the resulting `CursorWrapper` that owns the created cursor and frees
    /// it in `Drop`.
    ///
    /// Errors can come from the call to [Connection::generate_id] or [xproto::create_cursor].
    #[allow(clippy::too_many_arguments)]
    pub fn create_cursor<A: Into<xproto::Pixmap>>(
        conn: &'c C,
        source: xproto::Pixmap,
        mask: A,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
        x: u16,
        y: u16,
    ) -> Result<Self, ReplyOrIdError> {
        Ok(Self::create_cursor_and_get_cookie(
            conn, source, mask, fore_red, fore_green, fore_blue, back_red, back_green, back_blue,
            x, y,
        )?
        .0)
    }

    /// Create a new cursor and return a cursor wrapper and a cookie.
    ///
    /// This is a thin wrapper around [xproto::create_glyph_cursor] that allocates a id for the
    /// cursor. This function returns the resulting `CursorWrapper` that owns the created cursor
    /// and frees it in `Drop`. This also returns a `VoidCookie` that comes from the call to
    /// [xproto::create_glyph_cursor].
    ///
    /// Errors can come from the call to [Connection::generate_id] or [xproto::create_glyph_cursor].
    #[allow(clippy::too_many_arguments)]
    pub fn create_glyph_cursor_and_get_cookie<A: Into<xproto::Font>>(
        conn: &'c C,
        source_font: xproto::Font,
        mask_font: A,
        source_char: u16,
        mask_char: u16,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> Result<(Self, VoidCookie<'c, C>), ReplyOrIdError> {
        let id = conn.generate_id()?;
        let cookie = conn.create_glyph_cursor(
            id,
            source_font,
            mask_font,
            source_char,
            mask_char,
            fore_red,
            fore_green,
            fore_blue,
            back_red,
            back_green,
            back_blue,
        )?;
        Ok((Self::for_cursor(conn, id), cookie))
    }

    /// Create a new cursor and return a cursor wrapper
    ///
    /// This is a thin wrapper around [xproto::create_glyph_cursor] that allocates a id for the
    /// cursor. This function returns the resulting `CursorWrapper` that owns the created cursor
    /// and frees it in `Drop`.
    ///
    /// Errors can come from the call to [Connection::generate_id] or [xproto::create_glyph_cursor].
    #[allow(clippy::too_many_arguments)]
    pub fn create_glyph_cursor<A: Into<xproto::Font>>(
        conn: &'c C,
        source_font: xproto::Font,
        mask_font: A,
        source_char: u16,
        mask_char: u16,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> Result<Self, ReplyOrIdError> {
        Ok(Self::create_glyph_cursor_and_get_cookie(
            conn,
            source_font,
            mask_font,
            source_char,
            mask_char,
            fore_red,
            fore_green,
            fore_blue,
            back_red,
            back_green,
            back_blue,
        )?
        .0)
    }
}
