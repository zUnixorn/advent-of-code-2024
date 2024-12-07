use std::collections::HashSet;
use geo::{coord, Contains, Point, Rect};
use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::IResult;
use nom::multi::{many0, separated_list0};

advent_of_code::solution!(6);

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list0(tag("\n"), many0(none_of("\n")))(input.trim())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, parsed) = parse(input).ok()?;
    let mut obstacles = HashSet::new();
    let mut visited = HashSet::new();
    let width = parsed[0].len() as i32;
    let height = parsed.len() as i32;
    let bound = Rect::new(
        coord! { x: 0, y: 0},
        coord! { x: width - 1, y: height - 1}
    );
    let mut current_position = None;

    for y in 0..height {
        for x in 0..width {
            match parsed[y as usize][x as usize] {
                '#' => { obstacles.insert(Point::new(x, y)); }
                '^' => current_position = Some(Point::new(x, y)),
                _ => (),
            };
        }
    }

    let mut current_position = current_position.unwrap();
    visited.insert(current_position);
    let mut direction = 0;
    let directions = [
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
    ];

    while bound.contains(&current_position) {
        let next_position = current_position + directions[direction];

        if obstacles.contains(&next_position) {
            direction = (direction + 1) % directions.len();
        } else {
            current_position = next_position;
        }

        visited.insert(current_position);
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, parsed) = parse(input).ok()?;
    let mut obstacles = HashSet::new();
    let mut visited = HashSet::new();
    let width = parsed[0].len() as i32;
    let height = parsed.len() as i32;
    let bound = Rect::new(
        coord! { x: 0, y: 0},
        coord! { x: width - 1, y: height - 1}
    );
    let mut starting_position = None;

    for y in 0..height {
        for x in 0..width {
            match parsed[y as usize][x as usize] {
                '#' => { obstacles.insert(Point::new(x, y)); }
                '^' => starting_position = Some(Point::new(x, y)),
                _ => (),
            };
        }
    }

    let mut sum = 0;

    for y in 0..height {
        for x in 0..width {
            let new_obstacle = Point::new(x, y);
            let mut current_position = starting_position.unwrap();
            visited.clear();
            visited.insert((current_position, 0));
            let mut direction = 0;
            let directions = [
                Point::new(0, -1),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(-1, 0),
            ];

            while bound.contains(&current_position) {
                let next_position = current_position + directions[direction];

                if obstacles.contains(&next_position) || next_position == new_obstacle {
                    direction = (direction + 1) % directions.len();
                } else {
                    current_position = next_position;
                }

                if !visited.insert((current_position, direction)) {
                    sum += 1;
                    break;
                }
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
