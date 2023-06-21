use naive_cg_coefficient;
use naive_cg_coefficient::cg_coeffcient as cg;

fn main() {
    println!("Calculated by Rust");
    let cg_5 = cg::CGCoefficient::new(6,0,4,0,2,0);
    cg_5.show_list();
    let value_5 = cg_5.calc_value();
    println!("value_cg_5 = {}", value_5);

    println!("Calculated by C via CFFI");
    let cg_5_cffi = cg::CGCoefficient::new(6,0,4,0,2,0);
    cg_5_cffi.show_list();
    let value_5_cffi = unsafe{
        cg_5_cffi.calc_value_c()
    };
    println!("value_5_cffi = {}", value_5_cffi);
}
