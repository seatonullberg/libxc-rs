use libxc_sys;

use num_traits::FromPrimitive;

use crate::error::FunctionalError;
use crate::util::{_rust_string_from_c_buf, functional_number};

/// Representation of a LibXC functional.
#[derive(Clone)]
pub struct Functional {
    xc_func: *mut libxc_sys::xc_func_type,
    xc_info: *const libxc_sys::xc_func_info_type,
}

#[derive(Clone, Copy, Debug, Display, FromPrimitive)]
pub enum FunctionalKind {
    Exchange = 0,
    Correlation = 1,
    ExchangeCorrelation = 2,
    Kinetic = 3,
}

#[derive(Clone, Copy, Debug, Display, FromPrimitive)]
pub enum FunctionalFamily {
    Unknown = -1,
    LDA = 1,
    GGA = 2,
    MGGA = 4,
    LCA = 8,
    OEP = 16,
    HybridGGA = 32,
    HybridMGGA = 64,
    HybridLDA = 128,
}

impl Functional {
    /// Constructs a [Functional] from a given id.
    pub fn from_id(id: i32, with_spin: bool) -> Result<Self, FunctionalError> {
        // Process `with_spin` argument.
        let with_spin: i32 = match with_spin {
            true => 2,
            false => 1,
        };
        // Allocate a LibXC functional type.
        let xc_func: *mut libxc_sys::xc_func_type = unsafe { libxc_sys::xc_func_alloc() };
        // Initialize the LibXC functional type.
        let init_result = unsafe { libxc_sys::xc_func_init(xc_func, id, with_spin) };
        if init_result != 0 {
            return Err(FunctionalError::FailedInitialization(init_result));
        }
        // Gather information about the functional.
        let xc_info: *const libxc_sys::xc_func_info_type =
            unsafe { libxc_sys::xc_func_get_info(xc_func) };
        // Return the initialized struct.
        Ok(Functional { xc_func, xc_info })
    }

    /// Constructs a [Functional] from a given name.
    pub fn from_name<'a, S>(name: S, with_spin: bool) -> Result<Self, FunctionalError>
    where
        S: Into<&'a str>,
    {
        match functional_number(name) {
            Ok(number) => Self::from_id(number, with_spin),
            Err(err) => Err(err),
        }
    }

    /// Returns the name of the functional.
    pub fn name(&self) -> String {
        let c_buf = unsafe { libxc_sys::xc_func_info_get_name(self.xc_info) };
        _rust_string_from_c_buf(c_buf)
    }

    /// Returns the ID of the functional.
    pub fn number(&self) -> i32 {
        unsafe { libxc_sys::xc_func_info_get_number(self.xc_info) }
    }

    /// Returns the kind of the functional.
    pub fn kind(&self) -> FunctionalKind {
        let result = unsafe { libxc_sys::xc_func_info_get_kind(self.xc_info) };
        FunctionalKind::from_i32(result).unwrap()
    }

    /// Returns the family of the functional.
    pub fn family(&self) -> FunctionalFamily {
        let result = unsafe { libxc_sys::xc_func_info_get_family(self.xc_info) };
        FunctionalFamily::from_i32(result).unwrap()
    }

    // I do not understand why this is not an array of integers.
    // TODO: Map this result to an enum for clarity.
    /// Returns the flags of the functional.
    pub fn flags(&self) -> i32 {
        unsafe { libxc_sys::xc_func_info_get_flags(self.xc_info) }
    }
}

#[cfg(test)]
mod tests {
    use crate::functional::{Functional, FunctionalFamily, FunctionalKind};

    #[test]
    fn from_id() {
        let func = Functional::from_id(32, true);
        match func {
            Ok(_) => (),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn from_name() {
        let func = Functional::from_name("XC_GGA_X_GAM", false);
        match func {
            Ok(_) => (),
            Err(_) => panic!(),
        }
    }

    #[test]
    fn name() {
        let func = Functional::from_id(1, true).unwrap();
        let name = func.name();
        assert_eq!(name, "Slater exchange");
    }

    #[test]
    fn number() {
        let number = 1;
        let func = Functional::from_id(number, true).unwrap();
        assert_eq!(number, func.number());
    }

    #[test]
    fn kind() {
        let func = Functional::from_id(1, false).unwrap();
        match func.kind() {
            FunctionalKind::Exchange => (),
            _ => panic!(),
        }
    }

    #[test]
    fn family() {
        let func = Functional::from_id(32, false).unwrap();
        match func.family() {
            FunctionalFamily::GGA => (),
            _ => panic!(),
        }
    }

    #[test]
    fn flags() {
        let func = Functional::from_id(1, false).unwrap();
        assert_eq!(func.flags(), 135);
    }

    #[test]
    fn clone() {
        let func = Functional::from_id(1, false).unwrap();
        let cloned = func.clone();
        assert_eq!(func.name(), cloned.name());
    }
}
