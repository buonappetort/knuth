use std::fmt;

// Definiteness: make sure that m and n are postitive numbers
// by using signed integers
struct EuclidInput {
    m: i32,
    n: i32,
}

#[derive(Debug, PartialEq)]
struct Answer {
    gcd: i32,
    iter: i32,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GCD: {}, Iterations: {}", self.gcd, self.iter)
    }
}

// Euclids algo (E)
// we want m and n to be owned so we do not
// take over ownership of the inputted values
fn algo_e(e_input: &EuclidInput) -> Answer {
    // given 2 positive integers m and n
    // find their greatest common divisor
    // the largest possible integer that evenly
    // divides both m and n

    let mut m = e_input.m;
    let mut n = e_input.n;

    // E0
    if m < n {
        // swap em
        let n0 = n;
        let m0 = m;
        m = m0;
        n = n0;
    }

    let mut i = 1;
    loop {
        let r = m % n;

        match r {
            0 => break Answer { gcd: n, iter: i },
            _ => {
                i = i + 1;
                m = n;
                n = r;
            }
        }
    }
}

fn algo_f(e_input: &EuclidInput) -> Answer {
    let mut m = e_input.m;
    let mut n = e_input.n;
    let mut i = 1;

    loop {
        // F1 divide m by n and let m be the remainder
        m = m % n;

        match m {
            // F2: if m=0 then answer is n
            0 => break Answer { gcd: n, iter: i },
            _ => {
                // F3: divide n by m and let n be the remainder
                n = n % m;
                match n {
                    // F4: if n=0 then answer is m
                    0 => break Answer { gcd: m, iter: i  },
                    _ => {
                        i = i + 1;
                        continue;
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Chapter 1");

    println!("Euclid's algorithm");
    // TODO - read from command line user input
    let m = 6099;
    let n = 2166;
    let e_input = EuclidInput {
        m: m.clone(),
        n: n.clone(),
    };
    let euc_e = algo_e(&e_input);
    println!("{}", &euc_e);

    println!("Section 1.1 Exercises");

    // Number 3
    println!("1.1.3");
    // Change algo E so that all trivial replacement operations
    // such as m <- n are avoided, call this algorithm f
    // Faster!!!
    let euc_f = algo_f(&e_input);
    println!("{}", &euc_f);
}

#[test]
fn euclid_e() {
    let right_answer = Answer { gcd: 17, iter: 5 };
    // TODO: macro
    let answer = algo_e(&EuclidInput { m: 119, n: 544 });

    assert_eq!(right_answer, answer);

    // TODO
    // m = qn + r
}

#[test]
fn euclid_f() {
    let right_answer = Answer { gcd: 17, iter: 3 };
    let answer = algo_f(&EuclidInput { m: 119, n: 544 });

    assert_eq!(right_answer, answer);
}
