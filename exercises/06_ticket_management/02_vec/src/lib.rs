// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
    let n_usize: usize = n.try_into().unwrap();
    let mut fib_vec = Vec::<u32>::with_capacity(n_usize + 1);
    for i in 0..=n_usize {
        match i {
            0 => fib_vec.push(0),
            1 => fib_vec.push(1),
            _ => fib_vec.push(fib_vec[i-2] + fib_vec[i-1]),
        }
    }
    fib_vec[n_usize]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirthieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
