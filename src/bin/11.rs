use advent_of_code::number;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;
use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(" "), number)(input.trim())
}

fn cached(args: (u64, u32), cache: &mut HashMap<(u64, u32), u64>, max_depth: u32) -> u64 {
    if let Some(v) = cache.get(&args) {
        *v
    } else {
        let v = blink(args, cache, max_depth);
        cache.insert(args, v);
        v
    }
}

fn blink(args: (u64, u32), cache: &mut HashMap<(u64, u32), u64>, max_depth: u32) -> u64 {
    if args.1 >= max_depth {
        return 0;
    };

    if args.0 == 0 {
        return cached((1, args.1 + 1), cache, max_depth);
    }

    let mut digits = 0;
    let mut rest = args.0;

    while rest > 0 {
        rest /= 10;
        digits += 1;
    }

    if digits & 1 == 0 {
        let half_power = 10u64.pow(digits / 2);
        let left = args.0 / half_power;
        let right = args.0 % half_power;

        let a = cached((left, args.1 + 1), cache, max_depth);
        let b = cached((right, args.1 + 1), cache, max_depth);

        return a + b + 1;
    }

    cached((args.0 * 2024, args.1 + 1), cache, max_depth)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, stones) = parse(input).ok()?;
    let mut sum = stones.len() as u64;
    let mut cache = HashMap::new();

    for stone in stones {
        sum += blink((stone, 0), &mut cache, 25)
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, stones) = parse(input).ok()?;
    let mut sum = stones.len() as u64;
    let mut cache = HashMap::new();

    for stone in stones {
        sum += blink((stone, 0), &mut cache, 75)
    }

    Some(sum)
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
