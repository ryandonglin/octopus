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

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ImmutStr {
    Static(&'static str),
    Owned(Box<str>),
}

impl ImmutStr {

    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            ImmutStr::Static(s) => s,
            ImmutStr::Owned(s) => s.as_ref(),
        }
    }


    pub fn is_owned(&self) -> bool {
        match self {
            ImmutStr::Static(_) => false,
            ImmutStr::Owned(_) => true,
        }
    }
}

///
impl fmt::Display for ImmutStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}