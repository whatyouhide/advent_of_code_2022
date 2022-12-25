use std::str::FromStr;

struct SNAFU {
    digits: Vec<char>,
}

impl SNAFU {
    fn to_int(&self) -> i128 {
        let mut total = 0;

        for (index, digit) in self.digits.iter().enumerate() {
            total += Self::digit_value(*digit) as i128 * 5_i128.pow(index as u32);
        }

        total
    }

    pub fn digit_value(digit: char) -> i16 {
        match digit {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!("invalid digit {}, expected 0, 1, 2, -, or =", digit),
        }
    }
}

impl std::fmt::Display for SNAFU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for digit in self.digits.iter().rev() {
            write!(f, "{}", digit)?;
        }

        Ok(())
    }
}

impl TryFrom<i128> for SNAFU {
    type Error = &'static str;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let mut acc = value;
        let mut digits = Vec::new();

        loop {
            let c = match acc % 5 {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '=',
                4 => '-',
                _ => unreachable!(),
            };

            digits.push(c);

            acc = (acc - Self::digit_value(c) as i128) / 5;

            if acc == 0 {
                break;
            }
        }

        Ok(Self { digits })
    }
}

impl FromStr for SNAFU {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::new();

        for char in s.trim().chars().rev() {
            match char {
                '2' | '1' | '0' | '-' | '=' => digits.push(char),
                _ => return Err(()),
            };
        }

        Ok(Self { digits })
    }
}

#[cfg(test)]
mod snafu_tests {
    use super::*;

    const TABLE: [(&str, i128); 27] = [
        ("1", 1),
        ("2", 2),
        ("1=", 3),
        ("1-", 4),
        ("10", 5),
        ("11", 6),
        ("12", 7),
        ("2=", 8),
        ("2-", 9),
        ("20", 10),
        ("1=0", 15),
        ("1-0", 20),
        ("1=11-2", 2022),
        ("1-0---0", 12345),
        ("1=-0-2", 1747),
        ("12111", 906),
        ("2=0=", 198),
        ("21", 11),
        ("2=01", 201),
        ("111", 31),
        ("20012", 1257),
        ("112", 32),
        ("1=-1=", 353),
        ("1-12", 107),
        ("12", 7),
        ("1=", 3),
        ("122", 37),
    ];

    #[test]
    fn test_from_str_and_to_int() {
        for (snafu_str, expected_int) in TABLE.iter() {
            let snafu = snafu_str.parse::<SNAFU>().unwrap();
            assert_eq!(snafu.to_int(), *expected_int);
        }
    }

    #[test]
    fn test_fmt() {
        for (snafu_str, _) in TABLE.iter() {
            let snafu = snafu_str.parse::<SNAFU>().unwrap();
            assert_eq!(snafu.to_string(), *snafu_str);
        }
    }

    #[test]
    fn test_try_from() {
        for (snafu_str, expected_int) in TABLE.iter() {
            let snafu = SNAFU::try_from(*expected_int).unwrap();
            assert_eq!(snafu.to_string(), *snafu_str);
        }
    }
}

pub fn run(input: &str) {
    let sum_in_snafu: SNAFU = input
        .lines()
        .map(|line| line.parse::<SNAFU>().unwrap().to_int())
        .sum::<i128>()
        .try_into()
        .unwrap();

    println!("Sum of all SNAFUs (in SNAFU): {}", sum_in_snafu);
}
