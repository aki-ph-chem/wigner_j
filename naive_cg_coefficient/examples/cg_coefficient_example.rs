use naive_cg_coefficient;
use naive_cg_coefficient::cg_coeffcient as cg;

fn main(){
    let cg_1 = cg::CGCoefficient::new(2,2,3,2,5,4);
    let res_cg_1 = cg_1.calc_value();
    println!("res_cg_1 = {}", res_cg_1);

    let cg_2 = cg::CGCoefficient::new(2,1,1,1,3, 2);
    let res_cg_2 = cg_2.calc_value();
    println!("res_cg_2 = {}", res_cg_2);

    let cg_3 = cg::CGCoefficient::new(12,11,11,11,13,12);
    let res_cg_3 = cg_3.calc_value();
    println!("res_cg_3 = {}", res_cg_3);

    let cg_4 = cg::CGCoefficient::new(6,3,2,1,8,4);
    let res_cg_4 = cg_4.calc_value();
    println!("res_cg_4 = {}", res_cg_4);
}
