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

#[allow(clippy::new_without_default)]
use http::request::{Parts as ReqParts, Parts};
use http::request::Builder as ReqBuilder;
pub use http::HeaderMap as HMap;
use std::ops::Deref;
use http::{HeaderName, HeaderValue, Method, Uri};
use http::header::{AsHeaderName, IntoHeaderName};
use gateway_error::{ErrorType::*, Result};

mod http_header_support;
use http_header_support::CaseHttpHeaders;
use crate::http_header_support::IntoCaseHeader;

pub mod prelude {
    pub use crate::RequestHeader;
}


type CaseMap = HMap<CaseHttpHeaders>;

/// the http request header type
///
/// this type is similar to  [http::request::Parts] but preserves header name case
/// it also preserve raw request path if it is not valid utf8
///
/// [RequestHeader] implements [Deref] for [http::request::Parts] so it can be used as it is in most places
#[derive(Debug)]
pub struct RequestHeader {
    /// [http::request::ReqParts] type parameter. including most frequently used part in standart http request
    /// such as uri, headers, method, etc.
    base: ReqParts,
    header_name_map: Option<CaseMap>,
    // store the raw path in bytes only if it is invalid in utf8;
    raw_path_fallback: Vec<u8>

}

/// method to extract standard [http::request::Parts] from [RequestHeader];
impl AsRef<ReqParts> for RequestHeader {
    fn as_ref(&self) -> &ReqParts {
        &self.base
    }
}

/// dereference for [RequestHeader] to fetch original http request parts
impl Deref for RequestHeader {
    type Target = ReqParts;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl RequestHeader {

    fn new_no_case(size_hint: Option<usize>) -> Self {
        let mut base  = ReqBuilder::new().body(()).unwrap().into_parts().0;
        base.headers.reserve(http_header_map_upper_bound(size_hint));
        RequestHeader {
            base,
            header_name_map: None,
            raw_path_fallback: vec![]
        }
    }

    /// create new [RequestHeader] with the given method and path.
    ///
    /// note that the param 'path' can be non UTF-8
    pub fn build(
        method: impl TryInto<Method>,
        path: &[u8],
        size_hint: Option<usize>
    ) -> Result<Self> {
        let mut req = Self::build_no_case(method, path, size_hint)?;

        /// problems fix, cause by previous step [Self::build_no_case] return wrong type, which return
        /// [(RequestHeader, Box<Error>)] tuple type while actually expected [RequestHeader] type, cause by
        /// [gateway_error::Result] wrong type definition
        req.header_name_map = Some(CaseMap::with_capacity(http_header_map_upper_bound(
            size_hint,
        )));

        Ok(req)
    }

    pub fn build_no_case(
        method: impl TryInto<Method>,
        path: &[u8],
        size_hint: Option<usize>,
    ) -> Result<Self> {
        let mut req = Self::new_no_case(size_hint);
        req.base.method = method
            .try_into()
            .explain_err(InvalidHTTPHeader, |_| "invalid method")?;

        if let Ok(p) = std::str::from_utf8(path) {
            let uri = Uri::builder()
                .path_and_query(p)
                .build()
                .explain_err(InvalidHTTPHeader, |_| format!("invalid uri {}", p))?;

            req.base.uri = uri;
            // keep raw_path empty, no need to store twice
        } else {
            // put a valid utf-8 path into base for read only access
            let lossy_str = String::from_utf8_lossy(path);
            let uri = Uri::builder()
                .path_and_query(lossy_str.as_ref())
                .build()
                .explain_err(InvalidHTTPHeader, |_| format!("invalid url {}", lossy_str))?;

            req.base.uri = uri;
            req.raw_path_fallback = path.to_vec();
        }

        Ok(req)
    }

    /// append the header name and value to `self`
    ///
    /// if there are already some header with(under) the same name, a new name will be added without
    /// removing other existed;
    pub fn append_header(
        &mut self,
        name: impl IntoCaseHeader,
        value: impl TryInto<HeaderValue>
    ) -> Result<bool>{

        let header_value = value
            .try_into()
            .explain_err(InvalidHTTPHeader, |_| "invalid value to append to request header")?;

        ///
        append_header_value(
            self.header_name_map.as_mut(),
            &mut self.base.headers,
            name,
            header_value
        )
    }

    /// insert specific header-name into `self`
    ///
    /// different with [Self::append_header()], this method will replace all other existing headers
    /// with the same name (case insensitive), see the different of [HMap::insert()] and [HMap::append()]
    pub fn insert_header(
        &mut self,
        name: impl IntoCaseHeader,
        value: impl TryInto<HeaderValue>
    ) -> Result<bool> {

        let header_value = value
            .try_into()
            .explain_err(InvalidHTTPHeader, |_| "invalid value to insert to request header")?;

        insert_header_value(
            self.header_name_map.as_mut(),
            &self.base.headers,
            name,
            header_value
        )
    }

    /// using lifetime annotation `'a` which will automatically management the lifetime of reference
    pub fn remove_header<'a, N: ?Sized>(&mut self, name:&'a N) -> Option<HeaderValue>
    where
        &'a N: 'a + AsHeaderName {remove_header(self.header_name_map.as_mut(), &mut self.base.headers, name)}

