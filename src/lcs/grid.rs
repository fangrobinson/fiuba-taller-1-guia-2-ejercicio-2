//! Contains LCS grid implementation.
use std::cmp;

use super::lcs_errors::{CouldNotCreateLcsGrid, LcsValueNotFound};

/// represents grid to calculate Longest Common Secuence
#[derive(Debug)]
pub struct LcsGrid {
    pub cells: Vec<Vec<u32>>,
}

/// LCS Grid implementation.
impl LcsGrid {
    /// Private constructure of uninitialized LCS grid.
    fn new(i: usize, j: usize) -> LcsGrid {
        let mut grid: Vec<Vec<u32>> = Vec::<Vec<u32>>::new();
        for _i in 0..i + 1 {
            let value: Vec<u32> = vec![0; j + 1];
            grid.push(value);
        }
        LcsGrid { cells: grid }
    }

    /// Convinient function to access value of a [i, j] cell in LCS grid.
    ///
    /// Throws:
    ///  - LcsValueNotFound if invalid indexes were used.
    pub fn get_value_from(&self, i: usize, j: usize) -> Result<u32, LcsValueNotFound> {
        let row: &Vec<u32> = match self.cells.get(i) {
            Some(r) => r,
            None => return Err(LcsValueNotFound),
        };

        match row.get(j) {
            Some(v) => Ok(*v),
            None => Err(LcsValueNotFound),
        }
    }

    /// Convinient private function to save value in [i, j] cell in LCS grid.
    ///
    /// Does nothing on wrong indexes.
    ///
    /// This should only be used on LCS Grid initialization.
    fn save_value_to(&mut self, i: usize, j: usize, value: u32) {
        if let Some(cell) = self.cells.get_mut(i) {
            cell[j] = value;
        }
    }

    /// Convenient constructor of LCS Grid from two Vec<String>.
    pub fn from_vecs_of_strings(
        file_lines_1: &[String],
        file_lines_2: &[String],
    ) -> Result<LcsGrid, CouldNotCreateLcsGrid> {
        let mut grid = LcsGrid::new(file_lines_1.len(), file_lines_2.len());

        for i in 0..file_lines_1.len() {
            for j in 0..file_lines_2.len() {
                if file_lines_1.get(i).eq(&file_lines_2.get(j)) {
                    let value: u32 = match grid.get_value_from(i + 1, j + 1) {
                        Ok(v) => v + 1,
                        Err(LcsValueNotFound) => return Err(CouldNotCreateLcsGrid),
                    };
                    grid.save_value_to(i + 1, j + 1, value);
                } else {
                    let v1: u32 = match grid.get_value_from(i + 1, j) {
                        Ok(v) => v,
                        Err(LcsValueNotFound) => return Err(CouldNotCreateLcsGrid),
                    };
                    let v2: u32 = match grid.get_value_from(i, j + 1) {
                        Ok(v) => v,
                        Err(LcsValueNotFound) => return Err(CouldNotCreateLcsGrid),
                    };
                    grid.save_value_to(i + 1, j + 1, cmp::max(v1, v2));
                }
            }
        }
        Ok(grid)
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
            assert_eq!(row.len(), j + 1);
        }
    }

    #[test]
    fn create_from_vec_of_strings() {
        let mut lines1: Vec<String> = Vec::new();
        let mut lines2: Vec<String> = Vec::new();
        lines1.push(String::from("s"));
        lines2.push(String::from("s"));
        let grid = LcsGrid::from_vecs_of_strings(&lines1, &lines2).unwrap();
        assert_eq!(grid.cells.len(), lines1.len() + 1);
        for row in grid.cells {
            assert_eq!(row.len(), lines2.len() + 1);
        }
    }
}
