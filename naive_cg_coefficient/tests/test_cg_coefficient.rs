use naive_cg_coefficient::cg_coeffcient as cg;

#[test]
fn test_1() {
    let cg_1 = cg::CGCoefficient::new(2, 1, 1, 1, 3, 2); 
    let res_cg_1 = cg_1.calc_value(); 
    cg_1.show_list();
    println!("res_cg_1 = {}", res_cg_1);
}

#[test]
fn test_2() {
    let cg_1_c = cg::CGCoefficient::new(2, 1, 1, 1, 3, 2);
    let res_cg_1_c = unsafe {
        cg_1_c.calc_value_c()
    }; 

    cg_1_c.show_list();
    println!("res_cg_1_c = {}", res_cg_1_c);
}

#[test]
fn is_same_c_impl() {
    let cg_1 = cg::CGCoefficient::new(2, 1, 1, 1, 3, 2);
    let res_cg_1_c = unsafe {
        cg_1.calc_value_c()
    };
    assert!((cg_1.calc_value() - res_cg_1_c).abs() < std::f64::EPSILON);

    let cg_2 = cg::CGCoefficient::new(2, 2, 3, 2, 5, 4);
    let res_cg_2_c = unsafe {
        cg_2.calc_value_c()
    };
    assert!((cg_2.calc_value() - res_cg_2_c).abs() < std::f64::EPSILON);
}
