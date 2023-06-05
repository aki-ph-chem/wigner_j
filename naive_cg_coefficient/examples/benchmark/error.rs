use naive_cg_coefficient;
use naive_cg_coefficient::cg_coeffcient as cg;
enum CalcBy {
    Factorial,
    Binomial,
}

fn calc_error(j_1: i64, j_2: i64, j_3: i64, m_3: i64, calc_by: CalcBy) -> Option<f64>{
    let mut sq_sum_of_cg_coefficient = 0.0_f64;
    for m_1 in -j_1 .. j_1 + 1 {
        for m_2 in -j_2 .. j_2 +1 {
            let cg_coeff = cg::CGCoefficient::new(j_1, m_1, j_2, m_2, j_3, m_3);
            sq_sum_of_cg_coefficient += 
                match calc_by {
                CalcBy::Factorial => (cg_coeff.calc_value()).powi(2),
                CalcBy::Binomial => (cg_coeff.calc_value_binomial()).powi(2),
                };
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

fn calc_error_all_j(j_3_max: i64, calc_by: CalcBy) -> Vec<(f64, f64)> {
    let mut res: Vec<(f64,f64)> = vec![];
    for j_3 in 0 .. j_3_max + 1 {
        for m_3 in -j_3 .. j_3 + 1 {
            for j_1 in 0 .. j_3 + 1 {
                for j_2 in 0 .. j_3 + 1 {

                    let error = match calc_by {
                        CalcBy::Factorial => calc_error(j_1, j_2, j_3, m_3, CalcBy::Factorial), 
                        CalcBy::Binomial => calc_error(j_1, j_2, j_3, m_3, CalcBy::Binomial), 
                    };

                    if let Some(cg_error) = error {
                    res.push((((j_1 * j_2) as f64 ).powf(0.5), -cg_error.log10()));
                    }
                }
            } 
        }
    }

    res
}

fn main() {
    let argv:Vec<_> = std::env::args().collect(); 

    if argv.len() < 2 {
        eprintln!("Error: invalid args");
        std::process::exit(1);
    }

    let max_j = argv[1].clone().parse::<i64>().expect("Invalid number");
    let res = calc_error_all_j(max_j, CalcBy::Factorial);
    //let res = calc_error_all_j(max_j, CalcBy::Binomial);

    for (x, y) in res {
        println!("{}\t{}", x, y);
    }
}
