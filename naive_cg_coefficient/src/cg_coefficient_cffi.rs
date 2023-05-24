//! Wraps the C implementation of the function `CGcoeff()`
//! in `src/c/cg_coefficient.c`

#[link(name="cg_coefficient_c", kind="static")]
extern {
    fn CGcoeff(j_3: f64, m_3: f64,
               j_1: f64, m_1: f64,
               j_2: f64, m_2: f64) -> f64;
}

/// for calculate the Clebsch-Gordan coefficient
pub unsafe fn CGcoeff_c(j_1: f64, m_1: f64,
                 j_2: f64, m_2: f64,
                 j_3: f64, m_3: f64) -> f64 {
        CGcoeff(j_3, m_3, j_1, m_1, j_2, m_2)
}
