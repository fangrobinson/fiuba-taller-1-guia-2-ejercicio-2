//! Contains diff implementation.
use super::grid;

/// Convenience function to check if South-East lines match.
///
/// Returns:
///
///  - bool with condition result.
fn check_last_diagonal_pos(
    file_lines_1: &[String],
    file_lines_2: &[String],
    i: usize,
    j: usize,
) -> bool {
    if i == 0 || j == 0 {
        return false;
    }
    let line_1 = match file_lines_1.get(i - 1) {
        Some(l) => l,
        None => return false,
    };
    let line_2 = match file_lines_2.get(j - 1) {
        Some(l) => l,
        None => return false,
    };
    line_1.eq(line_2)
}

/// Convenience function to check if left LCS grid value is higher than up value.
///
/// Returns:
///
///  - bool with condition result.
fn check_west_is_higher(g: &grid::LcsGrid, i: usize, j: usize) -> bool {
    if i == 0 || j == 0 {
        return false;
    }
    let i_equals_zero = i == 0;
    let grid_v_1 = g.cells[i][j - 1];
    let grid_v_2 = g.cells[i - 1][j];
    i_equals_zero || grid_v_1 >= grid_v_2
}

/// Convenience function to check if left LCS grid value is lower than up value.
///
/// Returns:
///
///  - bool with condition result.
fn check_north_is_higher(g: &grid::LcsGrid, i: usize, j: usize) -> bool {
    if j == 0 || i == 0 {
        return false;
    }
    let j_equals_zero = j == 0;
    let grid_v_1 = g.cells[i][j - 1];
    let grid_v_2 = g.cells[i - 1][j];
    j_equals_zero || grid_v_1 < grid_v_2
}

/// Recursive function to print files diff
pub fn print_diff(
    g: &grid::LcsGrid,
    file_lines_1: &[String],
    file_lines_2: &[String],
    i: usize,
    j: usize,
) {
    if check_last_diagonal_pos(file_lines_1, file_lines_2, i, j) {
        print_diff(g, file_lines_1, file_lines_2, i - 1, j - 1);
        println!("  {}", file_lines_1[i - 1]);
    } else if check_west_is_higher(g, i, j) {
        print_diff(g, file_lines_1, file_lines_2, i, j - 1);
        println!("> {}", file_lines_2[j - 1]);
    } else if check_north_is_higher(g, i, j) {
        print_diff(g, file_lines_1, file_lines_2, i - 1, j);
        println!("< {}", file_lines_1[i - 1]);
    } else {
        println!();
    }
}
