pub fn binomial_rec(n: i64, k: i64) -> i64 {
    if n == k || k == 0 {
        1
    } else{
        binomial_rec(n - 1, k -1) + binomial_rec(n - 1, k)
    }
}

#[cfg(test)]
mod tests_binomial_recursive {
    use super::*;

    #[test]
    fn test_binomial_10() {
        assert_eq!(10, binomial_rec(10,1));
        assert_eq!(1, binomial_rec(3,0));
    }

    #[test]
    fn test_binomial_100() {
        assert_eq!(100, binomial_rec(100,1));
        assert_eq!(1, binomial_rec(100,0));
    }

    #[test]
    fn test_binomial_5() {
        let ans = [1,5,10,10,5,1];
        for i in 0..ans.len() {
            assert_eq!(ans[i], binomial_rec(5, i as i64));
        }
    }

    #[test]
    fn test_binomial_6() {
        let ans = [1,6,15,20,15,6,1];
        for i in 0..ans.len() {
            assert_eq!(ans[i], binomial_rec(6, i as i64));
        }
    }
} 

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
