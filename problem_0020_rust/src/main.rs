
const N: i32 = 100;


fn main() {

    let mut rev_digits = vec![1];

    for f in 1..=N {

        for d in rev_digits.iter_mut() {
            *d *= f;
        }

        
        let mut c = 0;
        for d in rev_digits.iter_mut() {
            let s = *d + c;
            *d = s % 10;
            c = s / 10;
        }
        while c > 0 {
            let d = c % 10;
            rev_digits.push(d);
            c = c / 10;
        }
    }

    let result: String = rev_digits
        .iter()
        .rev()
        .map(|d| d.to_string()).collect::<Vec<String>>()
        .join("");
    let digit_sum: i32 = rev_digits.iter().sum();
    println!("Result = {}, DigitSum = {}", result, digit_sum);
}
