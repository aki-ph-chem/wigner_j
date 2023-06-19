//!
//! functions for calculate Clebsch-Gordan coefficient and Wigner 3j-symbol
//!
use crate::factorial;
use crate::binomial;

/// Calculates the sign
///
/// if n is even `sign()` return 1.0, otherwise return -1.0
///
#[inline]
pub fn sign(n: i32) -> f64 {
    if n&1 != 0 {
        -1.0
    } else {
        1.0
    }
} 

/// test to determine if triangle conditions are met
pub fn is_triangle(j_1: i64, j_2: i64, j_3: i64,
                   m_1: i64, m_2: i64, m_3: i64) -> bool {

    let (j_min, j_max) = (j_1 + j_2, (j_1 - j_2).abs());

    let is_triangle_m = m_3 != m_1 + m_2;
    let is_traianble_j = j_3 < j_min || j_max < j_3;

    is_triangle_m && is_traianble_j
}

/// test  where j_1,j_2,j_3 is positive of not 
pub fn is_j_positive(j_1: i64, j_2: i64, j_3: i64) -> bool {
    j_1 < 0 && j_2 < 0 && j_3 < 0
 }

fn is_condition_j(j_1: i64, j_2: i64, j_3: i64) -> bool {
    let is_j_positive = j_1 >= 0 && j_2 >= 0 && j_3 >= 0;

    let (j_max, j_min) = (j_1 + j_2, (j_2 - j_1).abs());
    let is_traiangle = j_3 <= j_max && j_min <= j_3;

    is_j_positive && is_traiangle
}

fn is_condition_jm(j_1: i64, j_2: i64, j_3: i64,
                   m_1: i64, m_2: i64, m_3: i64) -> bool {
    let j_m = j_1 >= m_1.abs() && j_2 >= m_2.abs() && j_3 >= m_3.abs();
    let is_triangle_m = m_3 == m_1 + m_2;

    j_m && is_triangle_m
}

/// calculation of triangle factor
///
/// ```text
/// (j_1 + j_2 - j_3)!(j_1 - j_2 + j_3)!(j_2 - j_1 + j_3)!/(j_1 + j_2 + j_3 + 1)!
/// ```
///
pub fn delta(j_1: i64, j_2: i64, j_3: i64) -> f64{
    ((2 * j_3 + 1) as f64) 
        *(factorial::factorial(j_3 + j_1 - j_2) as f64)
        *(factorial::factorial(j_3 - j_1 + j_2) as f64)
        *(factorial::factorial(j_1 + j_2 - j_3) as f64)
        /(factorial::factorial(j_3 + j_1 + j_2 + 1) as f64)
}

/// show list of j_1, j_1, j_3, m_1, m_2, m_3
/// for clebsch-gordan coefficient
pub fn show_list(j_1: i64, j_2: i64, j_3: i64,
                 m_1: i64, m_2: i64, m_3: i64) {
    println!("<{},{},{},{}|{},{} >",
             j_1, m_1, j_2, m_2, j_3, m_3);
} 

/// show list of j_1, j_1, j_3, m_1, m_2, m_3
/// for wigner 3j-symbol
pub fn show_list_3j(j_1: i64, j_2: i64, j_3: i64,
                    m_1: i64, m_2: i64, m_3: i64) {
    println!("( {} ,{}, {} )\n( {}, {}, {} ) "
             ,j_1, j_2, j_3, m_1, m_2, m_3);
} 

/// calculate Clebsch-Gordan coefficient
pub fn calc_cg_raw(j_1: i64, j_2: i64, j_3: i64,
                   m_1: i64, m_2: i64, m_3: i64) -> f64 {

    if !is_condition_j(j_1, j_2, j_3) || !is_condition_jm(j_1, j_2, j_3, m_1, m_2, m_3){
        return 0.0;
    }

    let s = factorial::factorial(j_3 + m_3) 
        * factorial::factorial(j_3 - m_3);
    let s_1 = factorial::factorial(j_1 + m_1)
        * factorial::factorial(j_1 - m_1);
    let s_2 = factorial::factorial(j_2 + m_2)
        * factorial::factorial(j_2 - m_2);

    let s = ((s * s_1 * s_2) as f64).powf(0.5);

    let k_max = std::cmp::min( 
        std::cmp::min(j_1 + j_2 - j_3, j_1 - m_1), j_2 + m_2);
    let k_min = std::cmp::max(std::cmp::max(-j_3 + j_2 - m_1, -j_3 + j_1 + m_2), 0);

    let mut res_cg = 0.0;
    for k in k_min .. k_max + 1 {
        let k_1 = factorial::factorial(j_1 + j_2 - j_3 - k);
        let k_2 = factorial::factorial(j_1 - m_1 - k);
        let k_3 = factorial::factorial(j_2 + m_2- k);
        let k_4 = factorial::factorial(j_3 - j_2 + m_1 + k);
        let k_5 = factorial::factorial(j_3 - j_1 - m_2 + k);

        res_cg += sign(k as i32)/(factorial::factorial(k) * k_1 * k_2 * k_3 * k_4 * k_5);
    }

    delta(j_1, j_2, j_3).powf(0.5) * s * res_cg 
}

/// calculate Clebsch-Gordan coefficient by binomial 
pub fn calc_cg_binomial_raw(j_1: i64, j_2: i64, j_3: i64,
                            m_1: i64, m_2: i64, m_3: i64) -> f64 { 

    if !is_condition_j(j_1, j_2, j_3) || !is_condition_jm(j_1, j_2, j_3, m_1, m_2, m_3){
        return 0.0;
    }

    let s = (2.0 * (j_3 as f64) + 1.0)/((2.0 *(j_1 as f64) + 1.0 ) * (2.0 * (j_2 as f64) + 1.0)).powf(0.5); 

    let numerator = binomial::binomial(j_1 + j_2 + j_3 + 1, j_1 + j_2 - j_3) 
        * binomial::binomial(2*j_3, j_3 + m_3);

    let denominator = binomial::binomial(j_1 + j_2 + j_3 + 1, j_1 - j_2 + j_3) 
        * binomial::binomial(j_1 + j_2 + j_3 + 1, j_2 - j_1 + j_3)
        * binomial::binomial(2*j_1, j_1 + m_1) * binomial::binomial(2*j_2, j_2 + m_2); 

    let k_max = std::cmp::min(std::cmp::min(j_1 + j_2 - j_3, j_1 - m_1), j_2 + m_2);
    let k_min = std::cmp::max(std::cmp::max(-j_3 + j_2 - m_1, -j_3 + j_1 + m_2), 0);

    let mut res_cg = 0.0;
    for k in k_min .. k_max + 1 {
        res_cg += sign(k as i32) 
            *binomial::binomial(j_1 + j_2 - j_3, k)
            *binomial::binomial(j_3 + m_3, j_2 + m_2 - k)
            *binomial::binomial(j_3 - m_3, j_1 - m_1 - k);
    }

    s * (numerator / denominator).powf(0.5) * res_cg 
}
