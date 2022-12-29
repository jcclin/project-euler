
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


fn num_factor_combs(factors: &Vec<u64>) -> u64 {

    let f_cntr = factors.iter().collect::<Counter<_>>();
    let f_cnts = f_cntr.into_map().into_values();

    let mut nc: u64 = 1;
    for cnt in f_cnts {
        nc = nc * ((cnt  as u64) + 1);
    }
    return nc;
} 



fn main() {

    let mut primes: Vec<u64> = vec![2 as u64];

    let mut i = 0;
    let mut s = 0;
    let mut triangle_numbers = vec![];
    let mut factors: Vec<u64> = vec![];
    let mut nfc: u64 = num_factor_combs(&factors);

    while nfc < 500 {
        i += 1;
        s += i;
        triangle_numbers.push(s);

        factors = factorise(s, &mut primes);
        nfc = num_factor_combs(&factors);

        // println!("{:?}", primes);
        println!("{}: {}: {}", i, s, nfc);
    }
    println!("{}: {}: {} : {:?}", i, s, nfc, factors);
}
