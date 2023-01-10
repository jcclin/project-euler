use std::collections::VecDeque;


#[derive(Default,Debug)]
struct GenDigit {
    cur_num: u32,
    digits: VecDeque<u32>,
}


impl GenDigit {
    fn get_next(&mut self) -> u32 {
        if self.digits.len() == 0 {
            self.cur_num += 1;
            let mut v = self.cur_num;
            while v > 0 {
                self.digits.push_front(v % 10);
                v = v / 10;
            }
        }
        return self.digits.pop_front().unwrap();
    }
}



fn main() {

    let mut g = GenDigit::default();
    
    let mut i: usize = 0;
    let mut di: VecDeque<usize> = VecDeque::from([1, 10, 100, 1000, 10000, 100000, 1000000]);
    let mut dig: Vec<(usize, u32)> = vec![];
    let mut answer: u32 = 1;
    while di.len() > 0 {
        i += 1;
        let dg = g.get_next();
        if di.front() == Some(&i) {
            di.pop_front();
            println!("...... ({}, {})", i, dg);
            dig.push((i, dg));
            answer *= dg;
        }
    }
    println!("{:?}", dig);
    println!("{}", answer);
    
}
