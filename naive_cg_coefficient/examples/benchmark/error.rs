use naive_cg_coefficient;
use naive_cg_coefficient::cg_coeffcient as cg;

fn calc_error(j_1: i64, j_2: i64, j_3: i64, m_3: i64) -> Option<f64>{
    let mut sq_sum_of_cg_coefficient = 0.0_f64;
    for m_1 in -j_1 .. j_1 + 1 {
        for m_2 in -j_2 .. j_2 +1 {
            let cg_coeff = cg::CGCoefficient::new(j_1, m_1, j_2, m_2, j_3, m_3);
            sq_sum_of_cg_coefficient += (cg_coeff.calc_value()).powi(2);
        }
    } 

    if sq_sum_of_cg_coefficient < std::f64::EPSILON {
        None
    } else if (sq_sum_of_cg_coefficient - 1.0).abs() < std::f64::EPSILON {
        None
    } else {
        Some((1.0 - sq_sum_of_cg_coefficient).abs() )
    }
}

fn calc_error_all_j(j_3_max: i64) -> Vec<(f64, f64)> {
    let mut res: Vec<(f64,f64)> = vec![];
    for j_3 in 0 .. j_3_max + 1 {
        for m_3 in -j_3 .. j_3 + 1 {
            for j_1 in 0 .. j_3 + 1 {
                for j_2 in 0 .. j_3 + 1 {
                    if let Some(cg_error) = calc_error(j_1, j_2, j_3, m_3) {
                    res.push((((j_1 * j_2) as f64 ).powf(0.5), -cg_error.log10()));
                    }
                }
            } 
        }
    }

    res
}


fn main() {
    let res = calc_error_all_j(20);

    for (x, y) in res {
        println!("{}\t{}", x, y);
    }
}
