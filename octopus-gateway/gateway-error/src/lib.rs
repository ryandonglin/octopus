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

#[warn(clippy::all)]
mod immut_str;
pub use immut_str::ImmutStr;

use std::result::Result as StdResult;
use std::error::Error as ErrorTrait;

pub type BError = Box(Error);

pub type Result<T, E = BError> = StdResult(T, E);

#[derive(Debug)]
pub struct Error {
    /// the type of the error
    pub etype: ErrorType,
    /// the source of the error: who caused the error, upstream, downstream, internal ?
    pub esource: ErrorSource,
    /// if the error is retry-able
    pub retry: RetryType,
    /// chain to the cause of this error
    pub cause: Option<Box<dyn ErrorTrait + Send + Sync>>,
    /// an arbitrary string that explains the context when the error occurs
    pub context: Option<ImmutStr>,
}

impl RetryType {
    pub fn decide_reuse(&mut self, reused: bool) {
        if matches!(self, RetryType::ReuseOnly) {
            *self = RetryType::Decide(reused)
        }
    }

    pub fn retry(&self) -> bool {
        match self {
            RetryType::Decide(b) => *b,
            RetryType::ReuseOnly => {
                panic!("Retry is not decided")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorType {

    // connection errors
    ConnectionTimeout,
    ConnectionRefused,
    ConnectNoRoute,
    TLSHandshakeFailure,
    TLSHandshakeTimeout,
    InvalidCert,

    // protocol errors
    InvalidHTTPHeader,

}

#[derive(Debug)]
pub enum ErrorSource {
    /// The error is caused by the remote server side
    Upstream,
    /// The error is caused by the remote client side(which means the user side who invoke the gateway)
    Downstream,
    /// The error is caused by the gateway internal logic
    Internal,
    /// Errors that unknown caused source , unexpected error type will be set
    Unset,
}

#[derive(Debug)]
pub enum RetryType {
    Decide(bool),
    /// only retry when errors is from a reused connection
    ReuseOnly,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
