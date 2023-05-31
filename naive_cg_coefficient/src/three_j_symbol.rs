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
//! use naive_cg_coefficient::three_j_symbol;
//!
//! fn main() {
//!     // create a new `TreeJSymbol` instance
//!     let three_j = TreeJSymbol::new(2, 1, 1, 1, 3, 2);
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

#[derive(Debug,PartialEq, Eq)]
pub struct TreeJSymbol {
   j_1 : i64,
   m_1 : i64,
   j_2 : i64,
   m_2 : i64,
   j_3 : i64,
   m_3 : i64,
}

impl TreeJSymbol {
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
    /// let tree_j = TreeJSymbol::new(2, 1, 1, 1, 3, 2);
    /// ```
    ///
    pub fn new(j_1: i64, m_1: i64, j_2: i64, m_2: i64, j_3: i64, m_3:i64) -> TreeJSymbol {
        TreeJSymbol{j_1, m_1, j_2, m_2, j_3, m_3}
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
        internal::sign((self.j_1  - self.j_2 - self.m_3) as i32)
           /(self.j_3 as f64 + 1.0).powf(0.5) 
           * internal::calc_cg_raw(self.j_1, self.j_2, self.j_3,
                                   self.m_1, self.m_2, -self.m_3)
    }
}

#[cfg(test)]
mod test_3j_symbol {
    use super::*;

    #[test]
    fn show_list() {
        let tj_1 = TreeJSymbol::new(5, 2, 4, 3, 9, 5);
        tj_1.show_list();
    }
}
