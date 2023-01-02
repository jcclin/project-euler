

fn add_two_rev_digits(v_n_prev: &Vec<u8>, v_n: &Vec<u8>) -> Vec<u8> {

    let mut v_n_next = v_n.clone();
    for (i, d) in v_n_prev.into_iter().enumerate() {
        v_n_next[i] += *d;
    }

    let mut carry = 0;
    for d in v_n_next.iter_mut() {
        *d += carry;
        carry = 0;
        if *d >= 10 {
            *d -= 10;
            carry += 1;
        }
    }
    if carry > 0 {
        v_n_next.push(carry);
    }
    return v_n_next;
}



fn main() {
    
    let mut fib_n_prev = 1;
    let mut fib_n_prev_rev_digits: Vec<u8> = vec![1];
    println!("n = {}", fib_n_prev);
    println!("        Fn = {}", fib_n_prev_rev_digits.iter().rev().map(|x| x.to_string() ).collect::<Vec<String>>().join(""));

    let mut fib_n: i32 = 2;
    let mut fib_n_rev_digits: Vec<u8> = vec![1];
    println!("n = {}", fib_n);
    println!("        Fn = {}", fib_n_rev_digits.iter().rev().map(|x| x.to_string() ).collect::<Vec<String>>().join(""));

    loop {
        let fib_n_next: i32 = fib_n + 1;
        let fib_n_next_rev_digits: Vec<u8> = add_two_rev_digits(&fib_n_prev_rev_digits, &fib_n_rev_digits);

        fib_n_prev = fib_n;
        fib_n_prev_rev_digits = fib_n_rev_digits;

        fib_n = fib_n_next;
        fib_n_rev_digits = fib_n_next_rev_digits;

        println!("n = {}", fib_n);
        println!("        Fn = {}", fib_n_rev_digits.iter().rev().map(|x| x.to_string() ).collect::<Vec<String>>().join(""));
        if fib_n_rev_digits.len() >= 1000 {
            break;
        }
    }

}
