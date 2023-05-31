//! # CG Coefficient
//!
//! This module provides functionality for calculating Clebsch-Gordan (CG) coefficients.
//! It includes a struct `CGCoefficient` and its implementation with methods for calculating
//! the CG coefficient value and other related operations.
//!
//! ## Example
//!
//! ```rust
//!   use naive_cg_coefficient::cg_coeffcient::CGCoefficient; 
//!
//! fn main() {
//!     // Create a new CGCoefficient instance
//!     let cg = CGCoefficient::new(2, 1, 1, 1, 3, 2);
//!
//!     // Calculate the CG coefficient value
//!     let value = cg.calc_value();
//!
//!     // Print the result
//!     println!("CG coefficient value: {}", value);
//! }
//! ```
//!
//! For more information on CG coefficients and their calculations, refer to the documentation
//! of the individual methods and the associated mathematical formulas.

use crate::factorial;
use crate::cg_coefficient_cffi;
use crate::internal;

#[derive(Debug,PartialEq, Eq)]
pub struct CGCoefficient {
   j_1 : i64,
   m_1 : i64,
   j_2 : i64,
   m_2 : i64,
   j_3 : i64,
   m_3 : i64,
} 

impl CGCoefficient {
    /// Create a new instance of `CGCoefficient`.
    ///
    /// # Arguments
    ///
    /// * `j_1` - The value of j1.
    /// * `m_1` - The value of m1.
    /// * `j_2` - The value of j2.
    /// * `m_2` - The value of m2.
    /// * `j_3` - The value of j3.
    /// * `m_3` - The value of m3.
    ///
    /// # Examples
    ///
    /// ```
    /// let cg = CGCoefficient::new(2, 1, 1, 1, 3, 2);
    /// ```
    pub fn new(j_1: i64, m_1: i64, j_2: i64, m_2: i64, j_3: i64, m_3:i64) -> CGCoefficient {
        CGCoefficient{j_1, m_1, j_2, m_2, j_3, m_3}
    }
}

impl CGCoefficient {

    /// show list of j and m like `<j_1, m_1, j_2, m_2|j_3, m_3>`
    pub fn show_list(&self) {
        internal::show_list(self.j_1, self.j_2, self.j_3,
                            self.m_1, self.m_2, self.m_3);
    }

    /// Calculates the value of the CGCoefficient.
     pub fn calc_value(&self) -> f64 {

         internal::calc_cg_raw(self.j_1, self.j_2, self.j_3,
                               self.m_1, self.m_2, self.m_3)
    }
}

impl CGCoefficient {

    /// Calculates the value of the CGCoefficient 
    /// from `cg_coefficient_cffi::CGcoeff_c()` 
    pub unsafe fn calc_value_c(&self) -> f64 {
        cg_coefficient_cffi::CGcoeff_c(self.j_1 as f64, self.m_1 as f64,
                                       self.j_2 as f64, self.m_2 as f64,
                                       self.j_3 as f64, self.m_3 as f64)
    }  
}

#[cfg(test)]
mod test_cg_coefficient{
    use super::*;

    #[test]
    fn test_1() {
        let cg_1 = CGCoefficient::new(2, 1, 1, 1, 3, 2); 
        let res_cg_1 = cg_1.calc_value(); 
        cg_1.show_list();
        println!("res_cg_1 = {}", res_cg_1);
    }

    #[test]
    fn test_2() {
        let cg_1_c = CGCoefficient::new(2, 1, 1, 1, 3, 2);
        let res_cg_1_c = unsafe {
            cg_1_c.calc_value_c()
        }; 

        cg_1_c.show_list();
        println!("res_cg_1_c = {}", res_cg_1_c);
    }

    #[test]
    fn is_same_c_impl() {
        let cg_1 = CGCoefficient::new(2, 1, 1, 1, 3, 2);
        let res_cg_1_c = unsafe {
            cg_1.calc_value_c()
        };
        assert!((cg_1.calc_value() - res_cg_1_c).abs() < std::f64::EPSILON);

        let cg_2 = CGCoefficient::new(2, 2, 3, 2, 5, 4);
        let res_cg_2_c = unsafe {
            cg_2.calc_value_c()
        };
        assert!((cg_2.calc_value() - res_cg_2_c).abs() < std::f64::EPSILON);
    }
}
