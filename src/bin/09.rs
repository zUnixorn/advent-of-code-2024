use std::iter;

advent_of_code::solution!(9);

pub fn parse(input: &str) -> (Vec<Option<usize>>, Vec<usize>, Vec<u64>) {
    let parsed = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();

    let expanded = parsed
        .iter()
        .step_by(2)
        .zip(parsed.iter().skip(1).step_by(2).chain(iter::repeat(&0)))
        .enumerate()
        .map(|(i, (a, b))| {
            iter::repeat(Some(i))
                .take(*a as usize)
                .chain(iter::repeat(None).take(*b as usize))
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut offsets = parsed.iter().map(|i| *i as usize).collect::<Vec<_>>();
    offsets.insert(0, 0);
    offsets.pop();

    for i in 1..offsets.len() {
        offsets[i] += offsets[i - 1];
    }

    (expanded, offsets, parsed)
}

fn swap_parts<T>(slice: &mut [T], a: usize, b: usize, len: usize) {
    let lower = a.min(b);
    let upper = a.max(b);

    if lower + len > upper {
        panic!()
    }

    let (left, right) = slice.split_at_mut(upper);
    left[lower..(lower + len)].swap_with_slice(&mut right[..len]);
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut expanded, _, _) = parse(input);

    let mut j = expanded.len() - 1;

    for i in 0..expanded.iter().copied().flatten().count() {
        while let None = expanded[j] {
            j -= 1;
        }

        if expanded[i] == None {
            expanded.swap(i, j);
        }
    }

    Some(
        expanded
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(a, b)| (a as u64) * (b as u64))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut expanded, mut offsets, mut lengths) = parse(input);
    let mut block = lengths.len() & !1; // round down to nearest even number

    while block > 0 {
        let mut free = 1;
        let block_length = lengths[block];

        while free < block {
            if lengths[free] >= block_length {
                break;
            }

            free += 2;
        }

        if free > block {
            block -= 2;
            continue;
        }

        swap_parts(
            &mut expanded,
            offsets[free],
            offsets[block],
            block_length as usize,
        );

        lengths[block] = 0;
        lengths[free] -= block_length;

        // move to separate block with 0 space in between
        lengths.insert(free, block_length);
        lengths.insert(free, 0);

        // correct offsets
        offsets.insert(free, offsets[free]);
        offsets.insert(free + 2, offsets[free] + block_length as usize);
    }

    Some(
        expanded
            .into_iter()
            .enumerate()
            .filter_map(|(a, b)| b.map(|b| a as u64 * b as u64))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }

    #[test]
    fn test_slice_swap() {
        let mut a = (0..10).collect::<Vec<_>>();

        swap_parts(&mut a, 0, 5, 3);

        assert_eq!(a, vec![5, 6, 7, 3, 4, 0, 1, 2, 8, 9]);

        let mut b = (0..10).collect::<Vec<_>>();

        swap_parts(&mut b, 5, 0, 5);

        assert_eq!(b, vec![5, 6, 7, 8, 9, 0, 1, 2, 3, 4]);
    }
}
