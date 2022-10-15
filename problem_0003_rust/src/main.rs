fn main() {

    let f = 600851475143 as i64;
    let g = f as f64;
    let h = g.sqrt();
    let i = h.ceil() as i64;
    println!("sqrt({}) -> {} (ceil to {})", f, h, i);

    let mut primes = Vec::<i64>::new();

    println!("Find all the prime numbers from 0 to {}", i);
    for x in 2..(i + 1) {

        if x % 1000000 == 0 {
            println!("{}", x);
        }

        let mut is_divisible = false;
        for p in primes.iter_mut() {
            if *p != x && x % *p == 0 {
                is_divisible = true;
                break;
            }
        }
        if is_divisible {
             continue;
        } else {
            primes.push(x);
        }
    }
    
    //println!("{:?}", primes);

    for x in primes.into_iter().rev() {
        if f % x == 0 {
            println!("Answer is {}", x);
            break;
        }
    }
}
