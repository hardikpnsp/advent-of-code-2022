use std::fs::File;
use utils::read_lines;

fn process_lines(lines: &Vec<String>) -> Vec<Vec<i64>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().into())
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn is_tree_visible(i: usize, j: usize, heights: &Vec<Vec<i64>>) -> bool {
    if i == (heights.len() - 1) || i == 0 || j == (heights[0].len() - 1) || j == 0 {
        return true;
    }

    let height = heights[i][j];

    let south = ((i + 1usize)..heights.len()).map(|x| heights[x][j] < height).all(|x| x);
    let north =  (0..i).map(|x| heights[x][j] < height).all(|x| x);
    let east =  ((j + 1usize)..heights[0].len()).map(|x| heights[i][x] < height).all(|x| x);
    let west =  (0..j).map(|x| heights[i][x] < height).all(|x| x);

    west || north || east || south
}

fn find_number_of_trees_visible_from_outside(heights: &Vec<Vec<i64>>) -> i64 {
    let height = heights.len();
    let width = heights[0].len();
    let mut visible_trees = 0;

    for i in 0..height {
        for j in 0..width {
            if is_tree_visible(i, j, heights) {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn main() {
    let file = File::open("tree_top_tree_house/input/input.txt").unwrap();
    let lines = read_lines(file);

    let heights = process_lines(&lines);

    println!("Part 1: number of trees visible from outside - {:?}", find_number_of_trees_visible_from_outside(&heights));
}
