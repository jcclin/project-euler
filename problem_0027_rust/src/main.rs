const NUM_PRIMES: usize = 2000;


fn find_primes(num_primes: usize) -> Vec<i32> {

    let mut primes: Vec<i32> = vec![];
    let mut n: i32 = 1;
    while primes.len() <= num_primes {
        n += 1;
        let mut is_n_divisible = false;
        for p in &primes {
            if n % p == 0 {
                is_n_divisible = true;
                break;
            }
        }
        if !is_n_divisible {
            primes.push(n);
        }
    }
    return primes;
}


fn find_num_consecutive_primes(a: i32, b: i32, primes: &Vec<i32>) -> usize {

    let mut n: usize = 0;
    loop {
        let n_: i32 = n as i32;
        let p = n_ * n_ + a * n_ + b;
        if !primes.contains(&p) {
            break;
        }
        n += 1;
    }
    return n;
}



fn main() {
    
    let primes = find_primes(NUM_PRIMES);
    println!("Primes are = {:?}", primes);

    let mut max_num_n: usize = 0;
    let mut max_num_n_a: i32 = 0;
    let mut max_num_n_b: i32 = 0;
    for a in -999..=999 {
        for b in -999..=999 {
            let num_n = find_num_consecutive_primes(a, b, &primes);
            println!("......a = {}, b = {}, num_consecutive_n = {}", a, b, num_n);
            if num_n > max_num_n {
                max_num_n = num_n;
                max_num_n_a = a;
                max_num_n_b = b;
            }
        }
    }
    println!("Max consecutive.... a = {}, b = {}, num_consecutive_n = {}", max_num_n_a, max_num_n_b, max_num_n);

}
