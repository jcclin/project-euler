

fn to_rev_digits(n: u64) -> Vec<u64> {
    let mut rev_digits: Vec<u64> = vec![];
    let mut v = n;
    while v > 0 {
        rev_digits.push(v % 10);
        v = v / 10;
    }
    return rev_digits;
}


fn pan_digit_1(n1: u64) -> bool {

    let mut v = to_rev_digits(n1);
    if v.contains(&0) {
        return false;
    }
    let len_start = v.len();
    v.sort();
    v.dedup();
    let len_end = v.len();
    return len_start == len_end
}

fn pan_digit_2(n1: u64, n2: u64) -> bool {

    let mut v = [to_rev_digits(n1), to_rev_digits(n2)].concat();
    if v.contains(&0) {
        return false;
    }
    let len_start = v.len();
    v.sort();
    v.dedup();
    let len_end = v.len();
    return len_start == len_end
}


fn pan_digit_3(n1: u64, n2: u64, n3: u64) -> bool {

    let mut v = [to_rev_digits(n1), to_rev_digits(n2), to_rev_digits(n3)].concat();
    if v.contains(&0) {
        return false;
    }
    let len_start = v.len();
    v.sort();
    v.dedup();
    let len_end = v.len();
    return len_start == len_end && len_start == 9
}



fn main() {
    println!("Hello, world!");

    let mut products: Vec<u64>= vec![];
    for a in 1..10000_u64 {

        if !pan_digit_1(a) {
            continue;
        }

        for b in 1..a {

            if !pan_digit_1(b) {
                continue
            }

            if !pan_digit_2(a, b) {
                continue
            }

            let c = a * b;
            if !pan_digit_3(a, b, c) {
                continue
            }

            println!("a = {}, b = {}, c = {}", a, b, c);
            products.push(c);
        }
    }

    products.sort();
    products.dedup();
    println!("Sum of products = {}", products.iter().sum::<u64>());

}
