use itertools::Itertools;

const N: usize = 10;


fn main() {

    let perms = (0..N).permutations(N);
    for (i, t) in perms.enumerate() {

        let nth = i + 1;
        if nth > 1000000 {
            break;
        }        

        if nth % 10000 == 0 {
            println!("nth = {}, t = {:?}", nth, t);
        }
    }
}
