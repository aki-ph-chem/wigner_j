pub fn factorial(n: i64) -> i64 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n - 1),
    }
}

#[cfg(test)]
mod tests_factorial{
    use super::*;

    #[test]
    fn test_factorial_0() {
        let factorial_0 = 1;
        let factorial_1 = 1;

        assert_eq!(factorial_0, factorial(0));
        assert_eq!(factorial_1, factorial(1));
    }

    #[test]
    fn test_factorial_n() {
        let factorial_3 = 6;
        assert_eq!(factorial_3, factorial(3));

        let factorial_4 = 4 * 6;
        assert_eq!(factorial_4, factorial(4));

        let factorial_6 = 6 * 5 * factorial_4;
        assert_eq!(factorial_6, factorial(6));
    }
}

pub fn factorial_loop(n: i64) -> i64 {
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

#[cfg(test)]
mod tests_factorial_loop{
    use super::*;

    #[test]
    fn test_factorial_0() {
        let factorial_0 = 1;
        let factorial_1 = 1;

        assert_eq!(factorial_0, factorial_loop(0));
        assert_eq!(factorial_1, factorial_loop(1));
    }

    #[test]
    fn test_factorial_n() {
        let factorial_3 = 6;
        assert_eq!(factorial_3, factorial_loop(3));

        let factorial_4 = 4 * 6;
        assert_eq!(factorial_4, factorial_loop(4));

        let factorial_6 = 6 * 5 * factorial_4;
        assert_eq!(factorial_6, factorial_loop(6));
    }
}

pub fn factorial_f64(n: i32) -> f64 {
    if n == 0{
        1.0
    } else {
        let mut tmp = 1;
        for k in 1 .. n + 1 {
            tmp *= k;
        } 
        tmp as f64
    }
}
