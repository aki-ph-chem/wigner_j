/// calculate binomial by recursion
pub fn binomial(n: i64, k: i64) -> f64 {
    if k > n {
        0.0
    } else if n == k || k == 0 {
        1.0
    } else{
        binomial(n - 1, k -1) + binomial(n - 1, k)
    }
}

#[cfg(test)]
mod tests_binomial_recursive {
    use super::*;

    #[test]
    fn test_binomial_10() {
        assert!((10.0 - binomial(10,1).abs() < std::f64::EPSILON));
        assert!((1.0 - binomial(3,0)).abs() < std::f64::EPSILON);
    }

    #[test]
    fn test_binomial_100() {
        assert!((100.0 - binomial(100,1).abs() < std::f64::EPSILON));
        assert!((1.0 - binomial(100,0).abs() < std::f64::EPSILON));
    }

    #[test]
    fn test_binomial_5() {
        let ans = [1.0,5.0,10.0,10.0,5.0,1.0];
        for i in 0..ans.len() {
            assert!((ans[i] - binomial(5, i as i64)).abs() < std::f64::EPSILON);
        }
    }

    #[test]
    fn test_binomial_6() {
        let ans = [1.0,6.0,15.0,20.0,15.0,6.0,1.0];
        for i in 0..ans.len() {
            assert!((ans[i] - binomial(6, i as i64)).abs() < std::f64::EPSILON);
        }
    }
} 

/// calculate binomial by dp 
pub fn binomial_dp(n: i64, k: i64, array: &mut Vec<Vec<i64>>){
    for i in 0..n + 1 {
        for j in 0..std::cmp::min(i,k) + 1 {
            if i == j || j == 0 {
                array[i as usize][j as usize] = 1;
            } else {
                array[i as usize][j as usize] = array[(i - 1) as usize][(j - 1) as usize] + array[(i - 1) as usize][j as usize];
            }
        }
    }
}

#[cfg(test)]
mod test_binomial_dp {
    use super::*;

    #[test]
    fn test_binomial_dp_1() {
        const ARRAY_SIZE: usize = 100;
        let mut array = vec![vec![0;ARRAY_SIZE];ARRAY_SIZE];

        binomial_dp(6,2, &mut array);
        assert_eq!(15, array[6][2]);
    }

    #[test]
    fn test_binomial_dp_2() {
        const ARRAY_SIZE: usize = 100;
        let mut array = vec![vec![0;ARRAY_SIZE];ARRAY_SIZE];

        let ans = [1,6,15,20,15,6,1];

        for i in 0..ans.len() {
            binomial_dp(6,i as i64,&mut array);
        }

        for k in 0..ans.len() {
            assert_eq!(ans[k], array[6][k]);
        }
    }
}
