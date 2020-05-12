pub mod euler7 {

    fn rem(a: i64, b: i64) -> i64 {
        return a - (b * (a / b));
    }

    pub fn is_prime(num: i64) -> bool {
        let uper_bound: i64 = (num as f64).sqrt() as i64;
        let result: Vec<i64> = (2..uper_bound)
        .filter(|&x| rem(num, x) == 0)
        .collect();
        return result.len() == 0
    }

    pub fn solve (num: usize) -> Option<i64> {

        let mut curr: i64 = 2;
        return std::iter::repeat_with(|| { let tmp = curr; curr +=1 ; tmp })
                .filter(|&x| is_prime(x))
                .skip(num - 1)  
                .next();
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    
    #[test]
    fn solve() {
        assert_eq!(euler7::solve(10001), Some(104743));
    }
}