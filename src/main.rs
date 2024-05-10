fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 20;
    println!("fib(n) = {}", fib(n));
}

#[cfg(test)]
mod fibonacci {
    use super::fib;

    #[test]
    fn test_fib_n_equal_one() {
        assert_eq!(fib(1), 1);
    }
    
    #[test]
    fn test_fib_n_equal_zero() {
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(20), 6765);
    }
}
