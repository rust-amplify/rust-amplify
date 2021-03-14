// Rust language amplification library providing multiple generic trait
// implementations, type wrappers, derive macros and other language enhancements
//
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Martin Habovstiak <martin.habovstiak@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use libc::c_char;
use std::ffi::{CStr, CString};

pub trait TryIntoRaw {
    fn try_into_raw(self) -> Option<*const c_char>;
}

pub trait TryFromRaw {
    fn try_from_raw(ptr: *mut c_char) -> Option<Self>
    where
        Self: Sized;
}

impl TryIntoRaw for String {
    fn try_into_raw(self) -> Option<*const c_char> {
        CString::new(self)
            .map(CString::into_raw)
            .map(|ptr| ptr as *const c_char)
            .ok()
    }
}

impl TryFromRaw for String {
    fn try_from_raw(ptr: *mut c_char) -> Option<String> {
        unsafe { CString::from_raw(ptr) }.into_string().ok()
    }
}

pub trait TryAsStr {
    fn try_as_str(self) -> Option<&'static str>;
}

pub trait TryIntoString {
    fn try_into_string(self) -> Option<String>;
}

impl TryAsStr for *const c_char {
    fn try_as_str(self: *const c_char) -> Option<&'static str> {
        if self.is_null() {
            return None;
        }
        unsafe { CStr::from_ptr(self) }.to_str().ok()
    }
}

impl TryIntoString for *mut c_char {
    fn try_into_string(self: *mut c_char) -> Option<String> {
        if self.is_null() {
            return None;
        }
        String::try_from_raw(self)
    }
}
