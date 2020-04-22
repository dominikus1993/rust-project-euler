pub mod euler3 {
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


    pub fn solve (num:i64) -> Option<i64> {
        let uper_bound: i64 = (num as f64).sqrt() as i64;
        let result = (2..uper_bound)
            .filter(|&x| rem(num, x) == 0)
            .filter(|&x| is_prime(x))
            .max();
        return result;
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(euler3::solve(600851475143), Some(6857));
    }
}