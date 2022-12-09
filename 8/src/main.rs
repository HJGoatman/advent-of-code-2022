use std::{cmp, fs};

type TreeHeight = i16;

fn load_input() -> String {
    fs::read_to_string("input.txt").expect("Should have been able to read the file")
}

fn matrix_transpose(m: &[&[TreeHeight]]) -> Vec<Vec<TreeHeight>> {
    let mut t = vec![Vec::with_capacity(m.len()); m[0].len()];
    for r in m {
        for i in 0..r.len() {
            t[i].push(r[i]);
        }
    }
    t
}

fn parse_row(input: &str) -> Vec<TreeHeight> {
    input
        .chars()
        .map(|s| s.to_string().parse::<TreeHeight>().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<TreeHeight>> {
    input
        .split("\n")
        .filter(|a| a != &"")
        .map(parse_row)
        .collect()
}

fn get_rolling_max(matrix: &[&[TreeHeight]]) -> Vec<Vec<TreeHeight>> {
    let mut output = Vec::new();
    let mut carry_row = vec![0; matrix.len()];

    for i in 0..(matrix.len()) {
        let row = matrix[i];

        carry_row = carry_row
            .into_iter()
            .zip(row.iter())
            .map(|(a_val, b_val)| -> TreeHeight { cmp::max(a_val, *b_val) })
            .collect();

        output.push(carry_row.clone());
    }

    output
}

fn apply<A, B>(matrix_a: &[&[B]], matrix_b: &[&[B]], function: &dyn Fn(B, B) -> A) -> Vec<Vec<A>>
where
    B: Copy,
{
    matrix_a
        .iter()
        .zip(matrix_b.iter())
        .map(|(row_a, row_b)| -> Vec<A> {
            row_a
                .into_iter()
                .zip(row_b.into_iter())
                .map(|(a, b)| function(*a, *b))
                .collect()
        })
        .collect()
}

fn subtract(a: TreeHeight, b: TreeHeight) -> TreeHeight {
    a - b
}

fn or(a: bool, b: bool) -> bool {
    a || b
}

fn row_wise_max_diff(matrix: &[&[TreeHeight]]) -> Vec<Vec<TreeHeight>> {
    let rolling_max = get_rolling_max(matrix);
    let rolling_max_slice: Vec<&[TreeHeight]> = rolling_max.iter().map(Vec::as_slice).collect();

    apply(matrix, &rolling_max_slice, &subtract)
}

fn is_visible(matrix: &[&[TreeHeight]]) -> Vec<Vec<bool>> {
    let difference = row_wise_max_diff(matrix);
    difference
        .iter()
        .map(|row| row.iter().map(|val| val >= &0).collect())
        .collect()
}

fn get_visible_trees(grid: Vec<Vec<TreeHeight>>) -> TreeHeight {
    let top_slice: Vec<&[TreeHeight]> = grid.iter().map(Vec::as_slice).collect();
    let bottom_slice: Vec<&[TreeHeight]> = top_slice.iter().cloned().rev().collect();

    let grid_transpose = matrix_transpose(&top_slice);
    let left_slice: Vec<&[TreeHeight]> = grid_transpose.iter().map(Vec::as_slice).collect();
    let right_slice: Vec<&[TreeHeight]> = left_slice.iter().cloned().rev().collect();

    let top_visible = is_visible(&top_slice);
    let mut bottom_visible = is_visible(&bottom_slice);
    bottom_visible = bottom_visible.into_iter().rev().collect();
    let mut left_visible = is_visible(&left_slice);
    // left_visible = matrix_transpose(left_visible);
    let mut right_visible = is_visible(&right_slice);
    // right_visible = matrix_transpose(right_visible.iter().rev());

    let mut any_visible = vec![vec![false; top_visible[0].len()]; top_visible.len()];
    for i in 0..top_visible.len() {
        for j in 0..top_visible[0].len() {
            any_visible[i][j] = top_visible[i][j]
                || bottom_visible[i][j]
                || left_visible[i][j]
                || right_visible[i][j]
        }
    }

    let visible_count: TreeHeight = any_visible
        .into_iter()
        .map(|row| row.into_iter().filter(|a| *a).count() as TreeHeight)
        .sum::<TreeHeight>();

    visible_count
}

fn main() {
    let input = load_input();
    let grid = parse_input(&input);
    let visible_count = get_visible_trees(grid);
    println!("{}", visible_count);
}

#[cfg(test)]
mod tests {
    use crate::{get_rolling_max, get_visible_trees, parse_input, TreeHeight};

    fn get_test_matrix() -> Vec<Vec<TreeHeight>> {
        vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let expected = get_test_matrix();
        let actual = parse_input(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_rolling_max() {
        let input = get_test_matrix();
        let expected = vec![
            vec![3, 0, 3, 7, 3],
            vec![3, 5, 5, 7, 3],
            vec![6, 5, 5, 7, 3],
            vec![6, 5, 5, 7, 9],
            vec![6, 5, 5, 9, 9],
        ];
        let actual_refs: Vec<&[TreeHeight]> = input.iter().map(Vec::as_slice).collect();
        let actual = get_rolling_max(&actual_refs);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_visible_trees() {
        let input = get_test_matrix();
        let expected = 21;
        // let actual_refs: Vec<&[TreeHeight]> = input.iter().map(Vec::as_slice).collect();
        let actual = get_visible_trees(input);
        assert_eq!(expected, actual);
    }
}
