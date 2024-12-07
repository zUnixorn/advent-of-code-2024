use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use nom::bytes::complete::tag;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use advent_of_code::number;

advent_of_code::solution!(5);

fn parse(input: &str) -> IResult<&str, (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>)> {
    let (rest, parsed_rules) = separated_list0(tag("\n"), separated_pair(number, tag("|"), number))(input.trim())?;
    let (rest, updates) = separated_list0(tag("\n"), separated_list0(tag(","), number))(rest.trim())?;
    let mut rules = HashMap::with_capacity(parsed_rules.len());

    for (key, value) in parsed_rules {
        rules.entry(key).or_insert_with(|| HashSet::new()).insert(value);
    }

    Ok((rest, (rules, updates)))
}

fn split_updates(rules: &HashMap<u32, HashSet<u32>>, updates: &Vec<Vec<u32>>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut correct = Vec::new();
    let mut incorrect = Vec::new();
    let mut updates: Vec<HashMap<_, _>> = updates.iter()
        .map(|u| HashMap::from_iter(u.iter().enumerate().map(|(i, p)| (*p, i))))
        .collect::<Vec<_>>();

    while let Some(update) = updates.pop() {
        let mut valid = true;

        for page in update.keys().copied() {
            let page_index = *update.get(&page).unwrap();

            if let Some(after_pages) = rules.get(&page) {
                for after_page in after_pages.iter().copied() {
                    if page_index > *update.get(&after_page).unwrap_or(&usize::MAX) {
                        valid = false;
                        break
                    }
                }

                if !valid {
                    break
                }
            }
        }

        let mut update = update.into_iter().collect::<Vec<_>>();
        update.sort_by_key(|(_, i)| *i);

        let update = update.into_iter().map(|(p, _)| p).collect();

        if valid {
            correct.push(update);
        } else {
            incorrect.push(update);
        }
    }

    (correct, incorrect)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (rules, updates)) = parse(input).ok()?;
    let (correct, _) = split_updates(&rules, &updates);

    Some(correct.into_iter().map(|u| u[u.len() / 2]).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, (rules, updates)) = parse(input).ok()?;
    let (_, incorrect) = split_updates(&rules, &updates);
    let incorrect = incorrect.into_iter().map(|mut update| {
        update.sort_unstable_by(|a, b| {
            let less =  rules.get(a).map(|ab| ab.contains(b)).unwrap_or(false);
            let greater =  rules.get(b).map(|aa| aa.contains(a)).unwrap_or(false);

            match (less, greater) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                (false, false) => Ordering::Equal,
                _ => unreachable!()
            }
        });

        update
    });

    Some(incorrect.map(|u| u[u.len() / 2]).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
