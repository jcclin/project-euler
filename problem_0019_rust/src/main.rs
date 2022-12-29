use chrono::NaiveDate;
use chrono::Days;


fn main() {

    let d_start = NaiveDate::from_ymd_opt(1901, 1, 1).unwrap();
    let d_end = NaiveDate::from_ymd_opt(2001, 1, 1).unwrap();
    
    let mut i = 0;
    let mut num_sundays_start_of_months = 0;
    loop {
        let d_n = d_start.checked_add_days(Days::new(i)).unwrap();
        if d_n >= d_end {
            break;
        }
        if d_n.format("%d %A").to_string() == "01 Sunday" {
            println!("{}", d_n.format("%Y-%m-%d %A"));
            num_sundays_start_of_months += 1;
        }
        i += 1;
    }


    println!("Number of beginning of month Sundays = {}", num_sundays_start_of_months);
}
