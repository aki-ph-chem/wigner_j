use naive_cg_coefficient;
use naive_cg_coefficient::cg_coeffcient as cg;

fn main(){
    let cg_1 = cg::CGCoefficient::new(2,2,3,2,5,4);
    cg_1.show_list();
    let res_cg_1 = cg_1.calc_value();
    println!("res_cg_1 = {}", res_cg_1);

    let cg_2 = cg::CGCoefficient::new(2,1,1,1,3, 2);
    cg_2.show_list();
    let res_cg_2 = cg_2.calc_value();
    println!("res_cg_2 = {}", res_cg_2);

    let cg_3 = cg::CGCoefficient::new(12,11,11,11,13,12);
    cg_3.show_list();
    let res_cg_3 = cg_3.calc_value();
    println!("res_cg_3 = {}", res_cg_3);

    let cg_4 = cg::CGCoefficient::new(6,3,2,1,8,4);
    cg_4.show_list();
    let res_cg_4 = cg_4.calc_value();
    println!("res_cg_4 = {}", res_cg_4);

    let cg_5 = cg::CGCoefficient::new(6,0,4,0,2,0);
    cg_5.show_list();
    let value_5 = cg_5.calc_value();
    println!("res_cg_5 = {}", value_5);

    // ok for f64 
    let cg_6 = cg::CGCoefficient::new(5, 0, 4, 0, 9, 0);
    cg_6.show_list();
    let value_6 = cg_6.calc_value();
    println!("res_cg_6 = {}", value_6);

    let cg_7 = cg::CGCoefficient::new(10,5,12,2,22,7);
    cg_7.show_list();
    let value_7 = cg_7.calc_value();
    println!("res_cg_7 = {}", value_7);
}
