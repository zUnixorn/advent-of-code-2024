use advent_of_code::number;
use bitvec::macros::internal::funty::Fundamental;
use bitvec::order::Lsb0;
use bitvec::view::BitView;
use itertools::{repeat_n, Itertools};
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;

advent_of_code::solution!(7);

fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list0(
        tag("\n"),
        separated_pair(number, tag(": "), separated_list0(tag(" "), number)),
    )(input.trim())
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, parsed) = parse(input).ok()?;

    let mut sum = 0;

    for (expected, operands) in parsed {
        'try_combos: for combo in 0..2u32.pow(operands.len() as u32 - 1) {
            let mut result = Some(operands[0]);

            for (i, bit) in combo.view_bits::<Lsb0>().into_iter().take(operands.len() - 1).enumerate() {
                if bit.as_bool() {
                    result = result.and_then(|a| a.checked_mul(operands[i + 1]))
                } else {
                    result = result.and_then(|a| a.checked_add(operands[i + 1]))
                }

                if !result.is_some_and(|r| r <= expected) {
                    continue 'try_combos;
                }
            }

            if result.unwrap() == expected {
                sum += expected;
                break;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, parsed) = parse(input).ok()?;

    let mut sum = 0;

    for (expected, operands) in parsed {
        'try_combos: for combo in repeat_n(['*', '+', '|'].into_iter(), operands.len() - 1).multi_cartesian_product() {
            let mut result = Some(operands[0]);

            for (i, op) in combo.into_iter().take(operands.len() - 1).enumerate() {
                match op {
                    '*' => result = result.and_then(|a| a.checked_mul(operands[i + 1])),
                    '+' => result = result.and_then(|a| a.checked_add(operands[i + 1])),
                    '|' => result = result.and_then(|a| {
                        let mut n = a.to_string();
                        let b = operands[i + 1].to_string();
                        n.push_str(&b);
                        n.parse().ok()
                    }),
                    _ => (),
                }

                if !result.is_some_and(|r| r <= expected) {
                    continue 'try_combos;
                }
            }

            if result.unwrap() == expected {
                sum += expected;
                break;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
