use std::fs;

use nalgebra::{max, DMatrix, DMatrixSlice, DVector, RowDVector};

type TreeHeight = i32;

fn load_input() -> String {
    fs::read_to_string("input.txt").expect("Should have been able to read the file")
}

fn parse_row(input: &str) -> RowDVector<TreeHeight> {
    RowDVector::from_iterator(
        input.len(),
        input
            .chars()
            .map(|s| s.to_string().parse::<TreeHeight>().unwrap()),
    )
}

fn parse_input(input: &str) -> DMatrix<TreeHeight> {
    let rows: Vec<RowDVector<TreeHeight>> = input
        .split("\n")
        .filter(|a| a != &"")
        .map(parse_row)
        .collect();
    DMatrix::from_rows(&rows)
}

fn get_rolling_max_rowwise(matrix: &DMatrix<TreeHeight>, is_downwards: bool) -> DMatrix<bool> {
    let mut current_max: RowDVector<TreeHeight> = RowDVector::repeat(matrix.ncols(), -1);

    let mut mat = matrix.clone_owned();

    if !is_downwards {
        let mut stack = Vec::new();
        matrix.row_iter().for_each(|row| stack.push(row));
        stack.reverse();
        mat = DMatrix::from_rows(&stack);
    }

    let mut rows: Vec<RowDVector<bool>> = mat
        .row_iter()
        .map(|row| {
            let return_val = row.zip_map(&current_max, |x, y| x > y);
            current_max = row.zip_map(&current_max, max);
            return_val
        })
        .collect();

    if !is_downwards {
        rows.reverse();
    }

    DMatrix::from_rows(&rows)
}

fn get_rolling_max_colwise(matrix: &DMatrix<TreeHeight>, is_downwards: bool) -> DMatrix<bool> {
    let mut current_max: DVector<TreeHeight> = DVector::repeat(matrix.nrows(), -1);

    let mut mat = matrix.clone_owned();

    if !is_downwards {
        let mut stack = Vec::new();
        matrix.column_iter().for_each(|col| stack.push(col));
        stack.reverse();
        mat = DMatrix::from_columns(&stack);
    }

    let mut columns: Vec<DVector<bool>> = mat
        .column_iter()
        .map(|col| {
            let return_val = col.zip_map(&current_max, |x, y| x > y);
            current_max = col.zip_map(&current_max, max);
            return_val
        })
        .collect();

    if !is_downwards {
        columns.reverse();
    }

    DMatrix::from_columns(&columns)
}

fn is_visible(
    matrix: &DMatrix<TreeHeight>,
    is_row_wise: bool,
    is_downwards: bool,
) -> DMatrix<bool> {
    match is_row_wise {
        true => get_rolling_max_rowwise(matrix, is_downwards),
        _ => get_rolling_max_colwise(matrix, is_downwards),
    }
}

fn get_visible_trees(grid: &DMatrix<TreeHeight>) -> TreeHeight {
    let from_top = is_visible(grid, true, true);
    let from_left = is_visible(grid, false, true);
    let from_bottom = is_visible(grid, true, false);
    let from_right = is_visible(grid, false, false);

    let combined: DMatrix<TreeHeight> = from_top
        .zip_map(&from_left, |x, y| x || y)
        .zip_map(&from_bottom, |x, y| x || y)
        .zip_map(&from_right, |x, y| x || y)
        .map(|x| match x {
            true => 1,
            _ => 0,
        });

    combined.sum()
}

fn count_visible_trees(
    value: &TreeHeight,
    slice: DMatrixSlice<TreeHeight>,
    reverse: bool,
) -> TreeHeight {
    let mut num_visible_trees = 0;

    let iter: Box<dyn Iterator<Item = &TreeHeight>> = if !reverse {
        Box::new(slice.iter())
    } else {
        Box::new(slice.iter().rev())
    };

    for a in iter {
        num_visible_trees = num_visible_trees + 1;

        if a >= value {
            break;
        }
    }

    num_visible_trees
}