    /// set the request of http request, [POST] or [GET], etc
    pub fn set_method(&mut self, method: Method) {
        self.method = method;
    }

    pub fn set_uri(&mut self, uri: Uri) {
        self.uri = uri;
    }

    pub fn raw_path(&mut self) -> &[u8] {
        if !self.raw_path_fallback.is_empty() {
            &self.raw_path_fallback
        } else {
            self.base
                .uri
                .path_and_query()
                .as_ref()
                .unwrap()
                .as_str()
                .as_bytes()
        }
    }


}

// This function returns an upper bound on the size of the header map used inside the http crate.
// As of version 0.2, there is a limit of 1 << 15 (32,768) items inside the map. There is an
// assertion against this size inside the crate so we want to avoid panicking by not exceeding this
// upper bound.
fn http_header_map_upper_bound(size_hint: Option<usize>) -> usize {
    // Even though the crate has 1 << 15 as the max size, calls to `with_capacity` invoke a
    // function that returns the size + size / 3.
    //
    // See https://github.com/hyperium/http/blob/34a9d6bdab027948d6dea3b36d994f9cbaf96f75/src/header/map.rs#L3220
    //
    // Therefore we set our max size to be even lower so we guarantee ourselves we won't hit that
    // upper bound in the crate. Any way you cut it, 4,096 headers is insane.
    const PINGORA_MAX_HEADER_COUNT: usize = 4096;
    const INIT_HEADER_SIZE: usize = 8;

    // We select the size hint or the max size here such that we pick a value substantially lower
    // 1 << 15 with room to grow the header map.
    std::cmp::min(
        size_hint.unwrap_or(INIT_HEADER_SIZE),
        PINGORA_MAX_HEADER_COUNT,
    )
}

#[inline]
fn append_header_value<T>(
    name_map: Option<&mut CaseMap>,
    value_map: &mut HMap<T>,
    name: impl IntoCaseHeader,
    value: T
) -> Result<bool> {

    let case_header_name = name.into_case_header_name();

    let header_name: HeaderName = case_header_name
        .as_slice()
        .try_into()
        .or_err(InvalidHTTPHeader, "invalid http header name");

    if let Some(name_map) = name_map {
        name_map.append(header_name.clone(), case_header_name)
    }

    Ok(value_map.append(header_name, value))
}

#[inline]
fn insert_header_value<T>(
    name_map: Option<&mut CaseMap>,
    value_map: &mut HMap<T>,
    name: impl IntoCaseHeader,
    value: T
) -> Result<bool> {

    let case_header_name = name.into_case_header_name();

    let header_name: HeaderName = case_header_name
        .as_slice()
        .try_into()
        .or_err(InvalidHTTPHeader, "invalid http header name");

    if let Some(name_map) = name_map {
        name_map.insert(header_name.clone(), case_header_name)
    }

    Ok(value_map.insert(header_name, value))
}

#[inline]
fn remove_header<'a, T, N: ?Sized>(
    name_map: Option<&mut CaseMap>,
    value_map: &mut HMap<T>,
    name: &'a N
) -> Option<T>
    where
        &'a N: 'a + AsHeaderName {
    if let Some(name_map) = name_map {
        name_map.remove(name);
    }

    value_map.remove(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
