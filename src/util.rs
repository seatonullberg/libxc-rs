// TODO: xc_family_from_id

use std::ffi::{CStr, CString};
use std::mem::forget;

use libc::c_char;
use libxc_sys;

use crate::error::FunctionalError;

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

/// Returns the functional ID for a given name.
pub fn functional_number<'a, S>(name: S) -> Result<i32, FunctionalError>
where
    S: Into<&'a str>,
{
    let c_string = CString::new(name.into()).unwrap();
    let c_ptr: *const c_char = c_string.as_ptr();
    let number = unsafe { libxc_sys::xc_functional_get_number(c_ptr) };
    if number < 0 {
        Err(FunctionalError::InvalidName)
    } else {
        Ok(number)
    }
}

/// Returns the functional name for a given id.
pub fn functional_name(number: i32) -> Result<String, FunctionalError> {
    let numbers = available_functional_numbers();
    if numbers.contains(&number) {
        let c_buf = unsafe { libxc_sys::xc_functional_get_name(number) };
        Ok(_rust_string_from_c_buf(c_buf))
    } else {
        Err(FunctionalError::InvalidID)
    }
}

/// Returns the total number of available functionals.
pub fn number_of_functionals() -> i32 {
    unsafe { libxc_sys::xc_number_of_functionals() }
}

/// Returns a vec of all available functional IDs.
pub fn available_functional_numbers() -> Vec<i32> {
    let n_funcs = number_of_functionals() as usize;
    let length = n_funcs - 1;
    let mut vec: Vec<i32> = Vec::with_capacity(length);
    let ptr = vec.as_mut_ptr();
    forget(vec);
    unsafe { libxc_sys::xc_available_functional_numbers(ptr) };
    unsafe { Vec::from_raw_parts(ptr, length, length) }
}

/// Returns a vec of all available functional names.
pub fn available_functional_names() -> Vec<String> {
    // Getting a vec of strings through the C FFI is too complicated for me.
    // This should actually be done with an FFI call to `xc_available_functional_names`.
    available_functional_numbers()
        .iter()
        .map(|number| functional_name(*number).unwrap())
        .collect()
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

    #[test]
    fn functional_number_valid() {
        let name = "XC_GGA_X_GAM";
        match util::functional_number(name) {
            Ok(number) => assert_eq!(number, 32),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn functional_number_invalid() {
        let name = "INVALID_NAME";
        match util::functional_number(name) {
            Ok(_) => panic!(),
            Err(_) => (),
        }
    }

    #[test]
    fn functional_name_valid() {
        let number = 32;
        match util::functional_name(number) {
            Ok(name) => assert_eq!(name, "gga_x_gam"),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn functional_name_invalid() {
        let number = 0;
        match util::functional_name(number) {
            Ok(_) => panic!(),
            Err(_) => (),
        }
    }

    #[test]
    fn available_functional_numbers() {
        let n_funcs = util::number_of_functionals() as usize;
        let length = n_funcs - 1;
        let numbers = util::available_functional_numbers();
        assert_eq!(numbers.len(), length);
    }

    #[test]
    fn available_functional_names() {
        let n_funcs = util::number_of_functionals() as usize;
        let length = n_funcs - 1;
        let names = util::available_functional_names();
        assert_eq!(names.len(), length);
    }
}
