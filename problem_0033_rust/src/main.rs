

fn inexperience_simplification(n: u32, d: u32) -> [u32; 2] {

    let n1 = n / 10;
    let n0 = n % 10;
    let d1 = d / 10;
    let d0 = d % 10;

    if n1 == d1 {
        return [n0, d0];
    }

    if n1 == d0 {
        return [n0, d1];
    }

    if n0 == d0 {
        return [n1, d1];
    }

    if n0 == d1 {
        return [n1, d0];
    }

    return [0, 0];
}


fn main() {

    let mut answers: Vec<[u32; 4]> = vec![];
    for d in 10..100 {
        for n in 10..d {
            let snsd = inexperience_simplification(n, d);
            let sn = snsd[0];
            let sd = snsd[1];
            if n % 10 == 0 && d % 10 == 0 {
                /* trivia */
                continue;
            }
            if sn == 0 && sd == 0 {
                continue
            }
            if sn * d == sd * n {
                answers.push([n, d, sn, sd]);
                println!("{} / {} == {} / {}", n, d, sn, sd);
            }
        }
    }
}
