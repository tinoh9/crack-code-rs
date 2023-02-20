use std::collections::HashSet;

fn get_index_vec_col_row(col: usize, cols: usize, row: usize) -> usize {
    (row * cols) + col
}

fn check_is_island(
    islands: &Vec<bool>,
    visited_island: &mut HashSet<usize>,
    col: usize,
    cols: usize,
    row: usize,
    rows: usize,
) -> bool {
    let vec_index = get_index_vec_col_row(col, cols, row);

    let is_island = islands[vec_index];

    if is_island && !visited_island.contains(&vec_index) {
        visited_island.insert(vec_index);
        if col > 0 {
            check_is_island(islands, visited_island, col - 1, cols, row, rows);
        }
        if col < cols - 1 {
            check_is_island(islands, visited_island, col + 1, cols, row, rows);
        }
        if row > 0 {
            check_is_island(islands, visited_island, col, cols, row - 1, rows);
        }
        if row < rows - 1 {
            check_is_island(islands, visited_island, col, cols, row + 1, rows);
        }
        return true;
    }
    false
}

fn num_of_islands(islands: Vec<bool>, cols: usize, rows: usize) -> usize {
    let mut visited_island = HashSet::<usize>::new();
    let mut count_island: usize = 0;
    for col in 0..cols {
        for row in 0..rows {
            if check_is_island(&islands, &mut visited_island, col, cols, row, rows) {
                count_island += 1;
            }
        }
    }
    count_island
}

#[cfg(test)]
mod test {
    use super::num_of_islands;
    #[test]
    fn test_1() {
        let islands = vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, true, false, false, false, true, false, false, false, false, false, false,
            false, true, false, false, false, true, false, false, false, false, true, false, false,
            true, false, false, false, true, false, false, false, false, true, false, false, true,
            false, false, false, true, false, false, false, false, true, false, false, true, false,
            false, true, true, true, false, false, false, false, false, false, true, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, true, false, false, false, false, false, false, true, false, false, false,
            false, true, true, true, true, false, false, false, true, true, true, true, false,
            true, true, true, true, false, false, false, false, false, false, true, false, true,
            true, true, true, false, false, false, false, false, false, true, false, true, true,
            true, true, false, false, false, false, false, false, false, false, true, true, true,
            true, false, true, false, false, false, true, false, false, false, false, false, false,
            true, true, true, false, false, true, true, false, false, false, false, false, false,
            true, false, false, false, true, true, false, false, false, false, false, false, false,
            false, false, false, true, true, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false,
        ];
        let input = num_of_islands(islands, 12, 20);
        assert_eq!(input, 6);
    }
}
