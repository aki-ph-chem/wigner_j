use naive_cg_coefficient::cg_coeffcient as cg; 

#[test]
fn overflow() {


    /* overflow! 
       let tmp = cg::CGCoefficient::new(1,0,0,-1,1,-1); 
       tmp.calc_value_binomial();
       */

    /* overflow ! 
       let tmp = cg::CGCoefficient::new(0,-1,1,0,1,-1);
       tmp.calc_value_binomial();
       */
    let j_3_max = 3;

    for j_3 in 0 .. j_3_max + 1 {
        for m_3 in -j_3 .. j_3 +1 {
            for j_2 in 0 .. j_3 + 1 {
                for j_1 in 0 .. j_3 + 1{
                    for m_2 in -j_1 .. j_1 + 1{
                        for m_1 in -j_2 .. j_2 +1 {
                            let cg_tmp = cg::CGCoefficient::new(j_1, m_1, j_2, m_2, j_3, m_3 );
                            let _tmp = cg_tmp.calc_value_binomial();
                        }
                    }
                }
            }
        } 
    }

}
