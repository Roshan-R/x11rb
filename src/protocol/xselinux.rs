// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `SELinux` X11 extension.

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
pub const X11_EXTENSION_NAME: &str = "SELinux";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (1, 0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionRequest {
    pub client_major: u8,
    pub client_minor: u8,
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
        let client_major_bytes = self.client_major.serialize();
        let client_minor_bytes = self.client_minor.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            client_major_bytes[0],
            client_minor_bytes[0],
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
pub fn query_version<Conn>(conn: &Conn, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major,
        client_minor,
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
    pub server_major: u16,
    pub server_minor: u16,
}
impl TryParse for QueryVersionReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (server_major, remaining) = u16::try_parse(remaining)?;
        let (server_minor, remaining) = u16::try_parse(remaining)?;
        let result = QueryVersionReply { response_type, sequence, length, server_major, server_minor };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetDeviceCreateContextRequest<'input> {
    pub context: &'input [u8],
}
impl<'input> SetDeviceCreateContextRequest<'input> {
    /// Opcode for the SetDeviceCreateContext request
    pub const fn opcode() -> u8 { 1 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + (&self.context[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.context[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn set_device_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceCreateContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDeviceCreateContextRequest;
impl GetDeviceCreateContextRequest {
    /// Opcode for the GetDeviceCreateContext request
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
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
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
pub fn get_device_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetDeviceCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceCreateContextRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetDeviceCreateContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetDeviceCreateContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetDeviceCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetDeviceContextRequest<'input> {
    pub device: u32,
    pub context: &'input [u8],
}
impl<'input> SetDeviceContextRequest<'input> {
    /// Opcode for the SetDeviceContext request
    pub const fn opcode() -> u8 { 3 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let device_bytes = self.device.serialize();
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            device_bytes[0],
            device_bytes[1],
            device_bytes[2],
            device_bytes[3],
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + (&self.context[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.context[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn set_device_context<'c, 'input, Conn>(conn: &'c Conn, device: u32, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceContextRequest {
        device,
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetDeviceContextRequest {
    pub device: u32,
}
impl GetDeviceContextRequest {
    /// Opcode for the GetDeviceContext request
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
        let device_bytes = self.device.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            device_bytes[0],
            device_bytes[1],
            device_bytes[2],
            device_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_device_context<Conn>(conn: &Conn, device: u32) -> Result<Cookie<'_, Conn, GetDeviceContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceContextRequest {
        device,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDeviceContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetDeviceContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetDeviceContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDeviceContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetDeviceContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetWindowCreateContextRequest<'input> {
    pub context: &'input [u8],
}
impl<'input> SetWindowCreateContextRequest<'input> {
    /// Opcode for the SetWindowCreateContext request
    pub const fn opcode() -> u8 { 5 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + (&self.context[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.context[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn set_window_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetWindowCreateContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetWindowCreateContextRequest;
impl GetWindowCreateContextRequest {
    /// Opcode for the GetWindowCreateContext request
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
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
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
pub fn get_window_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetWindowCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetWindowCreateContextRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWindowCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetWindowCreateContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetWindowCreateContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetWindowCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetWindowCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetWindowContextRequest {
    pub window: xproto::Window,
}
impl GetWindowContextRequest {
    /// Opcode for the GetWindowContext request
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
        let window_bytes = self.window.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_window_context<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetWindowContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetWindowContextRequest {
        window,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetWindowContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetWindowContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetWindowContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetWindowContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetWindowContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub name: xproto::Atom,
    pub object_context: Vec<u8>,
    pub data_context: Vec<u8>,
}
impl TryParse for ListItem {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let value = remaining;
        let (name, remaining) = xproto::Atom::try_parse(remaining)?;
        let (object_context_len, remaining) = u32::try_parse(remaining)?;
        let (data_context_len, remaining) = u32::try_parse(remaining)?;
        let (object_context, remaining) = crate::x11_utils::parse_u8_list(remaining, object_context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let object_context = object_context.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let (data_context, remaining) = crate::x11_utils::parse_u8_list(remaining, data_context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let data_context = data_context.to_vec();
        // Align offset to multiple of 4
        let offset = remaining.as_ptr() as usize - value.as_ptr() as usize;
        let misalignment = (4 - (offset % 4)) % 4;
        let remaining = remaining.get(misalignment..).ok_or(ParseError::ParseError)?;
        let result = ListItem { name, object_context, data_context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListItem {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ListItem {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(12);
        self.name.serialize_into(bytes);
        let object_context_len = u32::try_from(self.object_context.len()).expect("`object_context` has too many elements");
        object_context_len.serialize_into(bytes);
        let data_context_len = u32::try_from(self.data_context.len()).expect("`data_context` has too many elements");
        data_context_len.serialize_into(bytes);
        bytes.extend_from_slice(&self.object_context);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
        bytes.extend_from_slice(&self.data_context);
        bytes.extend_from_slice(&[0; 3][..(4 - (bytes.len() % 4)) % 4]);
    }
}
impl ListItem {
    /// Get the value of the `object_context_len` field.
    ///
    /// The `object_context_len` field is used as the length field of the `object_context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn object_context_len(&self) -> u32 {
        self.object_context.len()
            .try_into().unwrap()
    }
    /// Get the value of the `data_context_len` field.
    ///
    /// The `data_context_len` field is used as the length field of the `data_context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn data_context_len(&self) -> u32 {
        self.data_context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetPropertyCreateContextRequest<'input> {
    pub context: &'input [u8],
}
impl<'input> SetPropertyCreateContextRequest<'input> {
    /// Opcode for the SetPropertyCreateContext request
    pub const fn opcode() -> u8 { 8 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + (&self.context[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.context[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn set_property_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPropertyCreateContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPropertyCreateContextRequest;
impl GetPropertyCreateContextRequest {
    /// Opcode for the GetPropertyCreateContext request
    pub const fn opcode() -> u8 { 9 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
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
pub fn get_property_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyCreateContextRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyCreateContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetPropertyCreateContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetPropertyCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetPropertyUseContextRequest<'input> {
    pub context: &'input [u8],
}
impl<'input> SetPropertyUseContextRequest<'input> {
    /// Opcode for the SetPropertyUseContext request
    pub const fn opcode() -> u8 { 10 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + (&self.context[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.context[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn set_property_use_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPropertyUseContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPropertyUseContextRequest;
impl GetPropertyUseContextRequest {
    /// Opcode for the GetPropertyUseContext request
    pub const fn opcode() -> u8 { 11 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
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
pub fn get_property_use_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyUseContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyUseContextRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyUseContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyUseContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetPropertyUseContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyUseContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetPropertyUseContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPropertyContextRequest {
    pub window: xproto::Window,
    pub property: xproto::Atom,
}
impl GetPropertyContextRequest {
    /// Opcode for the GetPropertyContext request
    pub const fn opcode() -> u8 { 12 }
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
        let property_bytes = self.property.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_property_context<Conn>(conn: &Conn, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Conn, GetPropertyContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyContextRequest {
        window,
        property,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetPropertyContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetPropertyContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPropertyDataContextRequest {
    pub window: xproto::Window,
    pub property: xproto::Atom,
}
impl GetPropertyDataContextRequest {
    /// Opcode for the GetPropertyDataContext request
    pub const fn opcode() -> u8 { 13 }
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
        let property_bytes = self.property.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            property_bytes[0],
            property_bytes[1],
            property_bytes[2],
            property_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_property_data_context<Conn>(conn: &Conn, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Conn, GetPropertyDataContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyDataContextRequest {
        window,
        property,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetPropertyDataContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetPropertyDataContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetPropertyDataContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPropertyDataContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetPropertyDataContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListPropertiesRequest {
    pub window: xproto::Window,
}
impl ListPropertiesRequest {
    /// Opcode for the ListProperties request
    pub const fn opcode() -> u8 { 14 }
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
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn list_properties<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, ListPropertiesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListPropertiesRequest {
        window,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListPropertiesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub properties: Vec<ListItem>,
}
impl TryParse for ListPropertiesReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (properties_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (properties, remaining) = crate::x11_utils::parse_list::<ListItem>(remaining, properties_len.try_into().or(Err(ParseError::ParseError))?)?;
        let result = ListPropertiesReply { response_type, sequence, length, properties };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListPropertiesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl ListPropertiesReply {
    /// Get the value of the `properties_len` field.
    ///
    /// The `properties_len` field is used as the length field of the `properties` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn properties_len(&self) -> u32 {
        self.properties.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetSelectionCreateContextRequest<'input> {
    pub context: &'input [u8],
}
impl<'input> SetSelectionCreateContextRequest<'input> {
    /// Opcode for the SetSelectionCreateContext request
    pub const fn opcode() -> u8 { 15 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + (&self.context[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.context[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn set_selection_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetSelectionCreateContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSelectionCreateContextRequest;
impl GetSelectionCreateContextRequest {
    /// Opcode for the GetSelectionCreateContext request
    pub const fn opcode() -> u8 { 16 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
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
pub fn get_selection_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionCreateContextRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionCreateContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionCreateContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetSelectionCreateContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionCreateContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetSelectionCreateContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetSelectionUseContextRequest<'input> {
    pub context: &'input [u8],
}
impl<'input> SetSelectionUseContextRequest<'input> {
    /// Opcode for the SetSelectionUseContext request
    pub const fn opcode() -> u8 { 17 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let context_len = u32::try_from(self.context.len()).expect("`context` has too many elements");
        let context_len_bytes = context_len.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            context_len_bytes[0],
            context_len_bytes[1],
            context_len_bytes[2],
            context_len_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        let length_so_far = length_so_far + (&self.context[..]).len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), (&self.context[..]).into(), Cow::Borrowed(&padding0)], vec![]))
    }
}
pub fn set_selection_use_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetSelectionUseContextRequest {
        context,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_without_reply(&slices, fds)?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSelectionUseContextRequest;
impl GetSelectionUseContextRequest {
    /// Opcode for the GetSelectionUseContext request
    pub const fn opcode() -> u8 { 18 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
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
pub fn get_selection_use_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionUseContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionUseContextRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionUseContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionUseContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetSelectionUseContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionUseContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetSelectionUseContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSelectionContextRequest {
    pub selection: xproto::Atom,
}
impl GetSelectionContextRequest {
    /// Opcode for the GetSelectionContext request
    pub const fn opcode() -> u8 { 19 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let selection_bytes = self.selection.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            selection_bytes[0],
            selection_bytes[1],
            selection_bytes[2],
            selection_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_selection_context<Conn>(conn: &Conn, selection: xproto::Atom) -> Result<Cookie<'_, Conn, GetSelectionContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionContextRequest {
        selection,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetSelectionContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetSelectionContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetSelectionDataContextRequest {
    pub selection: xproto::Atom,
}
impl GetSelectionDataContextRequest {
    /// Opcode for the GetSelectionDataContext request
    pub const fn opcode() -> u8 { 20 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let selection_bytes = self.selection.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            selection_bytes[0],
            selection_bytes[1],
            selection_bytes[2],
            selection_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_selection_data_context<Conn>(conn: &Conn, selection: xproto::Atom) -> Result<Cookie<'_, Conn, GetSelectionDataContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionDataContextRequest {
        selection,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSelectionDataContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetSelectionDataContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetSelectionDataContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetSelectionDataContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetSelectionDataContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListSelectionsRequest;
impl ListSelectionsRequest {
    /// Opcode for the ListSelections request
    pub const fn opcode() -> u8 { 21 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
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
pub fn list_selections<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListSelectionsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListSelectionsRequest;
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSelectionsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub selections: Vec<ListItem>,
}
impl TryParse for ListSelectionsReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (selections_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (selections, remaining) = crate::x11_utils::parse_list::<ListItem>(remaining, selections_len.try_into().or(Err(ParseError::ParseError))?)?;
        let result = ListSelectionsReply { response_type, sequence, length, selections };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ListSelectionsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl ListSelectionsReply {
    /// Get the value of the `selections_len` field.
    ///
    /// The `selections_len` field is used as the length field of the `selections` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn selections_len(&self) -> u32 {
        self.selections.len()
            .try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetClientContextRequest {
    pub resource: u32,
}
impl GetClientContextRequest {
    /// Opcode for the GetClientContext request
    pub const fn opcode() -> u8 { 22 }
    /// Serialize this request into bytes for the provided connection
    #[allow(unused)]
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<(Vec<Cow<'input, [u8]>>, Vec<RawFdContainer>), ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        let length_so_far = 0;
        let resource_bytes = self.resource.serialize();
        let mut request0 = vec![
            extension_information.major_opcode,
            Self::opcode(),
            0,
            0,
            resource_bytes[0],
            resource_bytes[1],
            resource_bytes[2],
            resource_bytes[3],
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
}
pub fn get_client_context<Conn>(conn: &Conn, resource: u32) -> Result<Cookie<'_, Conn, GetClientContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetClientContextRequest {
        resource,
    };
    let (bytes, fds) = request0.serialize(conn)?;
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    Ok(conn.send_request_with_reply(&slices, fds)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetClientContextReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub context: Vec<u8>,
}
impl TryParse for GetClientContextReply {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (context_len, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (context, remaining) = crate::x11_utils::parse_u8_list(remaining, context_len.try_into().or(Err(ParseError::ParseError))?)?;
        let context = context.to_vec();
        let result = GetClientContextReply { response_type, sequence, length, context };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetClientContextReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl GetClientContextReply {
    /// Get the value of the `context_len` field.
    ///
    /// The `context_len` field is used as the length field of the `context` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn context_len(&self) -> u32 {
        self.context.len()
            .try_into().unwrap()
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xselinux_query_version(&self, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major, client_minor)
    }
    fn xselinux_set_device_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_device_create_context(self, context)
    }
    fn xselinux_get_device_create_context(&self) -> Result<Cookie<'_, Self, GetDeviceCreateContextReply>, ConnectionError>
    {
        get_device_create_context(self)
    }
    fn xselinux_set_device_context<'c, 'input>(&'c self, device: u32, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_device_context(self, device, context)
    }
    fn xselinux_get_device_context(&self, device: u32) -> Result<Cookie<'_, Self, GetDeviceContextReply>, ConnectionError>
    {
        get_device_context(self, device)
    }
    fn xselinux_set_window_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_window_create_context(self, context)
    }
    fn xselinux_get_window_create_context(&self) -> Result<Cookie<'_, Self, GetWindowCreateContextReply>, ConnectionError>
    {
        get_window_create_context(self)
    }
    fn xselinux_get_window_context(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetWindowContextReply>, ConnectionError>
    {
        get_window_context(self, window)
    }
    fn xselinux_set_property_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_property_create_context(self, context)
    }
    fn xselinux_get_property_create_context(&self) -> Result<Cookie<'_, Self, GetPropertyCreateContextReply>, ConnectionError>
    {
        get_property_create_context(self)
    }
    fn xselinux_set_property_use_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_property_use_context(self, context)
    }
    fn xselinux_get_property_use_context(&self) -> Result<Cookie<'_, Self, GetPropertyUseContextReply>, ConnectionError>
    {
        get_property_use_context(self)
    }
    fn xselinux_get_property_context(&self, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Self, GetPropertyContextReply>, ConnectionError>
    {
        get_property_context(self, window, property)
    }
    fn xselinux_get_property_data_context(&self, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Self, GetPropertyDataContextReply>, ConnectionError>
    {
        get_property_data_context(self, window, property)
    }
    fn xselinux_list_properties(&self, window: xproto::Window) -> Result<Cookie<'_, Self, ListPropertiesReply>, ConnectionError>
    {
        list_properties(self, window)
    }
    fn xselinux_set_selection_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_selection_create_context(self, context)
    }
    fn xselinux_get_selection_create_context(&self) -> Result<Cookie<'_, Self, GetSelectionCreateContextReply>, ConnectionError>
    {
        get_selection_create_context(self)
    }
    fn xselinux_set_selection_use_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_selection_use_context(self, context)
    }
    fn xselinux_get_selection_use_context(&self) -> Result<Cookie<'_, Self, GetSelectionUseContextReply>, ConnectionError>
    {
        get_selection_use_context(self)
    }
    fn xselinux_get_selection_context(&self, selection: xproto::Atom) -> Result<Cookie<'_, Self, GetSelectionContextReply>, ConnectionError>
    {
        get_selection_context(self, selection)
    }
    fn xselinux_get_selection_data_context(&self, selection: xproto::Atom) -> Result<Cookie<'_, Self, GetSelectionDataContextReply>, ConnectionError>
    {
        get_selection_data_context(self, selection)
    }
    fn xselinux_list_selections(&self) -> Result<Cookie<'_, Self, ListSelectionsReply>, ConnectionError>
    {
        list_selections(self)
    }
    fn xselinux_get_client_context(&self, resource: u32) -> Result<Cookie<'_, Self, GetClientContextReply>, ConnectionError>
    {
        get_client_context(self, resource)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
