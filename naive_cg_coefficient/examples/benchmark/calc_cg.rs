use naive_cg_coefficient;
use naive_cg_coefficient as cg;

fn delta_set(j_1: i64 ,j_2: i64, j_3: i64) {
    if j_1 + j_2 - j_3 < 0{
        println!("j_1 + j_2 - j_3 = {}", j_1 + j_2 - j_3);
    } else if j_1 - j_2 + j_3 < 0 {
        println!("j_1 - j_2 + j_3 = {}", j_1 - j_2 + j_3);
    } else if -j_1 + j_2 + j_3 < 0 {
        println!("-j_1 + j_2 + j_3 = {}", -j_1 + j_2 + j_3);
    }
}

enum CalcBy{
    Factorial,
    Binomial,
}

fn get_cg_value(cg_coefficient: &cg::cg_coeffcient::CGCoefficient, calc_by :&CalcBy) -> f64 {
    match calc_by {
        CalcBy::Factorial => cg_coefficient.calc_value(),
        CalcBy::Binomial => cg_coefficient.calc_value_binomial(),
    }
}

fn main(){
    let argv:Vec<_> = std::env::args().collect();

    if argv.len() < 3 {
        eprintln!("Error: invalid args");
        let help = r"
            cargo run --example calc_cg binomial 3 # calculate by binomial for j_max = 3 
            cargo run --example calc_cg factorial 3 # calculate by factorial for j_max = 3 
            "; 
            eprintln!("usage: {}", help);
        std::process::exit(1);
    }

    let j_3_max = &argv[2].parse::<i64>().expect("Invalid number");
    let calc_method = &argv[1];

    let calc_by = match calc_method.as_str() {
        "factorial" =>  CalcBy::Factorial,
        "binomial" =>  CalcBy::Binomial,
        _ => {eprintln!("Error: invalid method"); 
            std::process::exit(1)},
    };

    // j_1, m_1, j_2, m_2, cg
    let mut res: Vec<(i64, i64, i64, i64,
                      i64, i64, f64)> = vec![];

    for j_3 in 0 .. *j_3_max + 1 {
        for m_3 in -j_3 .. j_3 +1 {
            for j_2 in 0 .. j_3 + 1 {
                for j_1 in 0 .. j_3 + 1{
                    for m_2 in -j_1 .. j_1 + 1{
                        for m_1 in -j_2 .. j_2 +1 {
                            let cg_tmp = cg::cg_coeffcient::CGCoefficient::new(j_1, m_1, j_2, m_2, j_3, m_3 );
                            res.push((j_1, m_1, j_2, m_2, j_3, m_3, get_cg_value(&cg_tmp, &calc_by)));
                        }
                    }
                }
            }
        } 
    }

    for (j_1, m_1, j_2, m_2, j_3, m_3, cg_coeff) in res {
        println!("<{}, {}, {}, {}|{} {}> = {}", j_1, m_1, j_2, m_2, j_3, m_3, cg_coeff);
        delta_set(j_1, j_2, j_3);
    }

}
