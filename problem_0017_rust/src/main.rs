


fn verbalise(n: i32) -> String {

    if n >= 1000 { return String::from("one thousand"); }
    else if n >= 100 {
        if n % 100 == 0 { return format!("{} hundred", verbalise(n / 100)); }
        else { return format!("{} hundred and {}", verbalise(n / 100), verbalise(n % 100)); }
    }
    else if n >= 91 { return format!("ninety-{}", verbalise(n - 90)); }        
    else if n >= 90 { return String::from("ninety"); }
    else if n >= 81 { return format!("eighty-{}", verbalise(n - 80)); }        
    else if n >= 80 { return String::from("eighty"); }
    else if n >= 71 { return format!("seventy-{}", verbalise(n - 70)); }        
    else if n >= 70 { return String::from("seventy"); }
    else if n >= 61 { return format!("sixty-{}", verbalise(n - 60)); }        
    else if n >= 60 { return String::from("sixty"); }
    else if n >= 51 { return format!("fifty-{}", verbalise(n - 50)); }        
    else if n >= 50 { return String::from("fifty"); }
    else if n >= 41 { return format!("forty-{}", verbalise(n - 40)); }        
    else if n >= 40 { return String::from("forty"); }
    else if n >= 31 { return format!("thirty-{}", verbalise(n - 30)); }        
    else if n >= 30 { return String::from("thirty"); }
    else if n >= 21 { return format!("twenty-{}", verbalise(n - 20)); }        
    else if n >= 20 { return String::from("twenty"); }
    else if n >= 19 { return String::from("nineteen"); }
    else if n >= 18 { return String::from("eighteen"); }
    else if n >= 17 { return String::from("seventeen"); }
    else if n >= 16 { return String::from("sixteen"); }
    else if n >= 15 { return String::from("fifteen"); }
    else if n >= 14 { return String::from("fourteen"); }
    else if n >= 13 { return String::from("thirteen"); }
    else if n >= 12 { return String::from("twelve"); }
    else if n >= 11 { return String::from("eleven"); } 
    else if n >= 10 { return String::from("ten"); }
    else if n >= 9 { return String::from("nine"); }
    else if n >= 8 { return String::from("eight"); }
    else if n >= 7 { return String::from("seven"); }
    else if n >= 6 { return String::from("six"); }
    else if n >= 5 { return String::from("five"); }
    else if n >= 4 { return String::from("four"); }
    else if n >= 3 { return String::from("three"); }
    else if n >= 2 { return String::from("two"); }
    else if n >= 1 { return String::from("one"); }
    return String::from("");
}


fn main() {

    for n in 1..=1000 {
        println!("{} ----> {}", n, verbalise(n));
    }

    let mut total_alphabets: usize = 0;
    for n in 1..=1000 {
        let s = verbalise(n);
        let num_alphabets = s.chars()
            .filter(|c| c.is_alphabetic() )
            .count();
        total_alphabets += num_alphabets;
    }
    println!("Total letters {}", total_alphabets);

}
