// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread::{scope, ScopedJoinHandle};

pub fn sum(v: Vec<i32>) -> i32 {
    let (ref_left, ref_right) = v.split_at(v.len() / 2);
    scope(|scope| {
        let sum_left: ScopedJoinHandle<i32> = scope.spawn(|| { ref_left.iter().sum() });
        let sum_right: ScopedJoinHandle<i32> = scope.spawn(|| { ref_right.iter().sum() });
        sum_left.join().unwrap() + sum_right.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
