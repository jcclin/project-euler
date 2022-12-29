
fn is_prime(x: i64, primes: &Vec<i64>) -> bool {

    for p in primes.into_iter() {
        if x % p == 0 {
            return false;
        }
    }
    return true;
} 


fn main() {

    let nth: usize = 10001;
    let mut primes: Vec<i64> = Vec::new();
    let mut x: i64 = 2;

    while primes.len() < nth {
        if is_prime(x, &primes) {
            primes.push(x);
        }
        x += 1;
    }
    println!("The {}th prime is {}", nth, primes.last().unwrap());
}
