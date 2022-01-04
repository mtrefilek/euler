pub fn is_prime(num: i64, primes: &Vec<i64>) -> bool {
    for prime in primes {
        if num % prime == 0 {
            return false;
        }
        if *prime > (num as f64).sqrt() as i64 {
            return true;
        }
    }
    return true;
}

pub fn read_file(path:&str) -> String {
    let contents = std::fs::read_to_string(path).expect("Error");
    contents
}
