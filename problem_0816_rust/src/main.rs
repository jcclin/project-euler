use std::fmt;


const SN_MOD: i64 = 50515093;
const K_MAX: usize = 2000000;
const N_MAX: usize = 2 * K_MAX + 1;


#[derive(Clone)]
struct P {
    x: i64,
    y: i64,
}

impl fmt::Display for P {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P(x={}, y={})", self.x, self.y)
    }
}


fn distance_squared(this: &P, that: &P) -> i64 {
    let diff_x = this.x - that.x;
    let diff_y = this.y - that.y;
    return diff_x * diff_x + diff_y * diff_y
}


fn main() {
    
    println!("K_MAX = {}, N_MAX = {}", K_MAX, N_MAX);

    println!("Builingd Sn series");
    let mut sn = vec![290797 as i64];
    let mut last_s = sn[0];
    for _n in 1..(N_MAX + 1) {
        last_s = (last_s * last_s) % SN_MOD;
        sn.push(last_s);
    }
    assert_eq!(sn.len(), N_MAX + 1);

    println!("Builingd Pn series");
    let mut pn: Vec<P> = Vec::new();
    for k in 1..(K_MAX + 1) {
        pn.push(P { x:sn[2 * k], y:sn[2 * k + 1] });
    }
    assert_eq!(pn.len(), K_MAX);    

    println!("Builingd Dn series"); 
    let mut dn = vec![2 * SN_MOD * SN_MOD, 2 * SN_MOD * SN_MOD, distance_squared(&pn[0], &pn[1])];
    for k in 3..(K_MAX + 1) {

        if k % 10000 == 0 { println!("        {}", k) }

        let mut min_d_squared = dn[k - 1];
        let p_last: &P = &pn[k - 1];
        for kf in 0..(k - 1) {
            let p_f: &P = &pn[kf];
            let d_f_squared = distance_squared(p_f, p_last);
            min_d_squared = i64::min(min_d_squared, d_f_squared);
        }
        dn.push(min_d_squared);
    }
    assert_eq!(dn.len(), K_MAX + 1);    



    println!("d({}) * d({}) = {}, d({}) = {}", K_MAX, K_MAX, dn[K_MAX], K_MAX, f64::sqrt(dn[K_MAX] as f64));
    
}
