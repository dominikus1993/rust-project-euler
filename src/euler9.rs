pub mod euler9 {
    pub fn solve() -> Option<i32> {
        for i in 1..1000 {
            for j in 1..1000 {
                for k in 1..1000 {
                    if i + j + k == 1000 {
                        let mut vect: Vec<i32> = vec![i, j, k];
                        vect.sort();
                        if vect[0] * vect[0] + vect[1] * vect[1] == vect[2] * vect[2] {
                            return Some(vect[0] * vect[1] * vect[2]);
                        }
                    }
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn solve() {
        assert_eq!(euler9::solve(), Some(31875000));
    }
}
