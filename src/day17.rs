#[derive(Clone)]
enum JetDirection {
    Left,
    Right,
}

use JetDirection::*;

fn parse_jets(input: String) -> Vec<JetDirection> {
    input.chars().map(|c| match c {
        '<' => Left,
        '>' => Right,
        _ => panic!("invalid char in jet pattern: {c}"),
    }).collect()
}

struct DefinitelyNotTetris {
    board: Vec<[bool; 7]>,
    pattern: Vec<JetDirection>,
    pattern_index: usize,
    piece_counter: usize,
}

impl DefinitelyNotTetris {
    fn new(pattern: Vec<JetDirection>) -> Self {
        Self {
            board: Default::default(),
            pattern,
            pattern_index: 0,
            piece_counter: 0,
        }
    }

    fn drop_piece(&mut self) {
        let rock_bott = self.board.len() + 3;

        let mut rock: Vec<(usize, usize)> = match self.piece_counter % 5 {
            0 => vec![(2, rock_bott), (3, rock_bott), (4, rock_bott), (5, rock_bott)],
            1 => vec![(3, rock_bott + 2), (2, rock_bott + 1), (3, rock_bott + 1), (4, rock_bott + 1), (3, rock_bott)],
            2 => vec![(2, rock_bott), (3, rock_bott), (4, rock_bott), (4, rock_bott + 1), (4, rock_bott + 2)],
            3 => vec![(2, rock_bott), (2, rock_bott + 1), (2, rock_bott + 2), (2, rock_bott + 3)],
            4 => vec![(2, rock_bott), (3, rock_bott), (2, rock_bott + 1), (3, rock_bott + 1)],
            _ => unreachable!(),
        };

        loop {
            match self.pattern[self.pattern_index] {
                Left => {
                    let mut can_move = true;
                    for coord in rock.iter() {
                        if coord.0 == 0 {
                            can_move = false;
                        } else if let Some(l) = self.board.get(coord.1) {
                            if l[coord.0 - 1] {
                                can_move = false;
                            }
                        }
                    }
                    if can_move {
                        rock = rock.iter().map(|(x, y)| (*x - 1, *y)).collect();
                    }
                },
                Right => {
                    let mut can_move = true;
                    for coord in rock.iter() {
                        if coord.0 == 6 {
                            can_move = false;
                        } else if let Some(l) = self.board.get(coord.1) {
                            if l[coord.0 + 1] {
                                can_move = false;
                            }
                        }
                    }
                    if can_move {
                        rock = rock.iter().map(|(x, y)| (*x + 1, *y)).collect();
                    }
                },
            }

            self.pattern_index += 1;
            if self.pattern_index == self.pattern.len() {
                self.pattern_index = 0;
            }

            let mut can_move = true;
            for coord in rock.iter() {
                if coord.1 == 0 {
                    can_move = false;
                } else if let Some(l) = self.board.get(coord.1 - 1) {
                    if l[coord.0] {
                        can_move = false;
                    }
                }
            }
            if can_move {
                rock = rock.iter().map(|(x, y)| (*x, *y - 1)).collect();
            } else {
                let max_y = rock.iter().map(|(_, y)| *y).max().unwrap();
                for _ in self.board.len()..=max_y {
                    self.board.push([false; 7]);
                }
                for (x, y) in rock {
                    self.board[y][x] = true;
                }
                break;
            }
        }
        self.piece_counter += 1;
    }
}

pub(crate) fn part1(input: String) {
    let mut not_tetris = DefinitelyNotTetris::new(parse_jets(input));
    for _ in 0..2022 {
        not_tetris.drop_piece();
    }
    println!("{}", not_tetris.board.len());
}

pub(crate) fn part2(input: String) {
    let jets = parse_jets(input);
    let mut not_tetris = DefinitelyNotTetris::new(jets.clone());

    for _ in 0..5 * jets.len() {
        not_tetris.drop_piece();
    }

    let f = not_tetris.board.len();

    let mut v: Vec<usize> = vec![];

    let (periodicity, p_sum, seq) = loop {
        let height = not_tetris.board.len();
        not_tetris.drop_piece();
        v.push(not_tetris.board.len() - height);

        if v.len() % 2 == 0 {
            let (p1, p2) = v.split_at(v.len() / 2);
            if p1 == p2 {
                break (v.len() / 2, p1.into_iter().sum::<usize>(), p2.to_vec());
            }
        }
    };

    let periods = (1000000000000 - 5 * jets.len()) / periodicity;
    let ext = (1000000000000 - 5 * jets.len()) % periodicity;
    println!("{}", f + periods * p_sum + seq[0..ext].into_iter().sum::<usize>());
}