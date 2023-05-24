use crate::factorial;
use crate::cg_coefficient_cffi;

#[derive(Debug,PartialEq, Eq)]
pub struct CGCoefficient {
   j_1 : i64,
   m_1 : i64,
   j_2 : i64,
   m_2 : i64,
   j_3 : i64,
   m_3 : i64,
} 

fn sign(n: u32) -> f64 {
    if n % 2 == 0 {
        1.0
    } else {
        -1.0
    }
}

impl CGCoefficient {
    pub fn new(j_1: i64, m_1: i64, j_2: i64, m_2: i64, j_3: i64, m_3:i64) -> CGCoefficient {
        CGCoefficient{j_1, m_1, j_2, m_2, j_3, m_3}
    }

    fn is_triangle(&self) -> bool {
        let (j_min, j_max) = (self.j_1 + self.j_2, (self.j_1 - self.j_2).abs());

        let is_triangle_m = self.m_3 != self.m_1 + self.m_2;
        let is_traianble_j = self.j_3 < j_min || j_max < self.j_3;

        is_triangle_m && is_traianble_j
    }

    fn delta(&self) -> f64 {
        ((2 * self.j_3 + 1) as f64) 
            *(factorial::factorial(self.j_3 + self.j_1 - self.j_2) as f64)
            *(factorial::factorial(self.j_3 - self.j_1 + self.j_2) as f64)
            *(factorial::factorial(self.j_1 + self.j_2 - self.j_3) as f64)
            /(factorial::factorial(self.j_3 + self.j_1 + self.j_2 + 1) as f64)
    }

    pub fn show_list(&self) {
        println!("(j_3, m_3) = ({}, {})", self.j_3, self.m_3);
        println!("(j_1, m_1) = ({}, {})", self.j_1, self.m_1);
        println!("(j_2, m_2) = ({}, {})", self.j_2, self.m_2);
    }

}

impl CGCoefficient {
    pub fn calc_value(&self) -> f64 {

        if self.is_triangle() {
            return 0.0;
        }

        let s = factorial::factorial(self.j_3 + self.m_3) 
            * factorial::factorial(self.j_3 - self.m_3);
        let s_1 = factorial::factorial(self.j_1 + self.m_1)
            * factorial::factorial(self.j_1 - self.m_1);
        let s_2 = factorial::factorial(self.j_2 + self.m_2)
            * factorial::factorial(self.j_2 - self.m_2);
        
        let s = ((s * s_1 * s_2) as f64).powf(0.5);

        let k_max = std::cmp::min( 
            std::cmp::min(self.j_1 + self.j_2 - self.j_3, self.j_1 - self.m_1)
            , self.j_2 + self.m_2
            );

        let mut res_cg = 0.0;
        for k in 0 .. k_max + 1 {
            let k_1 = factorial::factorial(self.j_1 + self.j_2 - self.j_3 - k);
            let k_2 = factorial::factorial(self.j_1 - self.m_1 - k);
            let k_3 = factorial::factorial(self.j_2 + self.m_2- k);
            let k_4 = factorial::factorial(self.j_3 - self.j_2 + self.m_1 + k);
            let k_5 = factorial::factorial(self.j_3 - self.j_1 - self.m_2 + k);

            if(k_1 == -100 || k_2 == -100 || k_3 == -100
               || k_4 == -100 || k_5 == -100){
                res_cg += 0.0;
            } else {
                res_cg += sign(k as u32)/((factorial::factorial(k) * k_1 * k_2 * k_3 * k_4 * k_5)  as f64);
            }
        }

        self.delta().powf(0.5) * s * res_cg 
    }
}

impl CGCoefficient {

    pub fn calc_value_c(&self) -> f64 {
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
        let res_cg_1_c = cg_1_c.calc_value_c();
        cg_1_c.show_list();
        println!("res_cg_1_c = {}", res_cg_1_c);
    }

    #[test]
    fn is_same_c_impl() {
        let cg_1 = CGCoefficient::new(2, 1, 1, 1, 3, 2);
        assert!((cg_1.calc_value() - cg_1.calc_value_c()).abs() < std::f64::EPSILON);

        let cg_2 = CGCoefficient::new(2, 2, 3, 2, 5, 4);
        assert!((cg_2.calc_value() - cg_2.calc_value_c()).abs() < std::f64::EPSILON);
    }
}
