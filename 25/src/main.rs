use std::{
    env,
    fmt::{Display, Error, Write},
    fs,
    iter::Sum,
    ops::Add,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct SNAFU {
    value: i64,
}

impl SNAFU {
    const BASE: i64 = 5;
}

#[derive(Debug)]
struct ParseSNAFUError;

impl FromStr for SNAFU {
    type Err = ParseSNAFUError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let powers = (0..).map(|exponent| SNAFU::BASE.pow(exponent));

        let value = s
            .chars()
            .map(|digit| match digit {
                '2' => Ok(2),
                '1' => Ok(1),
                '0' => Ok(0),
                '-' => Ok(-1),
                '=' => Ok(-2),
                _ => Err(ParseSNAFUError),
            })
            .rev()
            .zip(powers)
            .map(|(base, exponent)| base.map(|b| b * exponent))
            .sum::<Result<i64, ParseSNAFUError>>()?;

        Ok(SNAFU { value })
    }
}

impl Display for SNAFU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut remainders = Vec::new();
        let mut quotient = self.value;

        while quotient != 0 {
            let remainder = quotient % SNAFU::BASE;

            quotient = quotient / SNAFU::BASE;

            remainders.push(remainder);
        }

        log::debug!("{:?}", remainders);

        let mut i = 0;
        while i < remainders.len() {
            if remainders[i] > 2 {
                remainders[i] = remainders[i] - SNAFU::BASE;

                if i == remainders.len() - 1 {
                    remainders.push(1);
                } else {
                    remainders[i + 1] += 1;
                }
            }

            i = i + 1;
        }

        remainders.reverse();

        log::debug!("{:?}", remainders);

        for remainder in remainders.into_iter() {
            let c = match remainder {
                2 => Ok('2'),
                1 => Ok('1'),
                0 => Ok('0'),
                -1 => Ok('-'),
                -2 => Ok('='),
                _ => Err(Error::default()),
            }?;

            f.write_char(c)?;
        }

        Ok(())
    }
}

impl Add for SNAFU {
    type Output = SNAFU;

    fn add(self, rhs: Self) -> Self::Output {
        SNAFU {
            value: self.value + rhs.value,
        }
    }
}

impl Sum for SNAFU {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(SNAFU { value: 0 }, |a, b| a + b)
    }
}

fn main() {
    env_logger::init();

    let input = load_input();
    let fuel_requirements: Vec<SNAFU> = parse_fuel_requirements(&input).unwrap();
    let total_fuel_requirements: SNAFU = fuel_requirements.iter().cloned().sum();

    println!("{}", total_fuel_requirements);
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_fuel_requirements(input: &str) -> Result<Vec<SNAFU>, ParseSNAFUError> {
    input
        .split('\n')
        .filter(|line| line != &"")
        .map(|snafu| snafu.parse())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::SNAFU;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_parse_snafu() {
        init();

        let input = "2=-01";
        let expected = SNAFU { value: 976 };

        let actual: SNAFU = input.parse().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_display_snafu() {
        init();

        const TESTS: &'static [(SNAFU, &str)] = &[
            (SNAFU { value: 1 }, "1"),
            (SNAFU { value: 2 }, "2"),
            (SNAFU { value: 3 }, "1="),
            (SNAFU { value: 4 }, "1-"),
            (SNAFU { value: 5 }, "10"),
            (SNAFU { value: 6 }, "11"),
            (SNAFU { value: 7 }, "12"),
            (SNAFU { value: 8 }, "2="),
            (SNAFU { value: 9 }, "2-"),
            (SNAFU { value: 10 }, "20"),
            (SNAFU { value: 15 }, "1=0"),
            (SNAFU { value: 20 }, "1-0"),
            (SNAFU { value: 2022 }, "1=11-2"),
            (SNAFU { value: 12345 }, "1-0---0"),
            (SNAFU { value: 314159265 }, "1121-1110-1=0"),
            (SNAFU { value: 4890 }, "2=-1=0"),
        ];

        for (input, expected) in TESTS {
            let actual = input.to_string();

            assert_eq!(expected.to_string(), actual);
        }
    }
}
