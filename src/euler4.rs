pub mod euler4 {

    pub fn is_palindrome(num: i64) -> bool {
        let num_str = num.to_string();
        let reversed_num = num_str.chars().rev().collect::<String>();
        return num_str == reversed_num;
    }

    pub fn solve () -> Option<i64> {
        return (0..1000)
                    .flat_map(|x| (0..1000).map(move |y| x * y))
                    .filter(|&x| is_palindrome(x))
                    .max();
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    
    #[test]
    fn solve() {
        assert_eq!(euler4::solve(), Some(906609));
    }

    #[test]
    fn is_palindome() {
        assert_eq!(euler4::is_palindrome(12321), true);
        assert_eq!(euler4::is_palindrome(22), true);
        assert_eq!(euler4::is_palindrome(12123), false);
    }
}