fn get_scenic_score(grid: &DMatrix<TreeHeight>, i: usize, j: usize, v: TreeHeight) -> TreeHeight {
    let trees_up = grid.slice((0, j), (i, 1));
    let trees_left = grid.slice((i, 0), (1, j));
    let trees_right = grid.slice((i, j + 1), (1, grid.ncols() - j - 1));
    let trees_down = grid.slice((i + 1, j), (grid.nrows() - i - 1, 1));

    let visible_trees_up = count_visible_trees(&v, trees_up, true);
    let visible_trees_left = count_visible_trees(&v, trees_left, true);
    let visible_trees_right = count_visible_trees(&v, trees_right, false);
    let visible_trees_down = count_visible_trees(&v, trees_down, false);

    visible_trees_up * visible_trees_left * visible_trees_right * visible_trees_down
}

fn get_scenic_scores(grid: &DMatrix<TreeHeight>) -> DMatrix<TreeHeight> {
    grid.map_with_location(|i, j, v| get_scenic_score(grid, i, j, v))
}

fn main() {
    let input = load_input();
    let grid = parse_input(&input);
    let visible_count = get_visible_trees(&grid);
    println!("{}", visible_count);

    let scenic_scores = get_scenic_scores(&grid);
    let max_scenic_score = scenic_scores.max();
    println!("{}", max_scenic_score);
}

#[cfg(test)]
mod tests {
    use nalgebra::{dmatrix, DMatrix};

    use crate::{get_scenic_score, get_visible_trees, is_visible, parse_input, TreeHeight};

    fn get_test_matrix() -> DMatrix<TreeHeight> {
        dmatrix![
            3, 0, 3, 7, 3;
            2, 5, 5, 1, 2;
            6, 5, 3, 3, 2;
            3, 3, 5, 4, 9;
            3, 5, 3, 9, 0;
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
    fn test_is_visible_top() {
        let input = get_test_matrix();
        let expected_top = dmatrix![
            true, true, true, true, true;
            false, true, true, false, false;
            true, false, false, false, false;
            false, false, false, false, true;
            false, false, false, true, false;
        ];
        let actual_top = is_visible(&input, true, true);
        assert_eq!(expected_top, actual_top);
    }

    #[test]
    fn test_is_visible_right() {
        let input = get_test_matrix();
        let expected_right = dmatrix![
            true, false, false, true , false;
            true, true , false, false, false;
            true, false, false, false, false;
            true, false, true , false, true ;
            true, true , false, true, false;
        ];
        let actual_right = is_visible(&input, false, true);
        assert_eq!(expected_right, actual_right);
    }

    #[test]
    fn test_is_visible_left() {
        let input = get_test_matrix();

        let expected_left = dmatrix![
            false, false, false, true , true ;
            false, false, true , false, true ;
            true, true , false, true , true ;
            false, false, false, false, true ;
            false, false, false, true , true ;
        ];
        let actual_left = is_visible(&input, false, false);
        println!("{}", expected_left);
        println!("{}", actual_left);
        assert_eq!(expected_left, actual_left);
    }

    #[test]
    fn test_is_visible_bottom() {
        let input = get_test_matrix();

        let expected_bottom = dmatrix![
            false, false, false, false, false;
            false, false, false, false, false;
            true , false, false, false, false;
            false, false, true , false, true ;
            true , true , true , true , true ;
        ];
        let actual_bottom = is_visible(&input, true, false);
        assert_eq!(expected_bottom, actual_bottom);
    }

    #[test]
    fn test_get_visible_trees() {
        let input = get_test_matrix();
        let expected = 21;
        let actual = get_visible_trees(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_scenic_score() {
        let input = get_test_matrix();
        let expected = 4;
        let actual = get_scenic_score(&input, 1, 2, 5);
        assert_eq!(expected, actual);

        let expected_2 = 8;
        let actual_2 = get_scenic_score(&input, 3, 2, 5);
        assert_eq!(expected_2, actual_2);
    }
}
