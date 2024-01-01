use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::time::Instant;

// constant slice of tuples used to build series of Directions
const DIRECTIONS: &[(i32, i32)] = &[(1, 1), (1, -1), (-1, 1), (-1, -1)];

/// Describes the size of the knight's move. Given a square board and the
/// current position, the knight has a series of valid moves. The knight
/// hosts the ability to find the shortest path from the origin to the
/// oposite diagonal corner of the board.
#[derive(Debug, Default, Clone)]
struct Ability(i32, i32);
impl Ability {
    fn reverse(&self) -> Self {
        Ability(self.1, self.0)
    }
    // Combine the ability with a direction to create a move.
    fn moves(&self) -> Vec<Move> {
        let directions = Vec::from(DIRECTIONS).into_iter().map(Direction::from);
        if self.0 == self.1 {
            return directions.map(|d| Move::new(self, &d)).collect();
        }
        directions
            .clone()
            .map(|d| Move::new(self, &d))
            .chain(directions.map(|d| Move::new(&self.reverse(), &d)))
            .collect::<Vec<_>>()
    }
    // Find valid moves given the current position of the knight.
    fn valid_moves(&self, current: &Position, board_size: &Position) -> Vec<Position> {
        self.moves()
            .into_iter()
            .filter_map(|move_| {
                // Calculate the new position
                let new_r = current.0 + move_.0;
                let new_c = current.1 + move_.1;

                // Check if the new position is valid
                Position::try_from((new_r, new_c), board_size)
            })
            .collect()
    }
    pub fn find_shortest_path(&self, start: &Position, goal: &Position) -> Path {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parents = HashMap::new();

        queue.push_back(start.clone());
        visited.insert(start.clone());

        while let Some(current_pos) = queue.pop_front() {
            if current_pos == *goal {
                return Path::new(start, goal, parents);
            }

            for next_move in self.valid_moves(&current_pos, goal) {
                if !visited.contains(&next_move) {
                    queue.push_back(next_move.clone());
                    visited.insert(next_move.clone());
                    parents.insert(next_move, current_pos.clone());
                }
            }
        }

        Path::empty()
    }
}
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq)]
struct Position(i32, i32);
impl Position {
    fn is_valid(&self, board_size: &Position) -> bool {
        let origin = Position(1, 1);
        self.0 >= origin.0 && self.1 >= origin.1 && self.0 <= board_size.0 && self.1 <= board_size.1
    }
    fn try_from((a, b): (i32, i32), board_size: &Position) -> Option<Self> {
        let pos = Position(a, b);
        if pos.is_valid(board_size) {
            Some(pos)
        } else {
            None
        }
    }
}
/// A move is the application of a knight's ability to a direction.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Move(i32, i32);
impl Move {
    fn new(knight: &Ability, direction: &Direction) -> Self {
        Move(knight.0 * direction.0, knight.1 * direction.1)
    }
}
#[derive(Debug, Default, Clone)]
struct Direction(i32, i32);
impl From<(i32, i32)> for Direction {
    fn from((a, b): (i32, i32)) -> Self {
        Direction(a, b)
    }
}
#[derive(Debug, Default, Clone)]
struct Path(Vec<Position>);
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();
        for pos in &self.0 {
            path.push_str(&format!("({}, {})\n", pos.0, pos.1));
        }
        write!(f, "{}", path)
    }
}
impl Path {
    fn new(start: &Position, goal: &Position, parents: HashMap<Position, Position>) -> Self {
        let mut path = vec![goal.clone()];
        let mut current = goal;

        while current != start {
            current = parents.get(current).unwrap();
            path.push(current.clone());
        }

        path.reverse();
        Path(path)
    }
    fn empty() -> Self {
        Path(Vec::new())
    }
    // specialized output for the problem
    fn step_count(&self) -> i32 {
        if self.0.is_empty() {
            -1
        } else {
            (self.0.len() - 1) as i32
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Reporting
type Report = i32;

pub struct Reports {
    data: VecDeque<Report>,
    n: i32,
}
impl Reports {
    // convert the Vec<Report> to a report that includes
    // the results that apply to the mirrored knight (1,3 ~ 3,1).
    fn finalize(mut self) -> Self {
        let capacity = self.n * self.n;
        let dummy: Vec<i32> = vec![0i32; capacity as usize];
        let mut new_reports = VecDeque::from(dummy);
        for r in 1..=self.n {
            for c in 1..=self.n {
                if r <= c {
                    let Some(item) = self.data.pop_front() else {
                        panic!("no item")
                    };
                    let slot: usize = self.get_idx(r, c);
                    new_reports[slot] = item;
                } else {
                    let slot = self.get_idx(c, r);
                    let item = new_reports[slot];
                    let slot = self.get_idx(r, c);
                    new_reports[slot] = item;
                }
            }
        }
        Reports {
            data: new_reports,
            n: self.n,
        }
    }
    fn get_idx(&self, r: i32, c: i32) -> usize {
        ((r - 1) * self.n + (c - 1)) as usize
    }
    pub fn print(&self) {
        self.data.iter().enumerate().for_each(|(i, r)| {
            if i % self.n as usize == 0 {
                println!();
            }
            print!("{:3} ", r);
        });
    }
    // fn that returns a Vec<Vec<i32>> for the problem
    pub fn to_2dvec(mut self) -> Vec<Vec<i32>> {
        self.data
            .make_contiguous()
            .chunks(self.n as usize)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}

// uses a knight to generate a report
fn report(knight: &Ability, goal: &Position) -> Report {
    let path = knight.find_shortest_path(&Position(1, 1), goal);
    #[cfg(feature = "debug")]
    println!("knight: {:?} steps: {}", &knight, path.step_count());
    path.step_count()
}

/// generates a series of reports using a series of knights
/// starting with (1,1) and ending with (n-1, n-1). Errs when
/// n is not within the range of 5..=25.
pub fn run(n: i32) -> Result<Reports, Box<dyn Error>> {
    if !(5..=25).contains(&n) {
        return Err("n must be between 5 and 25".into());
    }
    let mut knights = Vec::new();
    let goal = Position(n, n);
    // only run unique knights (i.e., 1,3 and 3,1 are the same)
    for r in 1..n {
        for c in 1..n {
            if r <= c {
                knights.push(Ability(r, c));
            }
        }
    }
    let mut data = VecDeque::new();
    for knight in knights {
        #[cfg(feature = "debug")]
        println!("---------------\n🟢 knight: {:?}", &knight);
        data.push_back(report(&knight, &goal));
    }

    Ok(Reports { data, n: n - 1 })
}

// Specific to the problem
pub fn knights_on_board(n: i32) -> Vec<Vec<i32>> {
    match run(n) {
        Ok(reports) => reports.finalize().to_2dvec(),
        Err(e) => panic!("Error: {}", e),
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let reports = run(7)?;
    reports.finalize().print();

    Ok(())
}
