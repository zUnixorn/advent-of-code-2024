use std::ops::Index;

advent_of_code::solution!(4);

struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input.trim()
            .split('\n')
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<_>>>();

        Self {
            width: grid.len(),
            height: grid[0].len(),
            grid,
        }
    }

    fn all_letter_locations(&self, letter: char) -> Vec<(usize, usize)> {
        let mut entries = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == letter {
                    entries.push((x, y));
                }
            }
        }

        entries
    }

    fn get_letter_offset(&self, location: (usize, usize), offset: (isize, isize)) -> Option<(usize, usize)> {
        if let Some(nx) = location.0.checked_add_signed(offset.0) {
            if let Some(ny) = location.1.checked_add_signed(offset.1) {
                if nx < self.width && ny < self.height {
                    return Some((nx, ny));
                }
            }
        }

        None
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.1][index.0]
    }
}

// Would have liked to use a graph for this, unfortunately I couldn't find a graph crate that doesn't have this issue: https://github.com/petgraph/petgraph/issues/186
pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let entries = grid.all_letter_locations('X');

    let letters = ['M', 'A', 'S'];
    let directions = [
        (-1, -1),
        (-1,  0),
        (-1,  1),
        ( 0, -1),
        ( 0,  1),
        ( 1, -1),
        ( 1,  0),
        ( 1,  1),
    ];

    let mut occurrences = 0;

    for entry_location in entries {
        for offset in directions {
            let mut valid = true;
            let mut current_location = entry_location;

            for letter in letters {
                if let Some(next_location) = grid.get_letter_offset(current_location, offset) {
                    current_location = next_location;

                    if letter == grid[current_location] {
                        continue;
                    }
                }

                valid = false;
                break;
            }

            if valid {
                occurrences += 1;
            }
        }
    }

    Some(occurrences)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let centers = grid.all_letter_locations('A');

    let mut occurrences = 0;

    for center in centers {
        let mut valid = true;

        for offsets in [[(-1, -1), (1, 1)], [(1, -1), (-1, 1)]] {
            let letters = ['M', 'S'];
            let is_letter_at_offset = |(i, l)| grid.get_letter_offset(center, offsets[i])
                .map(|o| grid[o] == l)
                .unwrap_or(false);

            let forward = letters.into_iter().enumerate().all(is_letter_at_offset);
            let backward = letters.into_iter().rev().enumerate().all(is_letter_at_offset);

            if !forward && !backward {
                valid = false;
                break;
            }
        }

        if valid {
            occurrences += 1;
        }
    }

    Some(occurrences)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
