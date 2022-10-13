

fn main() {
    println!("Hello, world!");

    let mut vec = Vec::new();
    let limit = 4000000;
    vec.push(1);
    vec.push(2);


    let mut n = vec.len();
    let mut v = 1 + 2;
    while v < limit {
        vec.push(v);
        n = n + 1;
        v = vec[n - 2] + vec[n - 1];        
    }
    println!("{:?}", vec);

    let mut s = 0;
    for v in vec.iter() { 
        if v % 2 == 0 {
            s = s + v;
        }
    }
    println!("{}", s);

}
