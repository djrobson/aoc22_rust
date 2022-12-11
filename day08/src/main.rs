use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Vec<String> = io::stdin().lines().flatten().collect();

    let grid: Vec<Vec<u32>> =
        input.iter().map(|s| s.chars().flat_map(|c| c.to_digit(10)).collect()).collect();
    //println!("{:#?}", grid);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut total = (rows * 2) + ((cols - 2) * 2);
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if is_visible(&grid, row, col) {
                total += 1;
            }
        }
    }
    println!("{}", total);

    let mut best = 0;
    for row in 0..rows {
        for col in 0..cols {
            best = get_score(&grid, row, col).max(best);
        }
    }
    println!("{}", best);

    Ok(())
}

fn grid_col<T: Copy>(grid: &[Vec<T>], col: usize) -> Vec<T> {
    grid.iter().map(|v| v[col]).collect()
}

fn is_visible(grid: &[Vec<u32>], row: usize, col: usize) -> bool {
    let value = &grid[row][col];

    let r = &grid[row];
    if r[0..col].iter().all(|x| x < value) || r[col + 1..].iter().all(|x| x < value) {
        return true;
    }

    let c = grid_col(grid, col);
    if c[0..row].iter().all(|x| x < value) || c[row + 1..].iter().all(|x| x < value) {
        return true;
    }

    false
}

fn get_score(grid: &[Vec<u32>], row: usize, col: usize) -> usize {
    let value = &grid[row][col];
    let mut score = 1;

    let r = &grid[row];
    let mut trees = r[0..col].iter().rev().take_while(|&x| x < value).count();
    if trees < r[0..col].len() {
        trees += 1;
    }
    score *= trees;
    let mut trees = r[col + 1..].iter().take_while(|&x| x < value).count();
    if trees < r[col + 1..].len() {
        trees += 1;
    }
    score *= trees;

    let c = grid_col(grid, col);
    let mut trees = c[0..row].iter().rev().take_while(|&x| x < value).count();
    if trees < c[0..row].len() {
        trees += 1;
    }
    score *= trees;
    let mut trees = c[row + 1..].iter().take_while(|&x| x < value).count();
    if trees < c[row + 1..].len() {
        trees += 1;
    }
    score *= trees;

    score
}