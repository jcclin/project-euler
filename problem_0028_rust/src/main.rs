const N: usize = 1001;
const N_BY_N: i32 = (N as i32) * (N as i32);


fn init_board() -> Vec<Vec<Option<i32>>>  {

    let mut board: Vec<Vec<Option<i32>>> = vec![];
    for _r in 0..N {
        let mut row: Vec<Option<i32>> = vec![];
        for _c in 0..N {
            row.push(None);
        }
        board.push(row)
    }
    return board;
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}


#[derive(Debug)]
struct PosAndDir {
    r: usize, c: usize, d: Dir,
}


impl PosAndDir {
    fn turn_right(&self) -> Self {
       match self.d {
           Dir::Up    => Self { r: (self.r    ), c: (self.c + 1), d: Dir::Right},
           Dir::Right => Self { r: (self.r + 1), c: (self.c    ), d: Dir::Down },
           Dir::Down  => Self { r: (self.r    ), c: (self.c - 1), d: Dir::Left },
           Dir::Left  => Self { r: (self.r - 1), c: (self.c    ), d: Dir::Up   },
       }
    }

    fn go_forward(&self) -> Self {
        match self.d {
            Dir::Up    => Self { r: (self.r - 1), c: (self.c    ), d: self.d},
            Dir::Right => Self { r: (self.r    ), c: (self.c + 1), d: self.d},
            Dir::Down  => Self { r: (self.r + 1), c: (self.c    ), d: self.d},
            Dir::Left  => Self { r: (self.r    ), c: (self.c - 1), d: self.d},
        }
     }
}


fn next_move(board: &mut Vec<Vec<Option<i32>>>, cur_pos_and_dir: PosAndDir) -> PosAndDir {

    let right_turn_move = cur_pos_and_dir.turn_right();
    if board[right_turn_move.r][right_turn_move.c].is_none() {
        return right_turn_move;
    }
    let go_forward_move = cur_pos_and_dir.go_forward();
    assert!(board[go_forward_move.r][go_forward_move.c].is_none());
    return go_forward_move;
}


fn main() {
    println!("Hello, world!");

    let mut board = init_board();
    let init_pos_and_dir = PosAndDir { r: N  / 2,  c: N / 2, d: Dir::Up };
    let mut cur_pos_and_dir = init_pos_and_dir;
    let mut s: i32 = 1;
    loop {
        board[cur_pos_and_dir.r][cur_pos_and_dir.c] = Some(s);
        s += 1;
        println!(".....{:?}", cur_pos_and_dir);
        if s > N_BY_N {
            break;
        }
        cur_pos_and_dir = next_move(&mut board, cur_pos_and_dir);
    }

    for row in &board {
        println!("{:?}", row);
    }

    let mut diag_sum: i32 = 0;
    for n in 0..N {
        diag_sum += board[n][n].unwrap();
    }
    for n in 0..N {
        diag_sum += board[n][N - 1 - n].unwrap();
    }
    diag_sum -= 1;
    println!("Diagonal Sum = {}", diag_sum);

}
