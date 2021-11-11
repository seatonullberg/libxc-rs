use std::ffi::CStr;

use libc::c_char;
use libxc_sys;

fn _rust_string_from_c_buf(c_buf: *const c_char) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice.to_owned()
}

/// Returns the current LibXC version as a semantic versioning tuple.
pub fn version() -> (i32, i32, i32) {
    let mut major: i32 = -1;
    let mut minor: i32 = -1;
    let mut micro: i32 = -1;
    unsafe { libxc_sys::xc_version(&mut major, &mut minor, &mut micro) };
    (major, minor, micro)
}

/// Returns the current LibXC version as a string.
pub fn version_string() -> String {
    let c_buf: *const c_char = unsafe { libxc_sys::xc_version_string() };
    _rust_string_from_c_buf(c_buf)
}

/// Returns the reference for the current LibXC version as a string.
pub fn reference() -> String {
    let c_buf: *const c_char = unsafe { libxc_sys::xc_reference() };
    _rust_string_from_c_buf(c_buf)
}

/// Returns the doi of the reference for the current LibXC version as a string.
pub fn reference_doi() -> String {
    let c_buf: *const c_char = unsafe { libxc_sys::xc_reference_doi() };
    _rust_string_from_c_buf(c_buf)
}

#[cfg(test)]
mod tests {
    use crate::util;

    #[test]
    fn version() {
        let (major, minor, micro) = util::version();
        assert!(major >= 0);
        assert!(minor >= 0);
        assert!(micro >= 0);
    }

    #[test]
    fn version_string() {
        let result = util::version_string();
        assert!(result.len() > 0);
    }

    #[test]
    fn reference() {
        let result = util::reference();
        assert!(result.len() > 0);
    }

    #[test]
    fn reference_doi() {
        let result = util::reference_doi();
        assert!(result.len() > 0);
    }
}
