fn main() {

    for n in 1..=100 {
        if n % 15 == 0 {
            println!("{} is divisible by 3 and 5", n);
        } else if n % 3 == 0 {
            println!("{} is divisible by 3", n);
        } else if n % 5 == 0 {
            println!("{} is divisible by 5", n);
        } else {
            println!("{}", n);
        }
    }
}
