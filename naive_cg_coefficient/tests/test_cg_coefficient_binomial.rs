use naive_cg_coefficient::cg_coeffcient as cg; 

#[test]
fn is_same_fact() {
    let cg_1 = cg::CGCoefficient::new(2,2,3,2,5,4);
    cg_1.show_list();
    let error =  (cg_1.calc_value() - cg_1.calc_value_binomial()).abs();
    println!("error_cg_1 = {}", error);
    assert!( error < std::f64::EPSILON);


    let cg_2 = cg::CGCoefficient::new(2,1,1,1,3,2);
    cg_2.show_list();
    let error =  (cg_2.calc_value() - cg_2.calc_value_binomial()).abs();
    println!("error_cg_2 = {}", error);
    assert!( error < std::f64::EPSILON);

    let cg_3 = cg::CGCoefficient::new(6,3,2,1,8,4);
    cg_3.show_list();
    let error =  (cg_3.calc_value() - cg_3.calc_value_binomial()).abs();
    println!("error_cg_3 = {}", error);
    assert!( error < std::f64::EPSILON);

    let cg_4 = cg::CGCoefficient::new(6,0,4,0,2,0);
    cg_4.show_list();
    let error =  (cg_4.calc_value() - cg_4.calc_value_binomial()).abs();
    println!("error_cg_4 = {}", error);
    assert!( error < std::f64::EPSILON);

    let cg_5 = cg::CGCoefficient::new(5, 0, 4, 0, 9, 0);
    cg_5.show_list();
    let error =  (cg_5.calc_value() - cg_5.calc_value_binomial()).abs();
    println!("error_cg_5 = {}", error);
    assert!( error < std::f64::EPSILON);
}
