extern crate num_bigint;
extern crate num_traits;

use std::fmt::{Display,Formatter,Result};
use std::collections::BTreeSet;
use std::env;
use std::cmp::Ordering;
use std::io::stdin;
use num_bigint::BigInt;
use num_traits::{pow, One};

struct ExpNum {
    base : i32,
    exponent : i32
}

impl ExpNum {
    fn new(base:i32, exponent: i32) -> ExpNum {
        ExpNum {
            base,
            exponent
        }
    }

    fn extract(num : &mut i32, divisor : i32) -> Option<ExpNum> {
        let mut ds = 0;
        while *num % divisor == 0 {
            ds += 1;
            *num /= divisor;
        }
        if ds != 0 {
            Some(ExpNum::new(divisor, ds))
        }else {
            None
        }
    }

    fn eval(&self) -> BigInt {
        let b = BigInt::from(self.base);
        let e = self.exponent as usize;
        pow(b,e)
    }

    fn max_exponent(self, other : ExpNum) -> ExpNum {
        if self.exponent > other.exponent {self} else {other}
    }
}

impl PartialEq for ExpNum {
    fn eq(&self, other : &ExpNum) -> bool {
        self.base == other.base
    }
}

impl Eq for ExpNum {}

impl Ord for ExpNum {
    fn cmp(&self, other : &ExpNum) -> Ordering {
        self.base.cmp(&other.base)
    }
}

impl PartialOrd for ExpNum {
    fn partial_cmp(&self, other : &ExpNum) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for ExpNum {
    fn fmt(&self, f : &mut Formatter) -> Result {
        if self.exponent == 1 {
            return write!(f, "{}", self.base)
        }else {
            return write!(f, "{}^{}", self.base, self.exponent)
        }
    }
}

struct Factors {
    factors : BTreeSet<ExpNum>
}

impl Factors {
    fn new(mut num : i32) -> Factors {
        let mut factors = BTreeSet::new(); // Set of the factors in num
        for i in 2..=(f64::from(num)).sqrt() as i32 { // For every number between 2 and sqrt(num)
            let e = ExpNum::extract(&mut num, i);
            if e.is_some() {
                factors.insert(e.unwrap());
            } 
        }

        if factors.is_empty() { // if no factors, num is prime
            factors.insert(ExpNum::new(num,1));
        }

        Factors {factors}
    }

    fn insert_with(&mut self, e : ExpNum, f : impl Fn(ExpNum, ExpNum) -> ExpNum ) {
        let fs = &mut self.factors;
        if fs.contains(&e) {
            let new = f (fs.take(&e).unwrap(), e);
            fs.insert(new);
        }else {
            fs.insert(e);
        }
    }

    fn update_factor(&mut self, e : ExpNum) {
        self.insert_with(e, |x : ExpNum, o : ExpNum| x.max_exponent(o));
    }

    fn merge(&mut self, other : Factors) {
        for n in other.factors {
            self.update_factor(n);
        }
    }

    fn eval(self) -> BigInt {
        let result : BigInt = self.factors.into_iter().map(|x : ExpNum| x.eval()).fold(One::one(), |acc, e| acc * e);
        result
    }
}

impl Display for Factors {
    fn fmt(&self, f : &mut Formatter) -> Result {
        let mut str = String::new();
        for exp in &self.factors {
            str.push_str(&exp.to_string());
            str.push_str(" * ");
        }
        write!(f, "{}", str.trim_end_matches(" * "))
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();

    let n : i32 = match args.get(1) {
        Some(x) => x.parse().unwrap(),
        None => get_input()
    };

    let eval : bool = match args.get(2) {
        Some(x) => x == "eval",
        None => false
    };

    let mut result = Factors::new(1);
    for i in 2..n {
        result.merge(Factors::new(i));
    }
    print!("{}", result);
    if eval {
        print!(" = {}", result.eval());
    }
    println!();
}

fn get_input() -> i32 {
    println!("Please supply an upper bound : ");
    let mut inp : String = String::from("");
    stdin().read_line(&mut inp).expect("Couldn't read input");

    let res : i32 = inp.trim().parse().expect("Couldn't parse integer");
    res
}
