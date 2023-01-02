use std::collections::HashMap;
use std::collections::HashSet;


const UPPER_BOUND: i32 = 28123;


fn calc_divisor_sum(n: i32) -> i32 {

    let u = if n % 2 == 0 { n / 2 + 1 } else { n };
    (1..u).filter(|x| n % x == 0).sum()
}



fn main() {

    let mut abundant_n_to_divisor_sum_map: HashMap<i32, i32> = HashMap::new();
    let mut abundant_n: Vec<i32> = Vec::new();
    for n in 2..=UPPER_BOUND {
        let ds = calc_divisor_sum(n);
        if ds > n {
            abundant_n_to_divisor_sum_map.insert(n, ds);
            abundant_n.push(n);
        }
    }
    println!("abundant_n = {:?}", abundant_n);

    let sum_two_abundant_n: HashSet<i32> = abundant_n
        .iter()
        .map(|nx|
            { 
                abundant_n
                    .iter()
                    .map(|ny| { nx + ny })
                    .collect::<Vec<i32>>()
            }
        )
        .flatten()
        .collect();
    println!("sum_two_abundant_n = {:?}", sum_two_abundant_n);

    let target_n: Vec<i32> = (1..=UPPER_BOUND)
        .filter(|n| !(sum_two_abundant_n.contains(n)))
        .collect();
    println!("target_abundant_n = {:?}", target_n);    

    let result: i32 = target_n.into_iter().sum();
    println!("result = {:}", result);    
}
