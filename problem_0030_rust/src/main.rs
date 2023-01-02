const POWER: u32 = 5;
const SEARCH_BOUND: u32 = 9_u32.pow(POWER) * (POWER + 1);



fn sum_of_powers_of_digits(n: u32) -> u32 {
    
    let mut v: u32 = n;
    let mut s: u32 = 0;
    while v > 0 {
        s += (v % 10).pow(POWER);
        v = v / 10;
    }
    return s;
}



fn main() {
    
    let mut pds_sum: u32 = 0;
    for n in 2..SEARCH_BOUND {
        if n % 1000 == 0 {
            println!("{} / {}", n, SEARCH_BOUND);
        }
        let pds = sum_of_powers_of_digits(n);
        if pds == n {
            pds_sum += pds;
            println!("{} ................ {}", n, pds)
        }
    }
    println!("Answer = {}", pds_sum);
}
