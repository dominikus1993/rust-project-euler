pub mod euler5 {
    fn rem(a: i32, b: i32) -> i32 {
        return a - (b * (a / b));
    }

    pub fn is_divisable_by_all_numbers (num: i32, nums: &Vec<i32>) -> bool {
        let result = nums.iter().any(|&x| rem(num, x) != 0);
        return !result
    }

    pub fn solve () -> i32 {
        let nums: Vec<i32> = (2..20).collect();
        let mut i = 2520;
        loop {
            if is_divisable_by_all_numbers(i, &nums) {
                break;
            }
            i = i + 1;
        }
        return i;
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    
    #[test]
    fn solve() {
        assert_eq!(euler5::solve(), 232792560);
    }
}