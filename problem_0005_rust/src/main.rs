use counter::Counter;


const PRIMES: [i32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];


fn factorise(n: i32) -> Vec<i32> {

    let mut r = n;
    let mut v = Vec::new();

    while r > 1 {
        for p in PRIMES {
            if p > r {
                break;
            }
            if r % p == 0 {
                v.push(p);
                r = r / p;
                break;
            }
        }
    }

    v
}


fn main() {
    

    let mut x = 0;
    loop {
        x += 1;

        let mut is_divisible = true;
        for i in 1..11 {
            if x % i != 0 {
                is_divisible = false;
            }
        }

        if is_divisible {
            println!("Answer is {}", x);
            break;
        } else {
            println!("{}", x);
        }
    }

    let f12 = factorise(12);
    let c12 = f12.into_iter().collect::<Counter<_>>();

    println!("Factorise 12 = {:?}", f12);
    println!("Counter Factorise 12 = {:?}", c12);
}
