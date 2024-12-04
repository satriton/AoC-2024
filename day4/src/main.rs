use std::{
    cmp::{max, min},
    fs::read_to_string,
};

pub struct Grid {
    pub matrix: Vec<Vec<char>>,
    pub size_x: usize,
    pub size_y: usize,
}

fn main() {
    let lines: Vec<String> = read_to_string("day4/src/input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let grid = extract_data(lines);

    println! {"{:?}", find_all_xmas(&grid)}
    println! {"{:?}", find_all_x_shaped_mas(&grid)}
}

fn extract_data(lines: Vec<String>) -> Grid {
    let mut grid: Vec<Vec<char>> = vec![];

    for line in lines {
        grid.push(line.chars().collect());
    }

    Grid {
        size_x: grid[0].len(),
        size_y: grid.len(),
        matrix: grid,
    }
}

fn find_all_xmas(grid: &Grid) -> usize {
    let mut appearance = 0;
    for line in 0..grid.size_y {
        for column in 0..grid.size_x {
            if grid.matrix[line][column] == 'X' {
                appearance += explore_possibilities(&grid, column, line)
            }
        }
    }

    appearance
}

fn find_all_x_shaped_mas(grid: &Grid) -> usize {
    let mut appearance = 0;
    for line in 0..grid.size_y {
        for column in 0..grid.size_x {
            if grid.matrix[line][column] == 'A' && grid.is_mas_with_x_shape(column, line) {
                appearance += 1
            }
        }
    }

    appearance
}

fn explore_possibilities(grid: &Grid, x_start: usize, y_start: usize) -> usize {
    let mut string_to_test = vec![];

    string_to_test.extend(grid.get_column_strings(x_start, y_start));
    string_to_test.extend(grid.get_line_strings(x_start, y_start));
    string_to_test.extend(grid.get_diagonal_strings(x_start, y_start));

    let mut xmas = 0;
    for candidate in string_to_test {
        if candidate == "XMAS" {
            xmas += 1;
        }
    }
    xmas
}

impl Grid {
    fn get_line_strings(&self, x: usize, y: usize) -> Vec<String> {
        let mut right = String::new();
        for x in x..=min(x + 3, self.size_x - 1) {
            right.push(self.matrix[y][x]);
        }

        let mut left = String::new();
        for x in max(0, (x as i32 - 3) as usize)..=x {
            left.insert(0, self.matrix[y][x]);
        }
        return vec![left, right];
    }

    fn get_column_strings(&self, x: usize, y: usize) -> Vec<String> {
        let mut down = String::new();
        for y in y..=min(y + 3, self.size_y - 1) {
            down.push(self.matrix[y][x]);
        }

        let mut up = String::new();
        for y in max(0, (y as i32 - 3) as usize)..=y {
            up.insert(0, self.matrix[y][x]);
        }
        return vec![up, down];
    }

    fn get_diagonal_strings(&self, x: usize, y: usize) -> Vec<String> {
        let mut top_left = String::new();
        let mut top_right = String::new();
        let mut bottom_left = String::new();
        let mut bottom_right = String::new();

        for i in 0..=3 {
            self.matrix
                .get(y + i)
                .and_then(|row| row.get(x + i))
                .copied()
                .map(|ch| top_left.push(ch));

            self.matrix
                .get(y + i)
                .and_then(|row| row.get(x.wrapping_sub(i)))
                .copied()
                .map(|ch| top_right.push(ch));

            self.matrix
                .get(y.wrapping_sub(i))
                .and_then(|row| row.get(x.wrapping_sub(i)))
                .copied()
                .map(|ch| bottom_left.push(ch));

            self.matrix
                .get(y.wrapping_sub(i))
                .and_then(|row| row.get(x + i))
                .copied()
                .map(|ch| bottom_right.push(ch));
        }

        return vec![top_left, top_right, bottom_left, bottom_right];
    }

    fn is_mas_with_x_shape(&self, x: usize, y: usize) -> bool {
        let mut go_left = String::new();
        let mut go_right = String::new();
        for i in 0..=2 {
            self.matrix
                .get(y.wrapping_sub(i).wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_add(i.wrapping_sub(1))))
                .copied()
                .map(|ch| go_right.push(ch));

            self.matrix
                .get(y.wrapping_sub(i).wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_sub(i.wrapping_sub(1)))) //x-i-1
                .copied()
                .map(|ch| go_left.push(ch));
        }

        if (go_right == "MAS" || go_right == "SAM") && (go_left == "MAS" || go_left == "SAM") {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod test {
    use crate::{find_all_x_shaped_mas, find_all_xmas, Grid};

    #[test]
    fn should_find_xmas_in_lines() {
        let grid = Grid {
            matrix: vec![
                vec!['Q', 'X', 'M', 'A', 'S'],
                vec!['Q', 'X', 'L', 'A', 'S'],
                vec!['Q', 'S', 'A', 'M', 'X'],
            ],
            size_x: 5,
            size_y: 3,
        };

        assert_eq!(2, find_all_xmas(&grid));
    }

    #[test]
    fn should_find_xmas_in_columns() {
        let grid = Grid {
            matrix: vec![
                vec!['Q', 'X', 'M', 'S', 'S'],
                vec!['Q', 'M', 'L', 'A', 'S'],
                vec!['Q', 'A', 'A', 'M', 'X'],
                vec!['Q', 'S', 'A', 'X', 'X'],
            ],
            size_x: 5,
            size_y: 4,
        };

        assert_eq!(2, find_all_xmas(&grid));
    }

    #[test]
    fn should_find_xmas_in_diagonals() {
        let grid = Grid {
            matrix: vec![
                vec!['X', 'S', '.', 'S', 'X'],
                vec!['.', 'M', 'A', 'M', '.'],
                vec!['.', 'M', 'A', 'M', '.'],
                vec!['X', 'S', '.', 'S', 'X'],
            ],
            size_x: 5,
            size_y: 4,
        };

        assert_eq!(4, find_all_xmas(&grid));
    }

    #[test]
    fn should_find_x_shaped_mas() {
        let grid = Grid {
            matrix: vec![
                vec!['M', '.', 'S', '.', 'M'],
                vec!['.', 'A', '.', 'A', '.'],
                vec!['M', 'S', 'S', 'S', 'M'],
                vec!['.', '.', 'A', '.', '.'],
                vec!['.', 'M', '.', 'M', '.'],
            ],
            size_x: 5,
            size_y: 5,
        };

        assert_eq!(3, find_all_x_shaped_mas(&grid));
    }
}
