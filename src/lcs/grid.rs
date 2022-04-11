use std::cmp;

use super::lcs_errors::{LcsValueNotFound};

/// represents grid to calculate Longest Common Secuence
struct LcsGrid {
    cells: Vec<Vec<u32>>
}

impl LcsGrid {
    fn new(i: usize, j: usize) -> LcsGrid {
        let mut grid: Vec<Vec<u32>> = Vec::<Vec<u32>>::new();
        for i in 0..i+1 {
            let value: Vec<u32> = vec![0; j+1];
            grid.push(value);
        }
        LcsGrid {
            cells: grid
        }
    }

    fn get_value_from(&self, i: usize, j: usize) -> Result<u32, LcsValueNotFound> {
        let row: &Vec<u32> = match self.cells.get(i) {
            Some(r) => r,
            None => return Err(LcsValueNotFound),
        };

        match row.get(j) {
            Some(v) => return Ok(*v),
            None => return Err(LcsValueNotFound),
        }
    }

    fn save_value_to(& mut self, i: usize, j: usize, value: u32) {
        if let Some(cell) = self.cells.get_mut(i) { cell.insert(j, value) }
    }

    pub fn from_vecs_of_strings(file_lines_1: &[String], file_lines_2: &[String]) -> LcsGrid {
        let mut grid = LcsGrid::new(file_lines_1.len(), file_lines_2.len());

        for i in 0..file_lines_1.len() {
            for j in 0..file_lines_2.len() {
                if file_lines_1.get(i).eq(&file_lines_2.get(j)) {
                    let value: u32 = grid.get_value_from(i + 1, j + 1).unwrap() + 1;  
                    grid.save_value_to(i + 1, j + 1, value);
                } else {
                    let v1: u32 = grid.get_value_from(i + 1, j).unwrap();
                    let v2: u32 = grid.get_value_from(i, j + 1).unwrap();
                    grid.save_value_to(i + 1, j + 1, cmp::max(v1, v2));
                }
            }
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_lcs_grid() {
        let i = 1;
        let j = 2;
        let grid = LcsGrid::new(i, j);
        assert_eq!(grid.cells.len(), i + 1);
        for row in grid.cells {
            assert_eq!(row.len(), j+1);
        }
    }

    #[test]
    fn create_from_vec_of_strings() {
        let mut lines1: Vec<String> = Vec::new();
        let mut lines2: Vec<String> = Vec::new();
        lines1.push(String::from("s"));
        lines2.push(String::from("s"));
        let grid = LcsGrid::from_vecs_of_strings(&lines1, &lines2);
        assert_eq!(grid.cells.len(), lines1.len() + 1);
        for row in grid.cells {
            assert_eq!(row.len(), lines2.len() + 1);
        }
    }
}
