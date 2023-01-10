use std::collections::HashMap;

const MAX_P: u32 = 1000;



fn main() {
    
    let mut pc: HashMap<u32, Vec<(u32, u32, u32)>> = HashMap::new();

    for a in 1..MAX_P {
        for b in a..MAX_P {
            for c in b..MAX_P {
                if (a * a + b * b) == (c * c) {
                    println!("({}, {}, {})", a, b, c);
                    let p: u32 = a + b + c;
                    if pc.contains_key(&p) {
                        pc.get_mut(&p)
                        .unwrap()
                        .push((a, b, c));
                    } else {
                        pc.insert(p, vec![(a, b, c)]);
                    }
                }
            } 
        }
    }

    let mut max_v_len: usize = 0;
    let mut max_v_len_v: Vec<(u32, u32, u32)> = vec![];
    let mut max_v_len_p: u32 = 0; 
    for (p, v) in pc {
        println!("{:}, {:?}", p, v);
        let v_len = v.len();
        if p < MAX_P && v_len > max_v_len {
            max_v_len = v_len;
            max_v_len_v = v;
            max_v_len_p = p;
        }
    }
    println!("max_v_len_p = {}, max_v_len = {}, max_v_len_v = {:?}", max_v_len_p, max_v_len, max_v_len_v);
}
