use crossterm::{
    cursor::MoveToPreviousLine,
    event::{poll, read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::Rng;
use std::{io::stdout, time::Duration};

struct GameData {
    board: [u32; 16],
    points: u32,
    nr_of_free_slots: u8,
}

impl GameData {
    fn init() -> GameData {
        let mut rng = rand::thread_rng();
        let rand_spot1 = rng.gen_range(0..16);
        let rand_spot2 = rng.gen_range(0..16);

        let mut to_return = GameData {
            board: [0; 16],
            points: 0,
            nr_of_free_slots: 16,
        };

        to_return.board[rand_spot1] = 2;
        to_return.board[rand_spot2] = 2;

        to_return.nr_of_free_slots = if rand_spot1 != rand_spot2 { 14 } else { 15 };

        to_return
    }

    fn fill_rand_value(&mut self) {
        let mut rng = rand::thread_rng();
        let mut rand_spot = rng.gen_range(0..self.nr_of_free_slots);

        for (i, val) in self.board.iter().enumerate() {
            if rand_spot == 0 && (*val == 0) {
                self.board[i] = if rng.gen_bool(0.6) { 2 } else { 4 };
                break;
            }
            if val.eq(&0) {
                rand_spot -= 1;
            }
        }
        self.nr_of_free_slots -= 1;
    }

    fn to_string(&self) -> String {
        let b = self.board.map(|e| {
            if e == 0 {
                String::from(" ")
            } else {
                e.to_string()
            }
        });

        format!(
            "_____________________
            \r|{:>4}|{:>4}|{:>4}|{:>4}|
            \r|{:>4}|{:>4}|{:>4}|{:>4}|
            \r|{:>4}|{:>4}|{:>4}|{:>4}|
            \r|{:>4}|{:>4}|{:>4}|{:>4}|
            \r---------------------
            \rPoints: {}\n",
            b[0],
            b[1],
            b[2],
            b[3],
            b[4],
            b[5],
            b[6],
            b[7],
            b[8],
            b[9],
            b[10],
            b[11],
            b[12],
            b[13],
            b[14],
            b[15],
            self.points
        )
    }

    fn refresh_and_print(&self) {
        execute!(stdout(), MoveToPreviousLine(7), Print(self.to_string())).unwrap();
    }

    fn lost_game(&mut self) -> bool {
        self.nr_of_free_slots == 0
    }

    fn up(&mut self) -> bool {
        let checks: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

        self.move_(checks, 4, false)
    }

    fn down(&mut self) -> bool {
        let checks: [usize; 12] = [12, 13, 14, 15, 8, 9, 10, 11, 4, 5, 6, 7];

        self.move_(checks, 4, true)
    }

    fn right(&mut self) -> bool {
        let checks: [usize; 12] = [3, 7, 11, 15, 2, 6, 10, 14, 1, 5, 9, 13];

        self.move_(checks, 1, true)
    }

    fn left(&mut self) -> bool {
        let checks: [usize; 12] = [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14];

        self.move_(checks, 1, false)
    }

    // return if a change has been made
    fn compare_slots(&mut self, a: usize, b: usize) -> bool {
        if self.board[a] == 0 {
            return false;
        }

        if self.board[b] == 0 {
            self.board[b] = self.board[a];
            self.board[a] = 0;
            return true;
        }

        if self.board[b] == self.board[a] {
            self.board[b] = 2 * self.board[b];
            self.board[a] = 0;
            self.nr_of_free_slots += 1;
            self.points += self.board[b];
            return true;
        }

        false
    }

    fn move_(&mut self, checks: [usize; 12], off: usize, diff: bool) -> bool {
        loop {
            let mut change = false;
            for i in checks {
                if self.compare_slots(if !diff { i + off } else { i.abs_diff(off) }, i) {
                    change = true;
                }
            }

            if !change {
                break;
            }
        }

        if self.lost_game() {
            return false;
        }
        self.fill_rand_value();
        true
    }
}

fn main() {
    let mut game = GameData::init();
    print!("{}", game.to_string());
    loop {
        enable_raw_mode().unwrap();

        if !poll(Duration::from_millis(1_000)).unwrap() {
            continue;
        }

        // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
        let event = read().unwrap();

        disable_raw_mode().unwrap(); // to aboid warning

        if let Event::Key(key_event) = event {
            let continue_ = match key_event.code {
                KeyCode::Char('a') => game.left(),
                KeyCode::Char('s') => game.down(),
                KeyCode::Char('w') => game.up(),
                KeyCode::Char('d') => game.right(),
                KeyCode::Left => game.left(),
                KeyCode::Down => game.down(),
                KeyCode::Up => game.up(),
                KeyCode::Right => game.right(),
                KeyCode::Esc => false,
                KeyCode::Char('c') => false,
                _ => continue,
            };

            game.refresh_and_print();

            if !continue_ {
                break;
            }
        }
    }
}
