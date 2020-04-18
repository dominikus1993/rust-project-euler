pub mod euler1 {
    fn rem(a: i32, b: i32) -> i32 {
        return a - (b * (a / b));
    }

    pub fn solve() -> i32 {
        return (0..1000)
            .filter(|&x| rem(x, 5) == 0 || rem(x, 3) == 0)
            .sum();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(euler1::solve(), 233168);
    }
}
