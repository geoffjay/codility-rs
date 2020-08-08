#![feature(test)]

extern crate test;

pub fn solution() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solution(), 42);
    }
}
