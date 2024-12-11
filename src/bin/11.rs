use advent_of_code::number;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;
use rayon::iter::plumbing::{Folder, Reducer, UnindexedConsumer};
use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator};
use rayon::prelude::*;

advent_of_code::solution!(11);

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(" "), number)(input.trim())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut stones) = parse(input).ok()?;

    for _ in 0..25 {
        for i in 0..stones.len() {
            match stones[i] {
                0 => stones[i] = 1,
                number => {
                    let mut digits = 0;
                    let mut rest = number;

                    while rest > 0 {
                        rest /= 10;
                        digits += 1;
                    }

                    if digits & 1 == 0 {
                        let half_power = 10u64.pow(digits / 2);

                        stones[i] = number / half_power;
                        stones.push(number % half_power);
                    } else {
                        stones[i] *= 2024
                    }
                },
            }
        }
    }

    Some(stones.len() as u32)
}

/*
The number of total stone can be counted while being generated. Every time a split happen,
one is added to the previous count of the previous iteration.
The previous count of the first iteration is the number of starting stones.
*/

// from https://users.rust-lang.org/t/how-to-wrap-a-non-object-safe-trait-in-an-object-safe-one/33904/5
#[derive(Copy, Clone)]
struct BlinkIterator {
    stone: Option<u64>,
    blinks: usize,
}

impl BlinkIterator {
    pub fn new(i: Option<u64>, depth: usize) -> Self {
        Self {
            stone: i,
            blinks: depth,
        }
    }
}
impl ParallelIterator for BlinkIterator {
    type Item = u64;
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        if self.stone == None {
            let folder = consumer.into_folder();
            folder.complete()
        } else if self.blinks == 0 {
            let folder = consumer.into_folder();
            folder.consume(self.stone.unwrap()).complete()
        } else {
            let (left, right, reducer) = consumer.split_at(1 << (self.blinks - 1));

            let depth = self.blinks - 1;
            let (iter1, iter2) = match self.stone {
                Some(0) => (
                    BlinkIterator::new(Some(1), depth),
                    BlinkIterator::new(None, depth),
                ),
                Some(number) => {
                    let mut digits = 0;
                    let mut rest = number;

                    while rest > 0 {
                        rest /= 10;
                        digits += 1;
                    }

                    if digits & 1 == 0 {
                        let half_power = 10u64.pow(digits / 2);
                        (
                            BlinkIterator::new(Some(number / half_power), depth),
                            BlinkIterator::new(Some(number % half_power), depth),
                        )
                    } else {
                        (
                            BlinkIterator::new(Some(number * 2024), depth),
                            BlinkIterator::new(None, depth),
                        )
                    }
                },
                _ => unreachable!(),
            };

            let (res1, res2) = rayon::join(
                move || iter1.drive_unindexed(left),
                move || iter2.drive_unindexed(right),
            );
            reducer.reduce(res1, res2)
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut stones) = parse(input).ok()?;

    Some(
        stones
            .into_par_iter()
            .flat_map(|x| BlinkIterator::new(Some(x), 55))
            .count() as u32,
    )
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
        assert_eq!(result, None);
    }
}
