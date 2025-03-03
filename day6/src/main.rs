use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let lab: Vec<Vec<char>> = read_to_string("day6/src/input")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println! {"{:?}", patroled_space(&Lab{lab})};
}

fn patroled_space(lab: &Lab) -> usize {
    let mut guard = lab.find_guard();

    while !guard.is_outside_lab {
        guard.r#move(lab);
    }

    guard.visited_space.len()
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug, PartialEq)]
struct Guard {
    direction: Direction,
    position: Position,
    is_outside_lab: bool,
    visited_space: HashSet<Position>,
}

pub struct Lab {
    lab: Vec<Vec<char>>,
}

impl Lab {
    fn find_guard(&self) -> Guard {
        for (y, row) in self.lab.iter().enumerate() {
            for (x, space) in row.iter().enumerate() {
                if space == &'^' {
                    return Guard {
                        direction: Direction::UP,
                        position: Position { x, y },
                        is_outside_lab: false,
                        visited_space: HashSet::from([Position { x, y }]),
                    };
                }
            }
        }

        panic!("No guard");
    }

    fn is_inside_lab(&self, (x, y): (isize, isize)) -> bool {
        0 <= x && (x as usize) < self.lab[0].len() && 0 <= y && (y as usize) < self.lab.len()
    }

    fn get_space(&self, (x, y): (usize, usize)) -> Option<&char> {
        self.lab.get(y).map(|line| line.get(x)).flatten()
    }
}

impl Guard {
    fn r#move(&mut self, lab: &Lab) {
        let (dx, dy) = Direction::movement_offset(&self.direction);

        let new_x = self.position.x as isize + dx;
        let new_y = self.position.y as isize + dy;

        if !lab.is_inside_lab((new_x, new_y)) {
            self.is_outside_lab = true;
            return;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        if lab.get_space((new_x, new_y)) != Some(&'#') {
            let new_pos = Position { x: new_x, y: new_y };
            self.position = new_pos.clone();
            self.visited_space.insert(new_pos);
        } else {
            self.direction = self.direction.turn();
            self.r#move(lab);
        }
    }
}

impl Direction {
    fn turn(&self) -> Self {
        match *self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
        }
    }

    fn movement_offset(&self) -> (isize, isize) {
        match self {
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{Direction, Guard, Lab, Position};

    #[test]
    fn should_extract_guard_position() {
        let lab = Lab {
            lab: vec![
                vec!['#', '.', '.', '.', '.'],
                vec!['.', '.', '#', '^', '.'],
                vec!['#', '.', '.', '#', '.'],
            ],
        };

        let guard = lab.find_guard();

        assert_eq!(
            guard,
            Guard {
                direction: Direction::UP,
                position: Position { x: 3, y: 1 },
                is_outside_lab: false,
                visited_space: HashSet::from([Position { x: 3, y: 1 }]),
            }
        );
    }

    #[test]
    fn should_move_up_until_obstacle() {
        // Given
        let lab = Lab {
            lab: vec![
                vec!['#', '#', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.'],
                vec!['#', '^', '.', '#', '.'],
            ],
        };
        let mut guard = Guard {
            direction: Direction::UP,
            position: Position { x: 1, y: 2 },
            is_outside_lab: false,
            visited_space: HashSet::new(),
        };

        // When
        guard.r#move(&lab);
        guard.r#move(&lab);

        // Then
        assert_eq!(guard.position.y, 1);
    }

    #[test]
    fn should_turn_right_if_obstacle_and_move() {
        // Given
        let lab = Lab {
            lab: vec![
                vec!['#', '#', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.'],
                vec!['#', '^', '.', '#', '.'],
            ],
        };
        let mut guard = Guard {
            direction: Direction::UP,
            position: Position { x: 1, y: 2 },
            is_outside_lab: false,
            visited_space: HashSet::new(),
        };

        // When
        guard.r#move(&lab);
        guard.r#move(&lab);

        // Then
        assert_eq!(guard.position, Position { x: 2, y: 1 });
    }

    #[test]
    fn should_detect_when_going_out_of_lab_upside() {
        // Given
        let lab = Lab {
            lab: vec![vec!['#', '^', '.', '.']],
        };
        let mut guard = Guard {
            direction: Direction::UP,
            position: Position { x: 1, y: 0 },
            is_outside_lab: false,
            visited_space: HashSet::new(),
        };

        // When
        guard.r#move(&lab);

        // Then
        assert!(guard.is_outside_lab);
    }

    #[test]
    fn should_detect_when_going_out_of_lab_rightside() {
        // Given
        let lab = Lab {
            lab: vec![vec!['#', '.', '.', '.'], vec!['#', '.', '.', '>']],
        };
        let mut guard = Guard {
            direction: Direction::RIGHT,
            position: Position { x: 3, y: 1 },
            is_outside_lab: false,
            visited_space: HashSet::new(),
        };

        // When
        guard.r#move(&lab);

        // Then
        assert!(guard.is_outside_lab);
    }

    #[test]
    fn should_store_visited_space() {
        // Given
        let lab = Lab {
            lab: vec![
                vec!['#', '#', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.'],
                vec!['#', '^', '.', '#', '.'],
            ],
        };
        let mut guard = Guard {
            direction: Direction::UP,
            position: Position { x: 1, y: 2 },
            is_outside_lab: false,
            visited_space: HashSet::from([Position { x: 1, y: 2 }]),
        };

        // When
        guard.r#move(&lab);
        guard.r#move(&lab);

        // Then
        assert_eq!(
            guard.visited_space,
            HashSet::from([
                Position { x: 1, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 2, y: 1 }
            ])
        );
    }
}
