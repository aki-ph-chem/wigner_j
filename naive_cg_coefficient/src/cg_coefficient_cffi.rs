#[link(name="cg_coefficient_c", kind="static")]
extern {
    fn CGcoeff(j_3: f64, m_3: f64,
               j_1: f64, m_1: f64,
               j_2: f64, m_2: f64) -> f64;
}

pub fn CGcoeff_c(j_1: f64, m_1: f64,
                 j_2: f64, m_2: f64,
                 j_3: f64, m_3: f64) -> f64 {
    unsafe {
        CGcoeff(j_3, m_3, j_1, m_1, j_2, m_2)
    }
}
