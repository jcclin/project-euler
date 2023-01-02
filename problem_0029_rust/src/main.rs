use std::fmt;
use std::ops::{Add,Mul};


const N: usize = 100;

#[derive(PartialEq)]
struct VecInt {
    rev_digits: Vec<i32>
}


impl Clone for VecInt {
    fn clone(&self) -> Self {
        Self { rev_digits: self.rev_digits.clone() }
    }
}


impl VecInt {
    fn legalise(&mut self) {

        let mut carry = 0;
        for d in self.rev_digits.iter_mut() {
            *d = *d + carry;
            carry = *d / 10;
            *d = *d % 10;

        }
        while carry > 0 {
            self.rev_digits.push(carry % 10);
            carry = carry / 10;
        }
    }

    fn new(n: i32) -> VecInt {
        let mut new_self = Self { rev_digits: vec![n] };
        new_self.legalise();
        return new_self;
    }

    fn pow(&self, n: i32) -> VecInt {
        let result: VecInt = (0..n)
            .map(|_n| self.clone() )
            .fold(VecInt::new(1), |acc, other| acc * other);
        return result;
    }
}


impl fmt::Debug for VecInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_digits = self.rev_digits.iter()
            .rev()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("");
        write!(f, "VecInt[{:?}]", str_digits)
    }
}


impl fmt::Display for VecInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_digits = self.rev_digits.iter()
            .rev()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}", str_digits)
    }
}


impl Add<&VecInt> for VecInt {
    type Output = Self;

    fn add(self, other: &VecInt) -> Self {
        let mut rev_digits = self.rev_digits.clone();
        for (i, d) in other.rev_digits.iter().enumerate() {
            if i < rev_digits.len() {
                rev_digits[i] += *d;
            } else {
                rev_digits.push(*d);
            }
        }
        let mut added = Self { rev_digits: rev_digits };
        added.legalise();
        return added;
    }
}


impl Add<VecInt> for VecInt {
    type Output = Self;

    fn add(self, other: VecInt) -> Self {
        self.add(&other)
    }
}


impl Add<&i32> for VecInt {
    type Output = Self;

    fn add(self, other: &i32) -> Self {
        return self.add(*other);
    }
}

impl Add<i32> for VecInt {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        let other_vec_int = Self::new(other);
        return self.add(&other_vec_int);
    }
}


impl Mul<&VecInt> for VecInt {
    type Output = Self;

    fn mul(self, other: &VecInt) -> Self {
        let mut result_rev_digits: Vec<i32> = vec![];
        for (i, d) in other.rev_digits.iter().enumerate() {
            for (j, f) in self.rev_digits.iter().enumerate() {
                if i + j >= result_rev_digits.len() {
                    result_rev_digits.push(d * f);
                } else {
                    result_rev_digits[i + j] += d * f;
                }
            }
        }
        let mut muled = Self { rev_digits: result_rev_digits };
        muled.legalise();
        return muled;
    }
}


impl Mul<VecInt> for VecInt {
    type Output = Self;

    fn mul(self, other: VecInt) -> Self {
        self.mul(&other)
    }
}


impl Mul<&i32> for VecInt {
    type Output = Self;

    fn mul(self, other: &i32) -> Self {
        return self.mul(*other);
    }
}

impl Mul<i32> for VecInt {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        let other_vec_int = Self::new(other);
        return self.mul(&other_vec_int);
    }
}



fn main() {

    let mut all_items:  Vec<String> = vec![];
    for a in 2..=N {
        for b in 2..=N {
            all_items.push(VecInt::new(a as i32).pow(b as i32).to_string());
        }
    }
    let max_len = (&all_items).iter().map(|s| s.len() ).max().unwrap();
    println!("max_len = {}", max_len);
    all_items.sort();
    all_items.dedup();
    println!("{:?}", all_items);
    println!("{}", all_items.len());

}
