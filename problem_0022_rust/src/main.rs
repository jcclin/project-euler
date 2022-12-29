use std::fmt;
use reqwest;


const TARGET_FILE_URL: &str = "https://projecteuler.net/project/resources/p022_names.txt";


struct Name {
    name: String
}


impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self { name: String::from(value) }
    }
}


impl fmt::Debug for Name {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Name")
            .field("name", &self.name)
            .field("score", &(self.score()))
            .finish()
    }
}


impl Name {
    fn score(&self) -> u32 {
        self.name
            .chars()
            .map(|c| { u32::from(c) - u32::from('A') + 1 })
            .sum()
    }
}



fn main() {
    let target_file_content = reqwest::blocking::get(TARGET_FILE_URL).unwrap().text().unwrap();
    let mut names: Vec<Name> = target_file_content
        .split(",")
        .map(|s| Name::from(s.trim_matches('"')) )
        .collect();
    names.sort_by(|a, b| { a.name.cmp(&(b.name)) });

    println!("GET {} = {}", TARGET_FILE_URL, target_file_content);
    for name in &names {
        println!("........ {:?}", name)
    }

    let mut sum_total_product: i32 = 0;
    for (idx, name) in (&names).iter().enumerate() {
        sum_total_product += (idx as i32 + 1) * name.score() as i32;
        println!("........{}: {:?} : {}", idx + 1, name, sum_total_product);

    }
    println!("sum_total_product = {}", sum_total_product);

}
