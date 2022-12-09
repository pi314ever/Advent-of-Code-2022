use std::{fs, collections::HashMap};

#[derive(Clone, Debug)]
enum Visibility {
    Visible,
    Blocked,
    Unknown
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn step_forward(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::North => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y - 1),
            Direction::West => (x - 1, y),
        }
    }
}

#[derive(Clone, Debug)]
struct Tree {
    x: usize,
    y: usize,
    visibility: Visibility
}

struct Forest {
    trees: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    visibilities: HashMap<Direction, Vec<Vec<Visibility>>>,
}

impl Forest {
    fn new(trees: Vec<Vec<u8>>) -> Forest {
        let height = trees[0].len();
        let width = trees.len();
        let mut visible_north = vec![vec![Visibility::Unknown; height]; width];
        let mut visible_east = visible_north.clone();
        let mut visible_south = visible_north.clone();
        let mut visible_west = visible_north.clone();
        let height_idx = height - 1;
        let width_idx = width - 1;

        // Set boundary visible
        visible_north[0].iter_mut().for_each(|v| *v = Visibility::Visible);
        visible_east.iter_mut().for_each(|v| v[height_idx] = Visibility::Visible);
        visible_south[width_idx].iter_mut().for_each(|v| *v = Visibility::Visible);
        visible_west.iter_mut().for_each(|v| v[0] = Visibility::Visible);

        Forest {
            trees,
            height,
            width,
            visibilities: HashMap::from([
                (Direction::North, visible_north),
                (Direction::East, visible_east),
                (Direction::South, visible_south),
                (Direction::West, visible_west),
            ])
        }
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let mut visible = false;
        for (direction, visibility) in &self.visibilities {
            if let Visibility::Visible = visibility[x][y] {
                visible = true;
            }
        }

        visible
    }

    fn is_visible_direction(&self, x: usize, y: usize, direction: Direction) -> bool {
        matches!(self.visibilities[&direction][x][y], Visibility::Visible)
    }

    fn is_blocked(&self, x: usize, y: usize) -> bool {
        if self.is_visible(x, y) {
            return false;
        }
        let mut blocked = false;
        for (direction, visibility) in &self.visibilities {
            if let Visibility::Blocked = visibility[x][y] {
                blocked = true;
            }
        }

        blocked
    }

    fn is_valid_location(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn is_blocked_direction(&self, x: usize, y: usize, direction: Direction) -> bool {
        matches!(self.visibilities[&direction][x][y], Visibility::Blocked)
    }

    fn is_unknown(&self, x: usize, y: usize) -> bool {
        let mut unknown = false;
        for (direction, visibility) in &self.visibilities {
            if let Visibility::Unknown = visibility[x][y] {
                unknown = true;
            }
        }

        unknown
    }

    fn is_unknown_direction(&self, x: usize, y: usize, direction: Direction) -> bool {
        matches!(self.visibilities[&direction][x][y], Visibility::Unknown)
    }

    fn set_visibility_direction(&mut self, x: usize, y: usize, direction: Direction){
        let visibility = self.visibilities.get_mut(&direction).unwrap();
        if let Visibility::Unknown = visibility[x][y] {

        }

    }
}


fn main() {
    println!("Day 8");
    let content = fs::read_to_string("day 8/input.txt").expect("Something went wrong reading the file");

    // Parse content into a vector of vector of chars
    let trees: Vec<Vec<u8>> = content.lines().map(|line| line.chars().map(|c| c as u8).collect::<Vec<u8>>()).collect();
    let mut forest = Forest::new(trees);

}