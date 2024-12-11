use std::collections::HashMap;
use geo::{coord, Contains, Point, Rect};
use petgraph::algo::all_simple_paths;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;

advent_of_code::solution!(10);

fn build_graph(input: &str) -> (Graph<u32, ()>, Vec<NodeIndex>, Vec<NodeIndex>) {
    let parsed = input
        .trim()
        .split('\n')
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect::<Vec<Vec<_>>>();

    let mut zeros = Vec::new();
    let mut nines = Vec::new();
    let mut graph = Graph::new();
    let mut coord_to_id = HashMap::new();
    let bound = Rect::new(
        coord! { x: -1, y: -1, },
        coord! { x: parsed[0].len() as i32, y: parsed.len() as i32 },
    );
    let directions = [
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
    ];

    for y in 0..parsed.len() {
        for x in 0..parsed[y].len() {
            let point = Point::new(x as i32, y as i32);
            let node = *coord_to_id.entry(point).or_insert_with(|| graph.add_node(parsed[y][x]));


            match parsed[y][x] {
                0 => zeros.push(node),
                9 => nines.push(node),
                _ => (),
            }

            for direction in directions {
                let neighbour = point + direction;

                if bound.contains(&neighbour) {
                    let neighbour_value = parsed[neighbour.y() as usize][neighbour.x() as usize] as i32;
                    let value = parsed[point.y() as usize][point.x() as usize] as i32;

                    if neighbour_value - value == 1 {
                        let neighbour_node = *coord_to_id.entry(neighbour).or_insert_with(|| graph.add_node(neighbour_value as u32));

                        graph.add_edge(node, neighbour_node, ());
                    }
                }
            }
        }
    }

    (graph, zeros, nines)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (graph, zeros, _) = build_graph(input);

    let mut sum = 0;

    for zero in zeros {
        let mut dfs = Dfs::new(&graph, zero);

        while let Some(i) = dfs.next(&graph) {
            if *graph.node_weight(i).unwrap() == 9 {
                sum += 1;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (graph, zeros, nines) = build_graph(input);

    let mut sum = 0;

    for zero in zeros {
        for nine in &nines {
            sum += all_simple_paths::<Vec<_>, _>(&graph, zero, *nine, 7, None).count() as u32;
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
