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

const nums:[&str; 19] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen"
];
const tens:[&str; 8] = [
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety"
];
const order:[&str; 2] = [
    "hundred",
    "thousand"
];

pub fn num_counter() -> usize {
    let mut sum:usize = 0;
    for n in nums.iter() {
        sum += n.len();
        println!("{}", sum);
    }
    for ten in tens.iter() {
        sum += ten.len();
        for n in nums[0..9].iter() {
            sum += ten.len() + n.len();
        }
    }
    let ninety_nine_sum = sum;
    for n in nums[0..9].iter() {
        sum += n.len() + order[0].len();
        sum += ninety_nine_sum + 99 * (n.len() + order[0].len() + "and".len());
    }
    sum += nums[0].len() + order[1].len();
    sum
}
