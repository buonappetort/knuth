use std::fmt;

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

// we want m and n to be owned so we do not
// take over ownership of the inputted values
//
// Definiteness: make sure that m and n are postitive numbers
// by using signed integers
fn euclids(mut m: i32, mut n: i32) -> Answer {
    // given 2 positive integers m and n
    // find their greatest common divisor
    // the largest possible integer that evenly
    // divides both m and n

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

fn main() {
    println!("Chapter 1");

    // TODO - read from command line user input
    let m = 119;
    let n = 544;
    let euc = euclids(m.clone(), n.clone());
    println!("{}", &euc);
}

#[test]
fn euclid_slow() {
    let right_answer = Answer { gcd: 17, iter: 5 };
    let answer = euclids(119, 544);
    assert_eq!(right_answer, answer);

    // TODO
    // m = qn + r
}
