use crate::utils;

pub fn run() {
    let input = utils::read_input("input/day04");
    let test = utils::read_input("input/test");

    let result = solve(&input);
    println!(" {}", result);
}

pub fn solve1a(input: &str) -> i32 {
    let xmas = "XMAS";
    let g: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    fn woosh(grid: Vec<Vec<char>>, word: &str) -> i32 {
        let directions = [
            (0, 1),   // right
            (0, -1),  // left
            (1, 0),   // down
            (1, -1),  // down-left
            (1, 1),   // down-right
            (-1, -1), // up-left
            (-1, 0),  // up
            (-1, 1),  // up-right
        ];

        let r = grid.len();
        let c = grid[0].len();
        let mut count = 0;
        for i in 0..r {
            for j in 0..c {
                for (dx, dy) in directions.iter() {
                    if is_word(&grid, i, j, *dx, *dy, word) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    woosh(g, xmas)
}

fn is_word(grid: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize, word: &str) -> bool {
    let word_len = word.len() as isize;
    let mut current_x = x as isize;
    let mut current_y = y as isize;

    for i in 0..word_len {
        if current_x < 0
            || current_y < 0
            || current_x >= grid.len() as isize
            || current_y >= grid[0].len() as isize
        {
            return false;
        }

        if grid[current_x as usize][current_y as usize] != word.chars().nth(i as usize).unwrap() {
            return false;
        }

        current_x += dx;
        current_y += dy;
    }

    true
}

pub fn solve(input: &str) -> i32 {
    let g: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let r = g.len();
    let c = g[0].len();
    let mut count = 0;

    // Iterate through each cell and check for the 'X-MAS' pattern centered at (i, j)
    for i in 1..r - 1 {
        for j in 1..c - 1 {
            if xwoosh(&g, i, j) {
                count += 1;
            }
        }
    }

    count
}

fn xwoosh(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    // Check if the center of the 'X' has an 'A'
    if grid[x][y] != 'A' {
        return false;
    }

    if x >= 1 && y >= 1 && x + 1 < grid.len() && y + 1 < grid[0].len() {
        let top_left = grid[x - 1][y - 1];
        let bottom_right = grid[x + 1][y + 1];

        // Check top-right and bottom-left positions
        let top_right = grid[x - 1][y + 1];
        let bottom_left = grid[x + 1][y - 1];

        // Ensure that the opposite corners are different pairs
        if (top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M') {
            if (top_right == 'M' && bottom_left == 'S') || (top_right == 'S' && bottom_left == 'M')
            {
                return true;
            }
        }
    }

    false
}
