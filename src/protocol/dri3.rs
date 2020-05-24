// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `DRI3` X11 extension.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::eq_op)]

use std::borrow::Cow;
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::{Serialize, TryParse};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ConnectionError, ParseError};
use super::xproto;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "DRI3";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest {
    pub major_version: u32,
    pub minor_version: u32,
}
impl QueryVersionRequest {
    /// Opcode for the QueryVersion request
    pub const fn opcode() -> u8 { 0 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let major_version_bytes = self.major_version.serialize();
        let minor_version_bytes = self.minor_version.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            major_version_bytes[0],
            major_version_bytes[1],
            major_version_bytes[2],
            major_version_bytes[3],
            minor_version_bytes[0],
            minor_version_bytes[1],
            minor_version_bytes[2],
            minor_version_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn query_version<Conn>(conn: &Conn, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        major_version,
        minor_version,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u32,
    pub minor_version: u32,
}
impl TryParse for QueryVersionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u32::try_parse(remaining)?;
        let (minor_version, remaining) = u32::try_parse(remaining)?;
        let result = QueryVersionReply { response_type, sequence, length, major_version, minor_version };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenRequest {
    pub drawable: xproto::Drawable,
    pub provider: u32,
}
impl OpenRequest {
    /// Opcode for the Open request
    pub const fn opcode() -> u8 { 1 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let provider_bytes = self.provider.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            provider_bytes[0],
            provider_bytes[1],
            provider_bytes[2],
            provider_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn open<Conn>(conn: &Conn, drawable: xproto::Drawable, provider: u32) -> Result<CookieWithFds<'_, Conn, OpenReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = OpenRequest {
        drawable,
        provider,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply_with_fds(&slices, fds)?)
}

#[derive(Debug, PartialEq, Eq)]
pub struct OpenReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub device_fd: RawFdContainer,
}
impl OpenReply {
    fn try_parse_fd<'a>(remaining: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::ParseError) }
        let device_fd = fds.remove(0);
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let result = OpenReply { response_type, nfd, sequence, length, device_fd };
        Ok((result, remaining))
    }
}
impl TryFrom<(&[u8], Vec<RawFdContainer>)> for OpenReply {
    type Error = ParseError;
    fn try_from(value: (&[u8], Vec<RawFdContainer>)) -> Result<Self, Self::Error> {
        let (value, mut fds) = value;
        Ok(Self::try_parse_fd(value, &mut fds)?.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PixmapFromBufferRequest {
    pub pixmap: xproto::Pixmap,
    pub drawable: xproto::Drawable,
    pub size: u32,
    pub width: u16,
    pub height: u16,
    pub stride: u16,
    pub depth: u8,
    pub bpp: u8,
    pub pixmap_fd: RawFdContainer,
}
impl PixmapFromBufferRequest {
    /// Opcode for the PixmapFromBuffer request
    pub const fn opcode() -> u8 { 2 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let pixmap_bytes = self.pixmap.serialize();
        let drawable_bytes = self.drawable.serialize();
        let size_bytes = self.size.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let stride_bytes = self.stride.serialize();
        let depth_bytes = self.depth.serialize();
        let bpp_bytes = self.bpp.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            pixmap_bytes[0],
            pixmap_bytes[1],
            pixmap_bytes[2],
            pixmap_bytes[3],
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            stride_bytes[0],
            stride_bytes[1],
            depth_bytes[0],
            bpp_bytes[0],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![self.pixmap_fd]))
    }
}
pub fn pixmap_from_buffer<Conn, A>(conn: &Conn, pixmap: xproto::Pixmap, drawable: xproto::Drawable, size: u32, width: u16, height: u16, stride: u16, depth: u8, bpp: u8, pixmap_fd: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<RawFdContainer>,
{
    let pixmap_fd: RawFdContainer = pixmap_fd.into();
    let request0 = PixmapFromBufferRequest {
        pixmap,
        drawable,
        size,
        width,
        height,
        stride,
        depth,
        bpp,
        pixmap_fd,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferFromPixmapRequest {
    pub pixmap: xproto::Pixmap,
}
impl BufferFromPixmapRequest {
    /// Opcode for the BufferFromPixmap request
    pub const fn opcode() -> u8 { 3 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let pixmap_bytes = self.pixmap.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            pixmap_bytes[0],
            pixmap_bytes[1],
            pixmap_bytes[2],
            pixmap_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn buffer_from_pixmap<Conn>(conn: &Conn, pixmap: xproto::Pixmap) -> Result<CookieWithFds<'_, Conn, BufferFromPixmapReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = BufferFromPixmapRequest {
        pixmap,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply_with_fds(&slices, fds)?)
}

#[derive(Debug, PartialEq, Eq)]
pub struct BufferFromPixmapReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u32,
    pub width: u16,
    pub height: u16,
    pub stride: u16,
    pub depth: u8,
    pub bpp: u8,
    pub pixmap_fd: RawFdContainer,
}
impl BufferFromPixmapReply {
    fn try_parse_fd<'a>(remaining: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (size, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let (stride, remaining) = u16::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (bpp, remaining) = u8::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::ParseError) }
        let pixmap_fd = fds.remove(0);
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let result = BufferFromPixmapReply { response_type, nfd, sequence, length, size, width, height, stride, depth, bpp, pixmap_fd };
        Ok((result, remaining))
    }
}
impl TryFrom<(&[u8], Vec<RawFdContainer>)> for BufferFromPixmapReply {
    type Error = ParseError;
    fn try_from(value: (&[u8], Vec<RawFdContainer>)) -> Result<Self, Self::Error> {
        let (value, mut fds) = value;
        Ok(Self::try_parse_fd(value, &mut fds)?.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FenceFromFDRequest {
    pub drawable: xproto::Drawable,
    pub fence: u32,
    pub initially_triggered: bool,
    pub fence_fd: RawFdContainer,
}
impl FenceFromFDRequest {
    /// Opcode for the FenceFromFD request
    pub const fn opcode() -> u8 { 4 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let fence_bytes = self.fence.serialize();
        let initially_triggered_bytes = self.initially_triggered.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            fence_bytes[0],
            fence_bytes[1],
            fence_bytes[2],
            fence_bytes[3],
            initially_triggered_bytes[0],
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![self.fence_fd]))
    }
}
pub fn fence_from_fd<Conn, A>(conn: &Conn, drawable: xproto::Drawable, fence: u32, initially_triggered: bool, fence_fd: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<RawFdContainer>,
{
    let fence_fd: RawFdContainer = fence_fd.into();
    let request0 = FenceFromFDRequest {
        drawable,
        fence,
        initially_triggered,
        fence_fd,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FDFromFenceRequest {
    pub drawable: xproto::Drawable,
    pub fence: u32,
}
impl FDFromFenceRequest {
    /// Opcode for the FDFromFence request
    pub const fn opcode() -> u8 { 5 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let drawable_bytes = self.drawable.serialize();
        let fence_bytes = self.fence.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            drawable_bytes[0],
            drawable_bytes[1],
            drawable_bytes[2],
            drawable_bytes[3],
            fence_bytes[0],
            fence_bytes[1],
            fence_bytes[2],
            fence_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn fd_from_fence<Conn>(conn: &Conn, drawable: xproto::Drawable, fence: u32) -> Result<CookieWithFds<'_, Conn, FDFromFenceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = FDFromFenceRequest {
        drawable,
        fence,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply_with_fds(&slices, fds)?)
}

#[derive(Debug, PartialEq, Eq)]
pub struct FDFromFenceReply {
    pub response_type: u8,
    pub nfd: u8,
    pub sequence: u16,
    pub length: u32,
    pub fence_fd: RawFdContainer,
}
impl FDFromFenceReply {
    fn try_parse_fd<'a>(remaining: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        if fds.is_empty() { return Err(ParseError::ParseError) }
        let fence_fd = fds.remove(0);
        let remaining = remaining.get(24..).ok_or(ParseError::ParseError)?;
        let result = FDFromFenceReply { response_type, nfd, sequence, length, fence_fd };
        Ok((result, remaining))
    }
}
impl TryFrom<(&[u8], Vec<RawFdContainer>)> for FDFromFenceReply {
    type Error = ParseError;
    fn try_from(value: (&[u8], Vec<RawFdContainer>)) -> Result<Self, Self::Error> {
        let (value, mut fds) = value;
        Ok(Self::try_parse_fd(value, &mut fds)?.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSupportedModifiersRequest {
    pub window: u32,
    pub depth: u8,
    pub bpp: u8,
}
impl GetSupportedModifiersRequest {
    /// Opcode for the GetSupportedModifiers request
    pub const fn opcode() -> u8 { 6 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let depth_bytes = self.depth.serialize();
        let bpp_bytes = self.bpp.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            depth_bytes[0],
            bpp_bytes[0],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_supported_modifiers<Conn>(conn: &Conn, window: u32, depth: u8, bpp: u8) -> Result<Cookie<'_, Conn, GetSupportedModifiersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSupportedModifiersRequest {
        window,
        depth,
        bpp,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSupportedModifiersReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub window_modifiers: Vec<u64>,
    pub screen_modifiers: Vec<u64>,
}
impl TryParse for GetSupportedModifiersReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (num_window_modifiers, remaining) = u32::try_parse(remaining)?;
        let (num_screen_modifiers, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let (window_modifiers, remaining) = crate::x11_utils::parse_list::<u64>(remaining, num_window_modifiers.try_into().or(Err(ParseError::ParseError))?)?;
        let (screen_modifiers, remaining) = crate::x11_utils::parse_list::<u64>(remaining, num_screen_modifiers.try_into().or(Err(ParseError::ParseError))?)?;
        let result = GetSupportedModifiersReply { response_type, sequence, length, window_modifiers, screen_modifiers };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSupportedModifiersReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetSupportedModifiersReply {
    /// Get the value of the `num_window_modifiers` field.
    ///
    /// The `num_window_modifiers` field is used as the length field of the `window_modifiers` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_window_modifiers(&self) -> u32 {
        self.window_modifiers.len()
            .try_into().unwrap()
    }
    /// Get the value of the `num_screen_modifiers` field.
    ///
    /// The `num_screen_modifiers` field is used as the length field of the `screen_modifiers` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn num_screen_modifiers(&self) -> u32 {
        self.screen_modifiers.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PixmapFromBuffersRequest {
    pub pixmap: xproto::Pixmap,
    pub window: xproto::Window,
    pub width: u16,
    pub height: u16,
    pub stride0: u32,
    pub offset0: u32,
    pub stride1: u32,
    pub offset1: u32,
    pub stride2: u32,
    pub offset2: u32,
    pub stride3: u32,
    pub offset3: u32,
    pub depth: u8,
    pub bpp: u8,
    pub modifier: u64,
    pub buffers: Vec<RawFdContainer>,
}
impl PixmapFromBuffersRequest {
    /// Opcode for the PixmapFromBuffers request
    pub const fn opcode() -> u8 { 7 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let pixmap_bytes = self.pixmap.serialize();
        let window_bytes = self.window.serialize();
        let num_buffers = u8::try_from(self.buffers.len()).expect("`buffers` has too many elements");
        let num_buffers_bytes = num_buffers.serialize();
        let width_bytes = self.width.serialize();
        let height_bytes = self.height.serialize();
        let stride0_bytes = self.stride0.serialize();
        let offset0_bytes = self.offset0.serialize();
        let stride1_bytes = self.stride1.serialize();
        let offset1_bytes = self.offset1.serialize();
        let stride2_bytes = self.stride2.serialize();
        let offset2_bytes = self.offset2.serialize();
        let stride3_bytes = self.stride3.serialize();
        let offset3_bytes = self.offset3.serialize();
        let depth_bytes = self.depth.serialize();
        let bpp_bytes = self.bpp.serialize();
        let modifier_bytes = self.modifier.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            pixmap_bytes[0],
            pixmap_bytes[1],
            pixmap_bytes[2],
            pixmap_bytes[3],
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            num_buffers_bytes[0],
            0,
            0,
            0,
            width_bytes[0],
            width_bytes[1],
            height_bytes[0],
            height_bytes[1],
            stride0_bytes[0],
            stride0_bytes[1],
            stride0_bytes[2],
            stride0_bytes[3],
            offset0_bytes[0],
            offset0_bytes[1],
            offset0_bytes[2],
            offset0_bytes[3],
            stride1_bytes[0],
            stride1_bytes[1],
            stride1_bytes[2],
            stride1_bytes[3],
            offset1_bytes[0],
            offset1_bytes[1],
            offset1_bytes[2],
            offset1_bytes[3],
            stride2_bytes[0],
            stride2_bytes[1],
            stride2_bytes[2],
            stride2_bytes[3],
            offset2_bytes[0],
            offset2_bytes[1],
            offset2_bytes[2],
            offset2_bytes[3],
            stride3_bytes[0],
            stride3_bytes[1],
            stride3_bytes[2],
            stride3_bytes[3],
            offset3_bytes[0],
            offset3_bytes[1],
            offset3_bytes[2],
            offset3_bytes[3],
            depth_bytes[0],
            bpp_bytes[0],
            0,
            0,
            modifier_bytes[0],
            modifier_bytes[1],
            modifier_bytes[2],
            modifier_bytes[3],
            modifier_bytes[4],
            modifier_bytes[5],
            modifier_bytes[6],
            modifier_bytes[7],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], self.buffers))
    }
}
pub fn pixmap_from_buffers<Conn>(conn: &Conn, pixmap: xproto::Pixmap, window: xproto::Window, width: u16, height: u16, stride0: u32, offset0: u32, stride1: u32, offset1: u32, stride2: u32, offset2: u32, stride3: u32, offset3: u32, depth: u8, bpp: u8, modifier: u64, buffers: Vec<RawFdContainer>) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = PixmapFromBuffersRequest {
        pixmap,
        window,
        width,
        height,
        stride0,
        offset0,
        stride1,
        offset1,
        stride2,
        offset2,
        stride3,
        offset3,
        depth,
        bpp,
        modifier,
        buffers,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuffersFromPixmapRequest {
    pub pixmap: xproto::Pixmap,
}
impl BuffersFromPixmapRequest {
    /// Opcode for the BuffersFromPixmap request
    pub const fn opcode() -> u8 { 8 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let pixmap_bytes = self.pixmap.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            pixmap_bytes[0],
            pixmap_bytes[1],
            pixmap_bytes[2],
            pixmap_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn buffers_from_pixmap<Conn>(conn: &Conn, pixmap: xproto::Pixmap) -> Result<CookieWithFds<'_, Conn, BuffersFromPixmapReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = BuffersFromPixmapRequest {
        pixmap,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply_with_fds(&slices, fds)?)
}

#[derive(Debug, PartialEq, Eq)]
pub struct BuffersFromPixmapReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: u16,
    pub height: u16,
    pub modifier: u64,
    pub depth: u8,
    pub bpp: u8,
    pub strides: Vec<u32>,
    pub offsets: Vec<u32>,
    pub buffers: Vec<RawFdContainer>,
}
impl BuffersFromPixmapReply {
    fn try_parse_fd<'a>(remaining: &'a [u8], fds: &mut Vec<RawFdContainer>) -> Result<(Self, &'a [u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (nfd, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (width, remaining) = u16::try_parse(remaining)?;
        let (height, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (modifier, remaining) = u64::try_parse(remaining)?;
        let (depth, remaining) = u8::try_parse(remaining)?;
        let (bpp, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(6..).ok_or(ParseError::ParseError)?;
        let (strides, remaining) = crate::x11_utils::parse_list::<u32>(remaining, nfd.try_into().or(Err(ParseError::ParseError))?)?;
        let (offsets, remaining) = crate::x11_utils::parse_list::<u32>(remaining, nfd.try_into().or(Err(ParseError::ParseError))?)?;
        let fds_len = usize::try_from(nfd).or(Err(ParseError::ParseError))?;
        if fds.len() < fds_len { return Err(ParseError::ParseError) }
        let mut buffers = fds.split_off(fds_len);
        std::mem::swap(fds, &mut buffers);
        let result = BuffersFromPixmapReply { response_type, sequence, length, width, height, modifier, depth, bpp, strides, offsets, buffers };
        Ok((result, remaining))
    }
}
impl TryFrom<(&[u8], Vec<RawFdContainer>)> for BuffersFromPixmapReply {
    type Error = ParseError;
    fn try_from(value: (&[u8], Vec<RawFdContainer>)) -> Result<Self, Self::Error> {
        let (value, mut fds) = value;
        Ok(Self::try_parse_fd(value, &mut fds)?.0)
    }
}
impl BuffersFromPixmapReply {
    /// Get the value of the `nfd` field.
    ///
    /// The `nfd` field is used as the length field of the `strides` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn nfd(&self) -> u8 {
        self.strides.len()
            .try_into().unwrap()
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn dri3_query_version(&self, major_version: u32, minor_version: u32) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, major_version, minor_version)
    }
    fn dri3_open(&self, drawable: xproto::Drawable, provider: u32) -> Result<CookieWithFds<'_, Self, OpenReply>, ConnectionError>
    {
        open(self, drawable, provider)
    }
    fn dri3_pixmap_from_buffer<A>(&self, pixmap: xproto::Pixmap, drawable: xproto::Drawable, size: u32, width: u16, height: u16, stride: u16, depth: u8, bpp: u8, pixmap_fd: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<RawFdContainer>,
    {
        pixmap_from_buffer(self, pixmap, drawable, size, width, height, stride, depth, bpp, pixmap_fd)
    }
    fn dri3_buffer_from_pixmap(&self, pixmap: xproto::Pixmap) -> Result<CookieWithFds<'_, Self, BufferFromPixmapReply>, ConnectionError>
    {
        buffer_from_pixmap(self, pixmap)
    }
    fn dri3_fence_from_fd<A>(&self, drawable: xproto::Drawable, fence: u32, initially_triggered: bool, fence_fd: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<RawFdContainer>,
    {
        fence_from_fd(self, drawable, fence, initially_triggered, fence_fd)
    }
    fn dri3_fd_from_fence(&self, drawable: xproto::Drawable, fence: u32) -> Result<CookieWithFds<'_, Self, FDFromFenceReply>, ConnectionError>
    {
        fd_from_fence(self, drawable, fence)
    }
    fn dri3_get_supported_modifiers(&self, window: u32, depth: u8, bpp: u8) -> Result<Cookie<'_, Self, GetSupportedModifiersReply>, ConnectionError>
    {
        get_supported_modifiers(self, window, depth, bpp)
    }
    fn dri3_pixmap_from_buffers(&self, pixmap: xproto::Pixmap, window: xproto::Window, width: u16, height: u16, stride0: u32, offset0: u32, stride1: u32, offset1: u32, stride2: u32, offset2: u32, stride3: u32, offset3: u32, depth: u8, bpp: u8, modifier: u64, buffers: Vec<RawFdContainer>) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        pixmap_from_buffers(self, pixmap, window, width, height, stride0, offset0, stride1, offset1, stride2, offset2, stride3, offset3, depth, bpp, modifier, buffers)
    }
    fn dri3_buffers_from_pixmap(&self, pixmap: xproto::Pixmap) -> Result<CookieWithFds<'_, Self, BuffersFromPixmapReply>, ConnectionError>
    {
        buffers_from_pixmap(self, pixmap)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
