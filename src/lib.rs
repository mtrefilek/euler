#[cfg(test)]
mod tests {
    use crate::{fibonacci, greatest_prime_factor, multiples};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn greatest_prime_factor_works() {
        assert_eq!(greatest_prime_factor(13195), 29);
        // 3 hour runtime, needs optimizations
        // println!("{}", greatest_prime_factor(600851475143));
    }

    #[test]
    fn multiples_works() {
        assert_eq!(multiples(10), 23);
        println!("{}", multiples(1000));
    }

    #[test]
    fn fib_works() {
        assert_eq!(fibonacci(90), 44);
        println!("{}", fibonacci(4000000));
    }
}

fn multiples(num: i64) -> i64 {
    let mut sum = 0;
    for i in 0 .. num {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

fn fibonacci(num: i64) -> i64 {
    let mut prev = 1;
    let mut cur = 2;
    let mut sum = 0;

    while cur <= num {
        if cur % 2 == 0 {
            sum += cur;
        }
        let tmp = prev;
        prev = cur;
        cur = cur + tmp;
    }
    sum
}

fn greatest_prime_factor(num: i64) -> i64 {
    let mut largest = 0;
    let mut primes:Vec<i64> = Vec::new();
    for i in 2 .. (num / 2) + 1 {
        if num % i == 0 {
            if is_prime(i, &primes) {
                primes.push(i);
                largest = i;
            }
        }
    }
    largest
}

fn is_prime(num: i64, primes: &Vec<i64>) -> bool {
    for prime in primes {
        if num % prime == 0 {
            return false;
        }
    }
    return true;
}
