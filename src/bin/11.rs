use advent_of_code::number;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;
use cached::proc_macro::cached;

advent_of_code::solution!(11);

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(" "), number)(input.trim())
}

#[cached]
fn blink_stone(stone: u64, depth: u32, max_depth: u32) -> u64 {
    if depth >= max_depth {
        return 0;
    };

    if stone == 0 {
        return blink_stone(1, depth + 1, max_depth);
    }

    let mut digits = 0;
    let mut rest = stone;

    while rest > 0 {
        rest /= 10;
        digits += 1;
    }

    if digits & 1 == 0 {
        let half_power = 10u64.pow(digits / 2);
        let left = stone / half_power;
        let right = stone % half_power;

        let a = blink_stone(left, depth + 1, max_depth);
        let b = blink_stone(right, depth + 1, max_depth);

        return a + b + 1;
    }

    blink_stone(stone * 2024, depth + 1, max_depth)
}

fn blink(stones: Vec<u64>, times: u32) -> u64 {
    let mut sum = stones.len() as u64;

    for stone in stones {
        sum += blink_stone(stone, 0, times)
    }

    sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, stones) = parse(input).ok()?;

    Some(blink(stones, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, stones) = parse(input).ok()?;

    Some(blink(stones, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
