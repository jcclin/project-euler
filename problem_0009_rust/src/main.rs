static SUM_ABC: i64 = 1000;


fn main() {


    let mut found: bool = false;
    let mut abc = (0, 0, 0);

    for a in 1..SUM_ABC {
        for b in (a + 1)..SUM_ABC {
            let c = SUM_ABC - a - b;
            if c <= b {
                break;
            }
            if c * c == (a * a+ b * b) {
                found = true;
                abc = (a, b, c);
                break;
            }
        }
        if found {
            break;
        }
    }

    println!("a = {}, b = {}, c = {}, a x b x c = {}", abc.0, abc.1, abc.2, (abc.0 * abc.1 * abc.2));

}
