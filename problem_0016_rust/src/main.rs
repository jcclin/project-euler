
const POWER: i32 = 1000;

fn main() {

    let mut rev_digits: Vec<i32> = vec![1];

    for _ in 0..POWER {
        for n in rev_digits.iter_mut() {
            *n = *n * 2;
        }

        let mut carry = 0;
        for d in 0..rev_digits.len() {
            let mut n = rev_digits[d];

            n = n + carry;
            carry = 0;

            if n >= 10 {
                n = n - 10;
                carry = 1;
            } 
            rev_digits[d] = n;
        }
        if carry > 0 {
            rev_digits.push(carry);
        }
    }

    let s : i32 = rev_digits.iter().sum();
    println!("2 to power of {} is {:}", POWER, s);

}
