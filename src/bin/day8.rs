use std::{fs, collections::HashMap};

#[derive(Clone, Debug, PartialEq, Copy)]
enum Visibility {
    Visible,
    Blocked,
    Unknown
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn step_forward(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }

    fn iter() -> impl Iterator<Item = Direction> {
        vec![Direction::North, Direction::East, Direction::South, Direction::West].into_iter()
    }
}


#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    visibilities: HashMap<Direction, Vec<Vec<Visibility>>>,
}

/// Forest of trees with visibilities
/// A tree is visible if it is taller than all trees in any direction or is on the boundary
/// A tree is blocked if it is shorter than at least one tree in all directions
impl Forest {
    fn new(trees: Vec<Vec<u8>>) -> Forest {
        let width = trees[0].len();
        let height = trees.len();
        let mut visible_north = vec![vec![Visibility::Unknown; width]; height];
        let mut visible_east = visible_north.clone();
        let mut visible_south = visible_north.clone();
        let mut visible_west = visible_north.clone();
        let width_idx = width - 1;
        let height_idx = height - 1;

        // Set boundary visible
        visible_north[0].iter_mut().for_each(|v| *v = Visibility::Visible);
        visible_east.iter_mut().for_each(|v| v[width_idx] = Visibility::Visible);
        visible_south[height_idx].iter_mut().for_each(|v| *v = Visibility::Visible);
        visible_west.iter_mut().for_each(|v| v[0] = Visibility::Visible);

        Forest {
            trees,
            height: width,
            width: height,
            visibilities: HashMap::from([
                (Direction::North, visible_north),
                (Direction::East, visible_east),
                (Direction::South, visible_south),
                (Direction::West, visible_west),
            ])
        }
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        for visibility in self.visibilities.values() {
            if let Visibility::Visible = visibility[y][x] {
                return true;
            }
        }

        false
    }

    fn is_valid_location(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn is_unknown(&self, x: usize, y: usize) -> bool {
        for visibility in self.visibilities.values() {
            if let Visibility::Unknown = visibility[y][x] {
                return true;
            }
        }
        false
    }

    fn calculate_visibilities(&mut self) {
        for direction in Direction::iter() {
            self._calculate_visibilities_direction(&direction);
        }
    }

    fn _calculate_visibilities_direction(&mut self, direction: &Direction) {
        // Sweep through opposite sides as starting locations for recursion
        let (x_range, y_range) = match direction {
            Direction::North => (0..self.width, self.height - 1.. self.height),
            Direction::East => (0..1, 0..self.height),
            Direction::South => (0..self.width, 0..1),
            Direction::West => (self.width - 1..self.width, 0..self.height),
        };
        for y in y_range {
            for x in x_range.clone() {
                if self.is_unknown(x, y) {
                    self._visibilities_helper_recurse(direction, x, y);
                }
            }
        }
    }

    fn _visibilities_helper_recurse(&mut self, direction: &Direction, x: usize, y: usize) {
        let height = self.trees[y][x];
        let (x, y) = direction.step_forward(x as i32, y as i32);
        if !self.is_valid_location(x as usize, y as usize) {
            return;
        }

        let current_height = self.trees[y as usize][x as usize];
        if current_height > height {
            self.visibilities.get_mut(direction).unwrap()[y as usize][x as usize] = Visibility::Visible;
            self._visibilities_helper_recurse(direction, x as usize, y as usize);
        } else {
            self.visibilities.get_mut(direction).unwrap()[y as usize][x as usize] = Visibility::Blocked;
        }
    }

    fn visualize_visibilities(&self) {
        for direction in Direction::iter() {
            println!("{:?}", direction);
            self._visualize_visibilities_direction(&direction);
        }
    }

    fn _visualize_visibilities_direction(&self, direction: &Direction) {
        let direction_rep = match direction {
            Direction::North => "^",
            Direction::East => ">",
            Direction::South => "v",
            Direction::West => ">",
        };
        for y in (0..self.height) {
            for x in 0..self.width {
                match self.visibilities[direction][y][x] {
                    Visibility::Visible => print!("{}", direction_rep),
                    Visibility::Blocked => print!("x"),
                    Visibility::Unknown => print!("?"),
                }
            }
            println!();
        }
    }

    fn get_num_visible(&self) -> usize {
        let mut num_visible = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.is_visible(x, y) {
                    num_visible += 1;
                    // println!("{} {}", x, y);
                }
                // if self.is_unknown(x, y) {
                //     println!("Unknown {} {}", x, y);
                // }
            }
        }

        num_visible
    }
}


// Basic test from problem
#[test]
fn test_basic_part_1() {
    let content = fs::read_to_string("day 8/test.txt").expect("Something went wrong reading the file");

    // Parse content into a vector of vector of chars
    let trees: Vec<Vec<u8>> = content.lines().map(|line|
        line.chars().map(|c|
            c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>())
        .collect();
    let mut forest = Forest::new(trees);
    // Calculate visibilities for each direction
    forest.calculate_visibilities();

    // Visualize visibilities
    forest.visualize_visibilities();

    assert_eq!(forest.get_num_visible(), 21);
}


fn main() {
    println!("Day 8");
    let content = fs::read_to_string("day 8/input.txt").expect("Something went wrong reading the file");

    // Parse content into a vector of vector of chars
    let trees: Vec<Vec<u8>> = content.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()).collect();
    let mut forest = Forest::new(trees);

    forest.calculate_visibilities();

    println!("Part 1: {}", forest.get_num_visible());

}