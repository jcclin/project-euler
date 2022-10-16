

fn is_parlindrome(i: i32) -> bool {

    let s = i.to_string();
    let rs: String = s.chars().rev().collect();
    s == rs
}


fn main() {

    let mut max_axb = 0;

    for a in 100..1000 {
        for b in 100..1000 {
            let axb = a * b;
            if is_parlindrome(axb) {
                if axb > max_axb {
                    max_axb = axb;
                    println!("Current largest {} x {} = {}", a, b, axb)
                }
            }
        }
    }
}
