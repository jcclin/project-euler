use std::collections::HashSet;

const UPPER_BOUND: u32 = 1000000;


fn find_rev_digits(n: u32) -> Vec<u32> {
    let mut rev_digits: Vec<u32> = vec![];
    let mut v = n;
    while v > 0 {
        rev_digits.push(v % 10);
        v = v / 10;
    }
    return rev_digits;
}


fn find_n(rev_digits: &Vec<u32>) -> u32 {

    let mut m: u32 = 1;
    let mut acc: u32 = 0;
    for d in rev_digits.into_iter() {
        acc += (*d) * m;
        m *= 10;
    }
    return acc;
}


fn find_rotations(n: u32) -> Vec<u32> {

    let mut rev_digits = find_rev_digits(n);
    let mut rotated_ns: Vec<u32> = vec![];
    for _i in 0..rev_digits.len() {
        rev_digits.rotate_left(1);
        rotated_ns.push(find_n(&rev_digits));
    }
    return rotated_ns;
}


fn find_primes() -> Vec<u32> {

    let mut primes: Vec<u32> = vec![];
    for x in 2..=UPPER_BOUND {

        let mut is_divisible = false;
        let sn = f64::ceil(f64::sqrt(x as f64)) as u32;
        for p in primes.iter() {
            if *p >= (sn + 1) {
                break;
            }
            if x % (*p) == 0 {
                is_divisible = true;
                break;
            }
        }
        if !is_divisible {
            primes.push(x);
        }
    }
    return primes;
}



fn main() {
    
    let primes = find_primes();
    println!("All prime numbers under {}: {}", UPPER_BOUND, primes.len());
    println!("    {:?}", primes);
    let prime_set: HashSet<u32> = HashSet::from_iter(primes.iter().cloned());

    let mut circular_primes: Vec<u32> = vec![];

    for n in primes.iter()  {


        let rotated_ns = find_rotations(*n);
        let mut all_are_primes = true;
        for rn in rotated_ns.iter() {
            if !prime_set.contains(rn) {
                all_are_primes = false;
                break;
            }
        }
        if all_are_primes {
            circular_primes.push(*n);
            println!("{}....{:?}", *n, rotated_ns);
        }
    }

    println!("{:?} ({})", circular_primes, circular_primes.len());

}
