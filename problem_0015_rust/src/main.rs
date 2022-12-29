use counter::Counter;


fn sqrt_ceil(n: u64) -> u64 {
    f64::ceil(f64::sqrt(n as f64)) as u64
}

fn is_prime(n: u64, primes: &Vec<u64>) -> bool {
    let sn = sqrt_ceil(n);
    for p in primes {
        if *p > sn {
            break;
        }
        if n % *p == 0 {
            return  false;
        }
    }
    return true;
}


fn add_primes_until_above(primes: &mut Vec<u64>, n: u64) {

    let p_max: u64 = *(primes.last().unwrap());
    if p_max > n {
        return;
    }

    for x in (p_max + 1)..=n {
        if is_prime(x, &primes) {
            primes.push(x)
        }
    }

    let mut o: u64 = n;
    loop {
        o += 1;
        if is_prime(o, &primes) {
            primes.push(o);
            return;
        }
    }
}


fn _factorise(n: u64, primes: &Vec<u64>) -> Vec<u64> {                          

    let mut p = None;
    for q in primes {
        if *q > n {
            break;
        }
        if n % *q == 0 {
            p = Some(*q);
            break;
        }
    }

    let mut factors = vec![];
    if p.is_some() {
        let pv = p.unwrap();
        let r = n / pv;
        factors.push(pv);
        factors.extend(_factorise(r, primes).iter());
    }
    return factors;
}


fn factorise(n: u64, primes: &mut Vec<u64>) -> Vec<u64> {

    add_primes_until_above(primes, n);

    return _factorise(n,  primes);
}


fn combinations(n: u64, r: u64, primes: &mut Vec<u64>) ->u64 {

    assert!(n > r);
    add_primes_until_above(primes, n);

    let numerator_factors: Vec<u64> = ((n - r + 1)..=n)
        .map(|x| factorise(x, primes))
        .fold(vec![], |mut acc, v| { acc.extend(v.iter()); acc });
    let denominator_factors_counter = numerator_factors.iter().collect::<Counter<_>>();
    let denominator_factors: Vec<u64> = (1..=r)
        .map(|x| factorise(x, primes))
        .fold(vec![], |mut acc, v| { acc.extend(v.iter()); acc });
    let denominator_factor_counter = denominator_factors.iter().collect::<Counter<_>>();

    let mut result_factors: Vec<u64>= vec![];
    for p in primes.iter() {
        if *p > n {
            break;
        }
        let numerator_factor_freq = *(denominator_factors_counter.get(p).unwrap_or(&0));
        let denominator_factor_freq = *(denominator_factor_counter.get(p).unwrap_or(&0));
        assert!(numerator_factor_freq >= denominator_factor_freq);
        for _ in 0..(numerator_factor_freq - denominator_factor_freq) {
            result_factors.push(*p);
        }
    }
    let mut result = 1;
    for f in result_factors {
        result = result * f;
    }
    return result;
}



fn main() {

    let mut primes: Vec<u64> = vec![2];
    let cnt: u64 = combinations(40, 20, &mut primes);
    println!("Count to {}", cnt);
}
