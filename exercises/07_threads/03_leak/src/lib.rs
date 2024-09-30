// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let static_v = v.leak();
    let (ref_source_data_half1, ref_source_data_half2) =
        static_v.split_at(static_v.len() / 2);
    let thread_half1: thread::JoinHandle<i32> = thread::spawn(
        move || { ref_source_data_half1.iter().sum() });
    let thread_half2: thread::JoinHandle<i32> = thread::spawn(
        move || { ref_source_data_half2.iter().sum() });
    thread_half1.join().unwrap() + thread_half2.join().unwrap()
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
