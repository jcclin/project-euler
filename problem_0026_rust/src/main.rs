


const N: i64= 1000;
const G: usize = 10000;

fn decimal_division(n: i64) -> Vec<i64> {

    let mut decimal_places: Vec<i64> = vec![];

    let mut d: i64 = 1;
    while d != 0 {
        if decimal_places.len() >= (3 * G) {
            break;
        }
        d *= 10;
        decimal_places.push(d / n);
        d = d % n;
    }
    return decimal_places;
}


fn recurring_length(decimal_places: &Vec<i64>) -> i64 {
    if decimal_places.len() < G {
        return 0;
    }
    for l in 1..G {
        let mut is_matched = true;
        for g in 1..G {
            if decimal_places[G + g] != decimal_places[G + l + g] {
                is_matched = false;
                break;
            }
        }
        if is_matched {
            return l as i64;
        }
    }
    return -1;
}




fn main() {
    
    let mut max_recurring_len: i64 = 1;
    let mut max_recurring_len_n: i64 = 2;
    let mut max_recurring_len_decimal_places: Vec<i64> = vec![];
    for n in 2..=N {
        let decimal_places = decimal_division(n);
        let recurring_len = recurring_length(&decimal_places);
        
        println!("1 / {} ...... {}", n, recurring_len);

        if recurring_len > max_recurring_len {
            max_recurring_len = recurring_len;
            max_recurring_len_n = n;
            max_recurring_len_decimal_places = decimal_places;
        }
    }
    println!("Max recurring length:");
    println!("    1 / {}", max_recurring_len_n);
    println!("    = 0.{}", max_recurring_len_decimal_places.iter().map(|i| i.to_string() ).collect::<Vec<String>>().join(""));
    println!("    recurring length = {}", max_recurring_len);
}
