use naive_cg_coefficient;
use naive_cg_coefficient as cg;

fn main(){
    let j_3_max = 20;

    // j_1, m_1, j_2, m_2, cg
    let mut res: Vec<(i64, i64, i64, i64,
                      i64, i64, f64)> = vec![];

    for j_3 in 0 .. j_3_max + 1 {
        for m_3 in -j_3 .. j_3 +1 {
            for j_2 in 0 .. j_3 + 1 {
                for j_1 in 0 .. j_3 + 1{
                    for m_2 in -j_1 .. j_1 + 1{
                        for m_1 in -j_2 .. j_2 +1 {
                            let cg_tmp = cg::cg_coeffcient::CGCoefficient::new(j_1, m_1, j_2, m_2, j_3, m_3 );
                            res.push((j_1, m_1, j_2, m_2, j_3, m_3, cg_tmp.calc_value()));
                        }
                    }
                }
            }
        } 
    }

    for (j_1, m_1, j_2, m_2, j_3, m_3, cg_coeff) in res {
        //println!("<{}, {}, {}, {}|{} {}> = {}", j_1, m_1, j_2, m_2, j_3, m_3, cg_coeff);
        println!("{},{},{},{},{},{},{}", j_1, m_1, j_2, m_2, j_3, m_3, cg_coeff);
    }
}
