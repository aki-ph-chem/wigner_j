//! # Wigner 3j-symbol
//!
//! This module provide functionality for calculating Wigner 3j symbol.
//!
//! It includes a struct `TreeJSymbol` and its implemention with methods 
//! for caculating the value of Wigner 3j symbol.
//!
//! ## Example
//!
//! ```rust
//! use naive_cg_coefficient::three_j_symbol::ThreeJSymbol;
//!
//! fn main() {
//!     // create a new `TreeJSymbol` instance
//!     let three_j = ThreeJSymbol::new(2, 1, 1, 1, 3, 2);
//!
//!     // Calculate the 3j-symbol value
//!     let value = three_j.calc_value();
//!
//!     // Print the result
//!     three_j.show_list();
//!     println!("thre_j: {}", value);
//! } 
//! ```
//!

use crate::internal;
use crate::cg_coeffcient;

#[derive(Debug,PartialEq, Eq)]
pub struct ThreeJSymbol {
   j_1 : i64,
   m_1 : i64,
   j_2 : i64,
   m_2 : i64,
   j_3 : i64,
   m_3 : i64,
}

impl ThreeJSymbol {
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
    /// use naive_cg_coefficient::three_j_symbol::ThreeJSymbol;
    /// let tree_j = ThreeJSymbol::new(2, 1, 1, 1, 3, 2);
    /// ```
    ///
    pub fn new(j_1: i64, m_1: i64, j_2: i64, m_2: i64, j_3: i64, m_3:i64) -> ThreeJSymbol {
        ThreeJSymbol{j_1, m_1, j_2, m_2, j_3, m_3}
    }

    /// show list of j and m like 
    ///
    ///```text
    ///(j_1, j_2, j_3)
    ///(m_1, m_2, m_3)
    ///```
    ///
    ///
    pub fn show_list(&self) {
        internal::show_list_3j(self.j_1, self.j_2, self.j_3, 
                               self.m_1, self.m_2, self.m_3);
    } 

    /// Calculate th value of th 3j-symbol
    pub fn calc_value(&self) -> f64 {
        (internal::sign((self.j_1  - self.j_2 - self.m_3) as i32)/( (2 * self.j_3) as f64 + 1.0).powf(0.5))
            * internal::calc_cg_raw(self.j_1, self.j_2, self.j_3,
                                    self.m_1, self.m_2, -self.m_3)
    }
}

impl From<cg_coeffcient::CGCoefficient> for ThreeJSymbol {
    fn from(value: cg_coeffcient::CGCoefficient) -> ThreeJSymbol {
        let (j_1, j_2, j_3, m_1, m_2, m_3) = value.get_jm();
        ThreeJSymbol::new(j_1, m_1, j_2, m_2, j_3, -m_3)
    }
}

#[cfg(test)]
mod test_3j_symbol {
    use super::*;

    #[test]
    fn show_list() {
        let tj_1 = ThreeJSymbol::new(5, 2, 4, 3, 9, 5);
        tj_1.show_list();
    }

    #[test]
    fn test_from() {
        let tree_j_1 = ThreeJSymbol::new(7, 2, 4, 3 ,9, 5);
        tree_j_1.show_list();

        let cg = cg_coeffcient::CGCoefficient::new(7, 2, 4, 3, 9, -5);
        let tree_j_2 = ThreeJSymbol::from(cg);
        tree_j_2.show_list();

        assert_eq!(tree_j_1, tree_j_2);
    }
}
