pub mod euler6 {

    pub fn solve () -> i32 {
        let nums: Vec<i32> = (0..101).collect();
        let sum_of_square:i32 = nums.iter().map(|&x| i32::pow(x, 2)).sum();
        let sum = nums.iter().sum();
        let square = i32::pow(sum, 2);
        return square - sum_of_square;
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    
    #[test]
    fn solve() {
        assert_eq!(euler6::solve(), 25164150);
    }
}