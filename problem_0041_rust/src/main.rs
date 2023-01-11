const UPPER_BOUND: u64 = 8000000;


fn find_primes() -> Vec<u64> {

    let mut primes: Vec<u64> = vec![];
    for n in 2..UPPER_BOUND {
        if n % 1000000 == 0 {
            println!("{}", n);
        }
        let sn = f64::ceil(f64::sqrt(n as f64)) as u64;
        let mut is_divisible = false;
        for p in primes.iter() {
            if *p > sn {
                break;
            }
            if n % *p == 0 {
                is_divisible = true;
                break;
            }
        }
        if !is_divisible {
            primes.push(n);
        }
    }
    return primes;
}


fn pan_digital(n: u64) -> usize {

    let mut rev_digits: Vec<u64> = vec![];
    let mut v = n;
    while v > 0 {
        let d = v % 10;
        if d == 0 {
            return 0;
        } 
        if rev_digits.contains(&d) {
            return 0;
        }
        rev_digits.push(d);
        v = v / 10;
    }
    let l = rev_digits.len() as u64;
    for x in 1..=l {
        if !rev_digits.contains(&x) {
            return 0;
        }
    }
    return l as usize;
}


fn main() {

    let primes = find_primes();
    for p in primes {
        let l = pan_digital(p);
        if l > 0 {
            println!("{} -> {}", p, l);
        }
    }
}
