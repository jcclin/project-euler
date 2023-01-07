const UPPER_BOUND: u32 = 1000000;

fn _is_based_N_parlindromic(n: u32, N: u32)-> bool {
    let mut rd: Vec<u32> = vec![];
    let mut v = n;
    let d0 = v % N;
    if d0 == 0 {
        return false;
    }
    rd.push(d0);
    v = v / N;
    while v > 0 {
        rd.push(v % N);
        v = v / N;
    }
    let ds = rd.iter().rev();
    let mut is_any_mismatched = false;
    for (d, r) in ds.zip(rd.iter()) {
        if *d != *r {
            is_any_mismatched = true;
            break;
        }
    }
    return !is_any_mismatched;
}


fn is_based_2_parlindromic(n: u32) -> bool {
    return _is_based_N_parlindromic(n, 2_u32);
}

fn is_based_10_parlindromic(n: u32) -> bool {
    return _is_based_N_parlindromic(n, 10_u32);
}


fn main() {
    println!("Hello, world!");
    let mut n_sum: u32 = 0;
    for n in 1..UPPER_BOUND {
        
        if !is_based_2_parlindromic(n) {
            continue
        }
        if !is_based_10_parlindromic(n) {
            continue
        }

        n_sum += n;
        println!("{} = {}", n, format!("{n:b}"));
    }

    println!("n_sum = {}", n_sum);
}
