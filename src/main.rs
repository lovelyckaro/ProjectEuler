use std::fmt::{Display,Formatter,Result};
use std::env;

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

    fn to_int(&self) -> i32 {
        self.base.pow(self.exponent as u32)
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
    factors : Vec<ExpNum>
}

impl Factors {
    fn new(mut num : i32) -> Factors {
        let mut factors = Vec::new(); // Vector of the factors in num
        for i in 2..=(num as f64).sqrt() as i32 { // For every number between 2 and sqrt(num)
            let mut is = 0; // Number of that factor in num
            while num % i == 0 {
                is += 1;
                num /= i;
            }
            if is != 0 {
                factors.push(ExpNum::new(i, is));
            }
        }

        if factors.is_empty() { // if no factors, num is prime
            factors.push(ExpNum::new(num,1));
        }

        Factors {factors}
    }

    fn update_factor(&mut self, e : ExpNum) {
        let mut contained = false;
        for num in self.factors.iter_mut() {
            if num.base == e.base {
                contained = true;
                if num.exponent < e.exponent {
                    num.exponent = e.exponent;
                }
            }
        }
        if !contained {
            self.factors.push(e);
        }
    }

    fn merge(&mut self, other : Factors) {
        for n in other.factors {
            self.update_factor(n);
        }
    }

    fn eval(&self) -> i32 {
        let mut result = 1;
        for num in &self.factors {
            result *= num.to_int();
        }
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
        None => return ()
    };

    let eval : bool = match args.get(2) {
        Some(x) => x == "eval",
        None => false
    };

    let mut result = Factors::new(12);
    for i in 2..n {
        result.merge(Factors::new(i));
    }
    print!("{}", result);
    if eval {
        print!(" = {}", result.eval());
    }
    println!();
}
