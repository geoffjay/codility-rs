#![feature(test)]

extern crate test;

pub fn solution(mut a: Vec<i32>, b: usize) -> Vec<i32> {
    // This is cheating
    a.rotate_right(b);
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solution(vec![1, 2, 3, 4], 4), [1, 2, 3, 4]);
        assert_eq!(solution(vec![3, 8, 9, 7, 6], 3), [9, 7, 6, 3, 8]);
        assert_eq!(solution(vec![0, 0, 0], 1), [0, 0, 0]);
    }
}
