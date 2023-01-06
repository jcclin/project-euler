use std::collections::HashMap;

const UPPER_BOUND: u64 = 1000000000;


fn init_factorial_map() -> HashMap<u64, u64> {
    let mut factorials: Vec<u64> = vec![1];
    
    for n in 1..10 {
        let f = factorials.last().unwrap();
        factorials.push(f * n);
    }

    let mut factorial_map: HashMap<u64, u64> = HashMap::from([]);
    for n in 0..10 {
        factorial_map.insert(n, factorials[n as usize]);
    }
    return factorial_map;
}


fn calc_rev_digits(n: u64) -> Vec<u64> {

    let mut rev_digits: Vec<u64> = vec![];
    let mut v: u64 = n;
    while v > 0 {
        rev_digits.push(v % 10);
        v = v / 10;
    }
    return rev_digits
}


fn calc_sum_digit_factorials(rd: &Vec<u64>, factorial_map: &HashMap<u64, u64>) -> u64 {

    let mut s: u64 = 0;
    for d in rd.into_iter() {
        s += factorial_map.get(d).unwrap();
    }
    return s
}


fn main() {

    let factorial_map = init_factorial_map();
    let mut total_sum: u64 = 0;
    for n in 10..UPPER_BOUND {
        if n % 100000 == 0 {
            print!("......... {}", n)
        }
        let rd: Vec<u64> = calc_rev_digits(n);
        let sf = calc_sum_digit_factorials(&rd, &factorial_map);
        if sf == n {
            total_sum += n;
            println!("n = {}, sf = {} ........ total so far = {}", n, sf, total_sum);

    }   }
}
