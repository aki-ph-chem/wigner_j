use naive_cg_coefficient;
use naive_cg_coefficient::three_j_symbol::ThreeJSymbol;

fn main() {
    let three_1 = ThreeJSymbol::new(2,2,3,2,5,-4);
    three_1.show_list();
    let value_1 = three_1.calc_value();
    println!("tree_1: {}", value_1);

    let three_2 = ThreeJSymbol::new(6,0,4,0,2,0);
    three_2.show_list();
    let value_2 = three_2.calc_value(); 
    println!("tree_2: {}", value_2);

    let three_4 = ThreeJSymbol::new(2,1,1,1,3,-2);
    three_4.show_list();
    let value_4 = three_4.calc_value();
    println!("tree_4: {}", value_4);
}
