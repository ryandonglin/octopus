//                                      MIT License
//
// Copyright (c) [2024] [ryandonglin]
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::*;
use bytes::Bytes;
use http::header;

#[derive(Debug, Clone)]
pub struct CaseHttpHeaders(Bytes);

impl CaseHttpHeaders {
    pub fn new(name: String) -> Self {
        CaseHttpHeaders(name.into())
    }
}

impl CaseHttpHeaders {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn from_slice(buf: &[u8]) -> Self {
        CaseHttpHeaders(Bytes::copy_from_slice(buf))
    }
}

pub trait IntoCaseHeader {
    fn into_case_header_name(self) -> CaseHttpHeaders;
}

impl IntoCaseHeader for CaseHttpHeaders {
    fn into_case_header_name(self) -> CaseHttpHeaders {
        self
    }
}
impl IntoCaseHeader for String {
    fn into_case_header_name(self) -> CaseHttpHeaders {
        CaseHttpHeaders::new(self)
    }
}

impl IntoCaseHeader for &'static str {
    fn into_case_header_name(self) -> CaseHttpHeaders {
        CaseHttpHeaders(self.into())
    }
}

impl IntoCaseHeader for HeaderName {
    fn into_case_header_name(self) -> CaseHttpHeaders {
        CaseHttpHeaders(title_header_name(&self))
    }
}

impl IntoCaseHeader for &HeaderName {
    fn into_case_header_name(self) -> CaseHttpHeaders {
        CaseHttpHeaders(title_header_name(self))
    }
}

impl IntoCaseHeader for Bytes {
    fn into_case_header_name(self) -> CaseHttpHeaders {
        CaseHttpHeaders(self)
    }
}



pub(crate) fn title_header_name_str(header_name: &HeaderName) -> Option<&'static str> {

    /// using * to de-referencing
    Some(match *header_name {
        header::AGE => "Age",
        header::CACHE_CONTROL => "Cache-Control",
        header::CONNECTION => "Connection",
        header::CONTENT_TYPE => "Content-Type",
        header::CONTENT_ENCODING => "Content-Encoding",
        header::CONTENT_LENGTH => "Content-Length",
        header::DATE => "Date",
        header::TRANSFER_ENCODING => "Transfer-Encoding",
        header::HOST => "Host",
        header::SERVER => "Server",
        // TODO: add more const header here to map to their titled case
        // TODO: automatically upper case the first letter?
        _ => {
            return None;
        }
    })
}

pub fn title_header_name(header_name: &HeaderName) -> Bytes {
    title_header_name_str(header_name).map_or_else(
        || Bytes::copy_from_slice(header_name.as_str().as_bytes()),
        |s| Bytes::from_static(s.as_bytes()),
    )
}
