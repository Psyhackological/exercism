// These arrays represent the relative x and y coordinates of the 8 squares
// surrounding a square in a grid.
// The pattern follows a clockwise direction starting from the top-left square.
const DX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
const DY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];

// This function checks if a mine ('*') exists at a given x,y coordinate in the grid.
// It also makes sure that the x,y coordinate lies within the grid dimensions (w, h).
fn is_mine_at(result: &[Vec<char>], x: i32, y: i32, w: i32, h: i32) -> bool {
    x >= 0 && x < h && y >= 0 && y < w && result[x as usize][y as usize] == '*'
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Handle the case where the minefield is empty.
    if minefield.is_empty() {
        return Vec::new();
    }

    // Calculate the height (h) and width (w) of the minefield.
    let h = minefield.len();
    let w = minefield[0].len();

    // Convert the minefield to a 2D vector of characters for easier manipulation.
    let mut result = minefield
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Iterate over each cell in the grid.
    for i in 0..h {
        for j in 0..w {
            // If the current cell is a mine, skip to the next cell.
            if result[i][j] == '*' {
                continue;
            }

            let mut count = 0;
            // For each cell, we check its 8 surrounding cells for mines.
            for dir in 0..8 {
                // Calculate the coordinates of the neighboring cell.
                let ni = i as i32 + DX[dir];
                let nj = j as i32 + DY[dir];
                // If the neighboring cell contains a mine, increment the count.
                if is_mine_at(&result, ni, nj, w as i32, h as i32) {
                    count += 1;
                }
            }

            // If any mines were found in the surrounding cells,
            // replace the current cell with the count of mines.
            if count > 0 {
                result[i][j] = std::char::from_digit(count, 10).unwrap();
            }
        }
    }

    // Convert the 2D character vector back to a vector of strings before returning.
    result
        .into_iter()
        .map(|v| v.into_iter().collect())
        .collect()
}
