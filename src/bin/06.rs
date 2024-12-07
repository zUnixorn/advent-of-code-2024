use std::collections::{HashMap, HashSet};
use geo::{coord, point, Contains, LinesIter, Point, Rect};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, none_of};
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

    let c = current_position.unwrap();

    let mut current_position = current_position.unwrap();
    // TODO check the starting position on all 4 sides
    let mut direction = 0;
    let directions = [
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
    ];

    let mut corners = [
        // top-left
        HashSet::new(),
        // top-right
        HashSet::new(),
        // bottom-right
        HashSet::new(),
        // bottom-left
        HashSet::new(),
    ];

    while bound.contains(&current_position) {
        let next_position = current_position + directions[direction];

        if obstacles.contains(&next_position) {
            corners[direction].insert(current_position);
            direction = (direction + 1) % directions.len();
        } else {
            current_position = next_position;
        }
    }

    let mut debug_rectangles = HashSet::new();
    let mut new_obstacles = HashSet::new();

    for triplet in corners.iter().combinations(3) {
        for a in triplet[0] {
            for b in triplet[1] {
                for c in triplet[2] {
                    if [*a, *b, *c].into_iter().combinations(2).filter(|c| c[0].x() == c[1].x() || c[0].y() == c[1].y()).count() != 2 {
                        continue;
                    }
                    let min_x = a.x().min(b.x()).min(c.x());
                    let min_y = a.y().min(b.y()).min(c.y());
                    let max_x = a.x().max(b.x()).max(c.x());
                    let max_y = a.y().max(b.y()).max(c.y());

                    let rectangle = Rect::new(
                        coord! { x: min_x, y: min_y},
                        coord! { x: max_x, y: max_y},
                    );

                    if rectangle.lines_iter().any(|l| obstacles.iter().any(|o| l.contains(o))) {
                        continue;
                    }

                    let missing = [min_x, max_x].into_iter()
                        .cartesian_product([min_y, max_y])
                        .map(|(x, y)| Point::new(x, y))
                        .filter(|p| ![a, b, c].contains(&p))
                        .next().unwrap();

                    if let Some(corner) = corners.iter().position(|corner| [a, b, c].iter().any(|i| corner.contains(i))) {
                        let new_obstacle = missing + directions[(corner + 3) % directions.len()];
                        let mut debug_context = ([*a, *b, *c, missing], new_obstacle);
                        debug_context.0.sort_by_key(|p| (p.x() + p.y()) * (p.x() + p.y() + 1) / 2 + p.y());
                        debug_rectangles.insert(debug_context);

                        new_obstacles.insert(new_obstacle);
                    }
                }
            }
        }
    }

    println!("{}", debug_rectangles.len());

    for (i, (corners, obstacle)) in debug_rectangles.iter().enumerate() {
        let mut debug_map = parsed.clone();

        for corner in corners {
            debug_map[corner.y() as usize][corner.x() as usize] = '+'
        }

        debug_map[c.y() as usize][c.x() as usize] = '^';

        for line in debug_map {
            println!("{}", line.into_iter().join(""))
        }

        println!()
    }

    Some(new_obstacles.difference(&obstacles).count() as u32)
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
