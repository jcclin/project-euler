static MAX_VALUE: u64 = 2000000;

fn main() {

    let mut primes: Vec<u64> = Vec::new();


    for i in 2..MAX_VALUE {

        if i % 10000 == 0 {
            println!("    finding primes up to {}", i);
        }

        let mut is_divisible_by_p: bool = false;
        for p in primes.iter() {
            if i % *p == 0 {
                is_divisible_by_p = true;
                break;
            }
        }
        if !is_divisible_by_p {
            primes.push(i);
        }
    }

    //println!("Prime numbers below {} are: {:?}", MAX_VALUE, primes);
    println!("The sun of these primes are: {:}", primes.iter().sum::<u64>());
}
