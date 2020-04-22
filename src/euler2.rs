pub mod euler2 {

    fn rem(a: i32, b: i32) -> i32 {
        return a - (b * (a / b));
    }

    fn produce_fib() -> Vec<i32> {
        let mut result = vec![0, 1];
        let mut i = 1;
        while result[i] < 4000000 {
            println!("{}", result[i]);
            result.push(result[i] + result[i - 1]);
            
            i = i + 1;
        }
        return result;
    }

    pub fn solve() -> i32 {
        let fibs = produce_fib();
        return fibs
                .iter()
                .filter(|&&x| rem(x, 2) == 0)
                .sum();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(euler2::solve(), 4613732);
    }
}
