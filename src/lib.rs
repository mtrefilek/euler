mod util;
extern crate num;

use std::collections::HashMap;
use std::num::FpCategory::Zero;
use num::bigint::BigInt;
use num::BigUint;

#[cfg(test)]
mod tests {
    use crate::{large_sum, longest_collatz, n_div_triangle, util};
    use crate::{fibonacci, greatest_prime_factor, largest_product_grid, multiples, n_prime, palindrome, smallest_multiple, special_pyth, sum_primes, sum_squares, thousand_digit};
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn greatest_prime_factor_works() {
        assert_eq!(greatest_prime_factor(13195), 29);
        // 3 hour runtime, needs optimizations
        // println!("{}", greatest_prime_factor(600851475143));
        println!("{}", greatest_prime_factor(600851475143));
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

    #[test]
    fn palindrome_works() {
        assert_eq!(palindrome(10, 100), 9009);
        println!("{}", palindrome(100, 1000));
    }

    #[test]
    fn smallest_works() {
        assert_eq!(smallest_multiple(10), 2520);
        println!("{}", smallest_multiple(20))
    }

    #[test]
    fn sum_squares_works() {
        assert_eq!(sum_squares(10), 2640);
        println!("{}", sum_squares(100));
    }

    #[test]
    fn n_prime_works() {
        assert_eq!(n_prime(6), 13);
        println!("{}", n_prime(10001));
    }

    #[test]
    fn thousand_digit_works() {
        let str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
        assert_eq!(thousand_digit(str.to_string(), 4), 5832);
        println!("{}", thousand_digit(str.to_string(), 13));
    }

    #[test]
    fn special_pyth_test() {
        println!("{}", special_pyth());
    }

    #[test]
    fn summation_primes_works() {
        assert_eq!(sum_primes(10), 17);
        println!("{}", sum_primes(2000000));
    }

    #[test]
    fn largest_product_grid_works() {
        println!("{}", largest_product_grid(util::read_file("data/11.in")))
    }

    #[test]
    fn n_div_triange_works() {
        assert_eq!(n_div_triangle(5), 28);
        println!("{}", n_div_triangle(500));
    }

    #[test]
    fn large_sum_works() {
        println!("{}", large_sum(util::read_file("data/13.in")))
    }

    #[test]
    fn longest_collatz_works() {
        println!("{}", longest_collatz());
    }
}

fn longest_collatz() -> i64 {
    let mut memoize:HashMap<i64, i64> = HashMap::new();
    let mut longest_len:(i64, i64) = (0, 0);

    for i in 1 .. 1000000_i64 {
        let mut cur = i;
        let mut count = 0;
        while cur != 1 {
            if memoize.contains_key(&cur) {
                let mem = memoize.entry(cur).or_default();
                count += *mem;
                cur = 1;
                count = count - 1;
            } else if cur % 2 == 0 {
                cur = cur/2;
            } else {
                cur = cur*3 + 1;
            }
            count += 1;
        }
        if count > longest_len.1 {
            longest_len = (i, count);
        }
        memoize.insert(i, count);
    }

    longest_len.0
}

fn large_sum(nums:String) -> i64 {
    let mut sum:BigUint = num::Zero::zero();
    for line in nums.lines() {
        let add = BigUint::parse_bytes(line.as_bytes(), 10).unwrap();
        sum = sum + add;
    }
    let mut str = sum.to_str_radix(10);
    let ret_str = str.as_mut_str()[..10].parse::<i64>().unwrap();
    ret_str
}

fn n_div_triangle(n:i64) -> i64 {
    let mut i:i64 = 1;
    loop {
        let num = (i*(i+1))/2;
        let mut div_count = 2;
        for x in 2..(num as f64).sqrt() as i64 + 1 {
            if num%x == 0 {
                div_count += 2;
            }
        }
        if div_count > n {
            return num;
        }
        i += 1;
    }
}

fn largest_product_grid(grid:String) -> i64 {
    let mut grid_vec:Vec<i64> = Vec::new();
    let mut len = 0;
    let mut rows = 0;

    for line in grid.lines() {
        len = 0;
        for num in line.split_whitespace() {
            grid_vec.push(num.parse::<i64>().unwrap());
            len += 1;
        }
        rows += 1;
    }

    let paths:[[(i64, i64); 3]; 8] = [
        [
            (0, -1), (0, -2), (0, -3)
        ],
        [
            (1, -1), (2, -2), (3, -3)
        ],
        [
            (1, 0), (2, 0), (3, 0)
        ],
        [
            (1, 1), (2, -2), (3, -3)
        ],
        [
            (0, 1), (0, 2), (0, 3)
        ],
        [
            (-1, 1), (-2, 2), (-3, 3)
        ],
        [
            (-1, 0), (-2, 0), (-3, 0)
        ],
        [
            (-1, -1), (-2, -2), (-3, -3)
        ]
    ];

    let mut largest:i64 = 0;
    for i in 0 .. grid_vec.len() {
        let x:i64 = (i % len) as i64;
        let y:i64 = (i / len) as i64;

        for path in paths.iter() {
            let mut product:i64 = grid_vec[i];
            for dir in path.iter() {
                let new_x = x + dir.0;
                let new_y = y + dir.1;

                if new_x < 0 || new_x >= len as i64 || new_y < 0 || new_y >= rows {
                    continue;
                }
                product *= grid_vec[(new_y*(len as i64) + new_x) as usize]
            }
            if product > largest {
                largest = product;
            }
        }
    }
    largest
}

fn sum_primes(max:i64) -> i64{
    let mut sum:i64 = 0;
    let mut primes:Vec<i64> = Vec::new();
    for i in 2..max {
        if util::is_prime(i, &primes) {
            sum += i;
            primes.push(i);
        }
    }
    sum
}

fn special_pyth() -> i32 {
    for a in 1 .. 1001_i32 {
        for b in 1 .. 1001 {
            for c in 1 .. 1001 {
                if a + b + c != 1000 {
                    continue
                }
                if a.pow(2) + b.pow(2) == c.pow(2) {
                    return a * b * c as i32;
                }
            }
        }
    }
    0
}

fn thousand_digit(num:String, window:usize) -> u64 {
    let mut nums:Vec<u32> = Vec::new();
    for c in num.chars() {
        let n = c.to_digit(10).unwrap();
        nums.push(n);
    }
    let mut largest:u64 = 0;

    for i in 0 .. nums.len() {
        if i + window as usize > nums.len() - 1 {
            return largest;
        }
        let mut prod:u64 = 1;
        for x in i .. i + window as usize {
            prod *= nums[x] as u64;
        }
        if largest < prod {
            largest = prod;
        }
    }
    0
}

fn n_prime(n: i32) -> i64 {
    let mut primes:Vec<i64> = Vec::new();
    primes.push(2);
    let mut i:i64 = 3;
    loop {
        if util::is_prime(i, &primes) {
            primes.push(i);
            if primes.len() == n as usize {
                return i;
            }
        }
        i+=2;
    }
}

fn sum_squares(max: i32) -> i32 {
    let mut sum_square = 0;
    let mut square_sum = 0;

    for i in 1 .. max+1 {
        sum_square += i.pow(2);
        square_sum += i;
    }
    square_sum.pow(2) - sum_square
}

fn smallest_multiple(max: i32) -> i32 {
    let mut num = 1;
    loop {
        let mut div = true;
        for i in 1 .. max + 1 {
            if num % i != 0 {
                div = false;
                break;
            }
        }
        if div {
            return num;
        }
        num += 1;
    }
}

fn palindrome(start: i64, end: i64) -> i64 {
    let mut largest = 0;
    for x in start .. end {
        for y in x .. end {
            let num:i64 = x * y;
            let len = num.to_string().len()/2;
            let num_str = num.to_string();
            let mut pal = true;
            for i in 0 .. len {
                if num_str.chars().nth(i).unwrap() != num_str.chars().nth(num_str.len() - 1 - i).unwrap() {
                    pal = false;
                }
            }
            if pal && largest < num {
                largest = num;
            }
        }
    }
    largest
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

fn greatest_prime_factor(mut num: i64) -> i64 {
    // A little help from
    let mut div = 2;
    while num > 1 {
        while num % div == 0 {
            num = num / div;
        }
        div += 1;
    }
    // Took 3 Hours to Find
    // let mut primes:Vec<i64> = Vec::new();
    // for i in 2 .. (num / 2) + 1 {
    //     if num % i == 0 {
    //         if is_prime(i, &primes) {
    //             primes.push(i);
    //             largest = i;
    //         }
    //     }
    // }
    div - 1
}
