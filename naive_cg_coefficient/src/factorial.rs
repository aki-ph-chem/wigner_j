//!
//!Implementation of the function to compute factorials
//!

/// Implemented by recursive functions  
pub fn _factorial(n: i64) -> i64 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * _factorial(n - 1),
    }
}

#[cfg(test)]
mod tests_factorial{
    use super::*;

    #[ignore]
    #[test]
    fn test_factorial_0() {
        let factorial_0 = 1;
        let factorial_1 = 1;

        assert_eq!(factorial_0, _factorial(0));
        assert_eq!(factorial_1, _factorial(1));
    }

    #[ignore]
    #[test]
    fn test_factorial_n() {
        let factorial_3 = 6;
        assert_eq!(factorial_3, _factorial(3));

        let factorial_4 = 4 * 6;
        assert_eq!(factorial_4, _factorial(4));

        let factorial_6 = 6 * 5 * factorial_4;
        assert_eq!(factorial_6, _factorial(6));
    }
}

/// Implemented by 'for loop' 
///
/// Warning!: For n = 21 or greater, this factorial function will overflow 
/// because it exceeds the maximum value of `i64`.
pub fn factorial_i64(n: i64) -> i64 {
    if n == 0{
        1
    } else {
        let mut tmp = 1;
        for k in 1 .. n + 1 {
            tmp *= k;
        } 
        tmp
    }
}

/// Implemented by 'for loop' 
///
pub fn factorial(n: i64) -> f64 {
    if n == 0{
        1.0
    } else {
        let mut tmp = 1.0_f64;
        for k in 1 .. n + 1 {
            tmp *= k as f64;
        } 
        tmp
    }
}

#[cfg(test)]
mod tests_factorial_loop{
    use super::*;

    #[ignore]
    #[test]
    fn test_factorial_0() {
        let factorial_0 = 1;
        let factorial_1 = 1;

        assert_eq!(factorial_0, factorial_i64(0));
        assert_eq!(factorial_1, factorial_i64(1));
    }


    #[ignore]
    #[test]
    fn test_factorial_n() {
        let factorial_3 = 6;
        assert_eq!(factorial_3, factorial_i64(3));

        let factorial_4 = 4 * 6;
        assert_eq!(factorial_4, factorial_i64(4));

        let factorial_6 = 6 * 5 * factorial_4;
        assert_eq!(factorial_6, factorial_i64(6));
    }


    #[ignore]
    #[test]
    fn test_factorial_big_n() {
        let _res = factorial(10);
        let _res = factorial(20);
        // 21! > 2^{64} => Overflow
        let _res = factorial(21);
    }
}
