use crate::factorial;

/// Calculates the sign
///
/// if n is even `sign()` return 1.0, otherwise return -1.0
///
pub fn sign(n: i32) -> f64 {
    if n %2 == 0 {
        1.0
    } else {
        -1.0 
    }
} 

pub fn is_triangle(j_1: i64, j_2: i64, j_3: i64,
                   m_1: i64, m_2: i64, m_3: i64) -> bool {

        let (j_min, j_max) = (j_1 + j_2, (j_1 - j_2).abs());

        let is_triangle_m = m_3 != m_1 + m_2;
        let is_traianble_j = j_3 < j_min || j_max < j_3;

        is_triangle_m && is_traianble_j
}

pub fn delta(j_1: i64, j_2: i64, j_3: i64) -> f64{
        ((2 * j_3 + 1) as f64) 
            *(factorial::factorial(j_3 + j_1 - j_2) as f64)
            *(factorial::factorial(j_3 - j_1 + j_2) as f64)
            *(factorial::factorial(j_1 + j_2 - j_3) as f64)
            /(factorial::factorial(j_3 + j_1 + j_2 + 1) as f64)
}

pub fn show_list(j_1: i64, j_2: i64, j_3: i64,
                   m_1: i64, m_2: i64, m_3: i64) {

        println!("<{},{},{},{}|{},{} >",
                 j_1, m_1, j_2, m_2, j_3, m_3);
} 

pub fn show_list_3j(j_1: i64, j_2: i64, j_3: i64,
                   m_1: i64, m_2: i64, m_3: i64) {

        println!("( {} ,{}, {} )\n( {}, {}, {} ) "
                 ,j_1, j_2, j_3, m_1, m_2, m_3);
} 

pub fn calc_cg_raw(j_1: i64, j_2: i64, j_3: i64,
                   m_1: i64, m_2: i64, m_3: i64) -> f64 {

        if is_triangle(j_1, j_2, j_3, m_1, m_2, m_3) {
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
            std::cmp::min(j_1 + j_2 - j_3, j_1 - m_1)
            , j_2 + m_2
            );

        let mut res_cg = 0.0;
        for k in 0 .. k_max + 1 {
            let k_1 = factorial::factorial(j_1 + j_2 - j_3 - k);
            let k_2 = factorial::factorial(j_1 - m_1 - k);
            let k_3 = factorial::factorial(j_2 + m_2- k);
            let k_4 = factorial::factorial(j_3 - j_2 + m_1 + k);
            let k_5 = factorial::factorial(j_3 - j_1 - m_2 + k);

            res_cg += sign(k as i32)/((factorial::factorial(k) * k_1 * k_2 * k_3 * k_4 * k_5)  as f64);
        }

        delta(j_1, j_2, j_3).powf(0.5) * s * res_cg 
    }
