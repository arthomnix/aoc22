// cursed rust do-while loop
macro_rules! do_while {
    (do $body:block while $cond:expr;) => {
        while { $body; $cond } {}
    };
}

#[derive(PartialEq, Debug)]
enum BoardTile {
    Open,
    Wall,
    OutOfBounds,
}

use BoardTile::*;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

use Direction::*;

impl Direction {
    fn turn_clockwise(self) -> Self {
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn turn_anticlockwise(self) -> Self {
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }
}

#[derive(Debug)]
enum TurnDirection {
    TurnLeft,
    TurnRight,
}

use TurnDirection::*;

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Turn(TurnDirection),
}

use Instruction::*;

#[derive(Debug)]
struct BoardState {
    board: Vec<Vec<BoardTile>>,
    pos: (usize, usize),
    direction: Direction,
    instructions: Vec<Instruction>,
}

impl BoardState {
    fn new(input: String) -> Self {
        let mut i = input.split("\n\n");
        let mut board: Vec<Vec<BoardTile>> = i
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                l
                    .chars()
                    .map(|c| match c {
                        '.' => Open,
                        '#' => Wall,
                        ' ' => OutOfBounds,
                        _ => panic!("invalid tile in board"),
                    })
                    .collect::<Vec<BoardTile>>()
            })
            .collect();

        let max_len = board.iter().map(|l| l.len()).max().unwrap();
        board = board.into_iter().map(|mut l| {
            while l.len() < max_len {
                l.push(OutOfBounds);
            }
            l
        }).collect();

        let regex = regex::Regex::new(r"(?P<move>\d+)|(?P<turn>L|R)").unwrap();
        let instructions: Vec<Instruction> = regex.captures_iter(i.next().unwrap()).map(|caps| {
            if let Some(m) = caps.name("move") {
                Move(m.as_str().parse().unwrap())
            } else if let Some(m) = caps.name("turn") {
                Turn(match m.as_str() {
                    "L" => TurnLeft,
                    "R" => TurnRight,
                    _ => unreachable!(),
                })
            } else {
                unreachable!();
            }
        }).collect();

        let mut pos: (usize, usize) = (0, 0);
        while board[pos.1][pos.0] != Open {
            pos.0 += 1;
        }

        Self {
            board,
            pos,
            direction: Right,
            instructions,
        }
    }

    fn exec(&mut self) {
        for instruction in self.instructions.iter() {
            match instruction {
                Move(n) => {
                    for _ in 0..*n {
                        let mut new_pos = self.pos;
                        do_while! {
                            do {
                                match self.direction {
                                    Right => new_pos.0 = (new_pos.0 as isize + 1).rem_euclid(self.board[self.pos.1].len() as isize) as usize,
                                    Down => new_pos.1 = (new_pos.1 as isize + 1).rem_euclid(self.board.len() as isize) as usize,
                                    Left => new_pos.0 = (new_pos.0 as isize - 1).rem_euclid(self.board[self.pos.1].len() as isize) as usize,
                                    Up => new_pos.1 = (new_pos.1 as isize - 1).rem_euclid(self.board.len() as isize) as usize,
                                }
                            } while self.board[new_pos.1][new_pos.0] == OutOfBounds;
                        }

                        if self.board[new_pos.1][new_pos.0] != Wall {
                            self.pos = new_pos;
                        } else {
                            break;
                        }
                    }
                },
                Turn(d) => match d {
                    TurnLeft => self.direction = self.direction.turn_anticlockwise(),
                    TurnRight=> self.direction = self.direction.turn_clockwise(),
                },
            }
        }
    }

    fn cube_map(&self, pos: (isize, isize)) -> ((usize, usize), Direction) {
        if pos.0 >= 0 && pos.0 < self.board[0].len() as isize
            && pos.1 >= 0 && pos.1 < self.board.len() as isize
            && self.board[pos.1 as usize][pos.0 as usize] != OutOfBounds {
            ((pos.0 as usize, pos.1 as usize), self.direction)
        } else {
            let r = match (pos, self.direction) {
                // ugly hardcoded data
                ((49, 99), Up) => ((50, 99), Right),
                ((49, 99), Left) => ((40, 100), Down),
                ((50, 150), Right) => ((50, 149), Up),
                ((50, 150), Down) => ((49, 150), Left),
                ((100, 50), Right) => ((100, 49), Up),
                ((100, 50), Down) => ((99, 50), Left),
                ((50..=99, -1), _) => ((0, (pos.0 + 100) as usize), Right),
                ((-1, 150..=199), _) => (((pos.1 - 100) as usize, 0), Down),
                ((100..=149, -1), _) => (((pos.0 - 100) as usize, 199), Up),
                ((0..=49, 200), _) => (((pos.0 + 100) as usize, 0), Down),
                ((100..=149, 50), _) => ((99, (pos.0 - 50) as usize), Left),
                ((100, 50..=99), _) => (((pos.1 + 50) as usize, 49), Up),
                ((0..=49, 99), _) => ((50, (pos.0 + 50) as usize), Right),
                ((49, 50..=99), _) => (((pos.1 - 50) as usize, 100), Down),
                ((-1, 100..=149), _) => ((50, -(pos.1 - 149) as usize), Right),
                ((49, 0..=49), _) => ((0, -(pos.1 - 149) as usize), Right),
                ((50, 150..=199), _) => (((pos.1 - 100) as usize, 149), Up),
                ((50..=99, 150), _) => ((49, (pos.0 + 100) as usize), Left),
                ((100, 100..=149) ,_) => ((149, -(pos.1 - 149) as usize), Left),
                ((150, 0..=49), _) => ((99, -(pos.1 - 149) as usize), Left),
                _ => panic!("invalid cube mapping {:?} old pos {:?}", pos, self.pos),
            };
            // sanity check
            assert_ne!(self.board[r.0.1][r.0.0], OutOfBounds);
            r
        }
    }

    fn exec_cube(&mut self) {
        for instruction in self.instructions.iter() {
            match instruction {
                Move(n) => {
                    for _ in 0..*n {
                        let (new_pos, new_dir) = self.cube_map(match self.direction {
                            Right => (self.pos.0 as isize + 1, self.pos.1 as isize),
                            Down => (self.pos.0 as isize, self.pos.1 as isize + 1),
                            Left => (self.pos.0 as isize - 1, self.pos.1 as isize),
                            Up => (self.pos.0 as isize, self.pos.1 as isize - 1),
                        });
                        if self.board[new_pos.1][new_pos.0] != Wall {
                            self.pos = new_pos;
                            self.direction = new_dir;
                        } else {
                            break;
                        }
                    }
                },
                Turn(d) => match d {
                    TurnLeft => self.direction = self.direction.turn_anticlockwise(),
                    TurnRight=> self.direction = self.direction.turn_clockwise(),
                },
            }
        }
    }
}

