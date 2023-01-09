use std::collections::HashSet;


const UPPER_BOUND: u32 = 10000;

fn to_digits(n: u32) -> Vec<u32> {
    let mut ds: Vec<u32> = vec![];
    let mut v = n;
    while v > 0 {
        ds.push(v % 10);
        v = v / 10;
    }
    return ds.into_iter().rev().collect();
}


fn search_for_pandigits(n: u32) -> (bool, Vec<u32>, Vec<u32>) {

    let mut v: Vec<u32> = vec![];
    let mut pandigits: Vec<u32> = vec![];
    let mut s: HashSet<u32> = (1..10).into_iter().collect();
    let mut is_digit_repeated = false;

    for p in 1..UPPER_BOUND {
        v.push(p);
        for d in to_digits(n * p).iter() {
            if s.contains(d) {
                s.remove(d); 
                pandigits.push(*d);
            } else {
                is_digit_repeated = true;
            }
        }
        if s.len() == 0 {
            break;
        }
    }
    return (is_digit_repeated, v, pandigits);

}



fn main() {

    let mut max_n_pandigital: Vec<u32> = vec![];
    for n in 2..UPPER_BOUND {
        let t = search_for_pandigits(n);
        let is_pandigital = !t.0;
        let multiplier = t.1;
        let pandigits = t.2;
        if is_pandigital {     
            println!("n = {}, multiplier = {:?}, pandigits = {:?}", n, multiplier, pandigits);
            max_n_pandigital = pandigits;
        }
    }
    println!("Largest is {:?}", max_n_pandigital);
    
}
