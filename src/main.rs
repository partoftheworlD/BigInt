use std::{fmt::Display, ops::Mul};

mod tests;

#[derive(Debug, Default, PartialEq)]
struct BigInt {
    sign: bool,
    capacity: usize,
    nodigints: usize,
    digits: Vec<char>,
}

impl BigInt {
    pub fn new(capacity: usize) -> Self {
        Self {
            sign: true,
            nodigints: 0,
            capacity,
            digits: Vec::with_capacity(capacity),
        }
    }

    pub fn str_to_big(str: &'static str) -> Self {
        let len = str.len();

        if len == 0 {
            return Self {
                ..Default::default()
            };
        }

        let mut nodigints = len;
        let mut sign = true;

        if str.chars().nth(0).unwrap() == '-' {
            sign = false;
            nodigints -= 1;
        }

        Self {
            sign,
            capacity: nodigints,
            nodigints,
            digits: str.chars().collect(),
        }
    }

    pub fn bigint(value: i128) -> Self {
        let len = value.checked_ilog10().unwrap_or(0) + 1;
        if len == 0 {
            return Self {
                ..Default::default()
            };
        }

        let nodigints = len as usize;

        Self {
            capacity: nodigints,
            nodigints,
            digits: value.to_string().chars().collect(),
            ..Default::default()
        }
    }

    pub fn biguint(value: u128) -> Self {
        let len = value.checked_ilog10().unwrap_or(0) + 1;
        if len == 0 {
            return Self {
                ..Default::default()
            };
        }
        let nodigints = len as usize;

        Self {
            capacity: nodigints,
            nodigints,
            digits: value.to_string().chars().collect(),
            ..Default::default()
        }
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.digits.iter().collect::<String>())
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        let to_digits = |chars: &[char]| -> Vec<u32> {
            chars.iter().map(|c| c.to_digit(10).unwrap()).collect()
        };

        let mut v1 = to_digits(&self.digits);
        let mut v2 = to_digits(&rhs.digits);

        v1.reverse();
        v2.reverse();

        let mut tmp = vec![0; v1.len() + v2.len()];

        for (i, &digit1) in v1.iter().enumerate() {
            let mut carry = 0;
            for (j, &digit2) in v2.iter().enumerate() {
                let product = digit1 * digit2 + tmp[i + j] + carry;
                tmp[i + j] = product % 10;
                carry = product / 10;
            }
            if carry > 0 {
                tmp[i + v2.len()] += carry;
            }
        }

        while tmp.len() > 1 && tmp.last() == Some(&0) {
            tmp.pop();
        }

        tmp.reverse();

        let str: String = tmp
            .iter()
            .map(|&d| char::from_digit(d, 10).unwrap())
            .collect();

        let mut bigint = BigInt::new(str.len());

        bigint.sign = self.sign == rhs.sign;
        bigint.nodigints = str.len();
        bigint.digits = str.chars().collect();

        bigint
    }
}

fn main() {
    let a = BigInt::str_to_big("1233123121321");
    let b = BigInt::str_to_big("442342432423423");

    println!("{}", a * b);
}