pub(crate) fn part1(input: String) {
    let mut state = BoardState::new(input);
    state.exec();
    println!("{}, end pos {:?} facing {:?}", 1000 * (state.pos.1 + 1) + 4 * (state.pos.0 + 1) + match state.direction {
        Right => 0,
        Down => 1,
        Left => 2,
        Up => 3,
    }, state.pos, state.direction);
}

pub(crate) fn part2(input: String) {
    let mut state = BoardState::new(input);

    // sanity checks
    assert_eq!(state.cube_map((-1, 150)), ((50, 0), Down));
    assert_eq!(state.cube_map((50, -1)), ((0, 150), Right));
    assert_eq!(state.cube_map((-1, 149)), ((50, 0), Right));
    assert_eq!(state.cube_map((49, 0)), ((0, 149), Right));
    assert_eq!(state.cube_map((48, 99)), ((50, 98), Right));
    assert_eq!(state.cube_map((49, 98)), ((48, 100), Down));
    assert_eq!(state.cube_map((0, 200)), ((100, 0), Down));
    assert_eq!(state.cube_map((100, -1)), ((0, 199), Up));
    assert_eq!(state.cube_map((50, 151)), ((51, 149), Up));
    assert_eq!(state.cube_map((51, 150)), ((49, 151), Left));
    assert_eq!(state.cube_map((100, 51)), ((101, 49), Up));
    assert_eq!(state.cube_map((101, 50)), ((99, 51), Left));
    assert_eq!(state.cube_map((100, 100)), ((149, 49), Left));
    assert_eq!(state.cube_map((150, 49)), ((99, 100), Left));

    state.exec_cube();
    println!("{}, end pos {:?} facing {:?}", 1000 * (state.pos.1 + 1) + 4 * (state.pos.0 + 1) + match state.direction {
        Right => 0,
        Down => 1,
        Left => 2,
        Up => 3,
    }, state.pos, state.direction);
}