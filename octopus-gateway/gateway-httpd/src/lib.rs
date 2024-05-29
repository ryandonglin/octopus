#[allow(clippy::new_without_default)]
use http::request::{Parts as ReqParts, Parts};
use http::request::Builder as ReqBuilder;
pub use http::HeaderMap as HMap;
use std::ops::Deref;
use http::Method;
use gateway_error::{ErrorType::*, Result}

mod http_header_support;
use http_header_support::CaseHttpHeaders;



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
    raw_parse_fallback: Vec<u8>

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

    pub fn new_no_case(size_hint: Option<usize>) -> Self {
        let mut base  = ReqBuilder::new().body(()).unwrap().into_parts().0;
        base.headers.reserve(http_header_map_upper_bound(size_hint));
        RequestHeader {
            base,
            header_name_map: None,
            raw_parse_fallback: vec![]
        }
    }

    pub fn build(
        method: impl TryInto<Method>,
        path: &[u8],
        size_hint: Option<usize>
    ) -> Result<Self> {
        let mut req = Self::build_no_case(method, path, size_hint)?;
        req.header_name_map = Some(CaseMap::with_capacity(http_header_map_upper_bound(
            size_hint,
        )));
        Ok(req)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
