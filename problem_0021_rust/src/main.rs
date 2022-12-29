use std::collections::HashMap;


const N: i32 = 10000;


fn find_divisors(n: i32) -> Vec<i32> {
    assert!(n > 0);

    let mut divisors: Vec<i32> = vec![];
    for x in 1..n {
        if n % x == 0 {
            divisors.push(x);
        }
    }
    divisors
}


fn main() {

    let mut n_to_divisor_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut n_to_dn_map: HashMap<i32, i32> = HashMap::new();
    for n in 1..=N {
        let divisors = find_divisors(n);
        n_to_dn_map.insert(n, divisors.iter().sum());
        n_to_divisor_map.insert(n, divisors);
    }

    let mut pairs: Vec<(i32, i32)> = vec![];
    let mut sums: i32 = 0;
    for n in 1..=N {
        let dn1 = n_to_dn_map.get(&n);
        if dn1.is_none() {
            continue;
        }
        if *(dn1.unwrap()) <= n {
            continue;
        }
        let dn2 = n_to_dn_map.get(dn1.unwrap());
        if dn2.is_none() {
            continue;
        }
        if *(dn2.unwrap()) != n {
            continue;
        }
        pairs.push((n, *(dn1.unwrap())));
        sums += n;
        sums += *(dn1.unwrap())
    }

    println!("Amicable pairs = {:?}, sums= {}", pairs, sums);
}
