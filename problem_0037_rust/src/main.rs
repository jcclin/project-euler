use std::collections::HashSet;

const UPPER_BOUND: u32 = 1000000;


fn find_primes() -> Vec<u32> {
    let mut primes: Vec<u32> = vec![];
    for x in 2..UPPER_BOUND {
        let mut is_divisible: bool = false;
        let u = f64::ceil(f64::sqrt(x as f64)) as u32;
        for p in primes.iter() {
            if *p > u {
                break;
            }
            if x % *p == 0 {
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


fn find_chop_from_right_candidates(n: u32) -> Vec<u32> {

    let mut candidates: Vec<u32> = vec![];
    let mut v = n;
    while v > 0 {
        candidates.push(v);
        v = v / 10;
    }
    return candidates;
}


fn find_chop_from_left_candidates(n: u32) -> Vec<u32> {

    let mut candidates: Vec<u32> = vec![];
    let v = n;
    let mut x = 1;
    while v > x {
        x *= 10;
        candidates.push(v % x);

    }
    return candidates;
}



fn main() {
    
    let primes = find_primes();
    let prime_set: HashSet<u32> = primes.iter().cloned().collect();

    println!("{:?}", primes);
    println!("{:?}", prime_set);

    let mut answer: u32 = 0;

    for n in 10..UPPER_BOUND {

        let mut all_candidates_are_primes = true;
        let chop_from_left_candidates = find_chop_from_left_candidates(n);
        let chop_from_right_candidates: Vec<u32> = find_chop_from_right_candidates(n);

        if all_candidates_are_primes {
            for lc in chop_from_left_candidates.iter() {
                if !prime_set.contains(lc) {
                    all_candidates_are_primes = false;
                    break;
                }
            }
        }

        if all_candidates_are_primes {
            for rc in chop_from_right_candidates.iter() {
                if !prime_set.contains(rc) {
                    all_candidates_are_primes = false;
                    break;
                }
            }
        }

        
        if all_candidates_are_primes {
            println!("{} ........ {:?} {:?}", n, chop_from_left_candidates, chop_from_right_candidates);
            answer += n;
        }
    }

    println!("Answer = {}", answer);
}
