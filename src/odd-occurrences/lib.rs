#![feature(test)]

extern crate test;

// Returns the element that has an odd number of occurrences. This isn't the
// most efficient method as it filters on the array every iteration.
pub fn solution(mut v: Vec<i32>) -> Option<i32> {
    let el = v[0];
    let count = v.iter().filter(|&n| *n == el).count();
    if count % 2 == 1 {
        Some(el)
    } else {
        v.retain(|&i| i != el);
        solution(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solution(vec![1, 3, 1, 3, 1]).unwrap(), 1);
        assert_eq!(solution(vec![4, 4, 4, 4, 4]).unwrap(), 4);
        assert_eq!(solution(vec![9, 3, 7, 7, 9]).unwrap(), 3);
        assert_eq!(solution(vec![9, 3, 9, 3, 9, 7, 9]).unwrap(), 7);
    }
}
