#![feature(test)]

extern crate test;

fn solution(n: i32) -> i32 {
    let mut gap = 0;
    let mut largest = 0;
    let mut num = n;
    let mut init = false;
    loop {
        // increase gap size when number is zero, and count has been initialized
        if num & 1 == 0 {
            if init {
                gap += 1;
            }
        } else {
            if gap > largest {
                largest = gap;
            }
            gap = 0;
            // start the count once a 1 has been seen
            init = true;
        }
        num = num >> 1;
        if num == 0 {
            break;
        }
    }
    return largest;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(solution(9), 2);
        assert_eq!(solution(529), 4);
        assert_eq!(solution(20), 1);
        assert_eq!(solution(15), 0);
        assert_eq!(solution(1041), 5);
        assert_eq!(solution(32), 0);
        assert_eq!(solution(0b0000), 0);
        assert_eq!(solution(0b10000), 0);
        assert_eq!(solution(0b10000000), 0);
        assert_eq!(solution(0b10000001), 6);
    }

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        b.iter(|| solution(204913));
    }

    #[bench]
    fn bench_solution_many(b: &mut Bencher) {
        b.iter(|| (0..10000).map(solution).collect::<Vec<i32>>())
    }
}
