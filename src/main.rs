use std::{process::Command, fmt};
use console::{Term, Key};
use rand::Rng;

const RANDOM_STEPS: usize = 100000;
struct BarleyBreak {
    field: Vec<Vec<u8>>,
    position: Vec<u8>,
    size: usize,
    steps: u64,
    solved: bool,
}

impl BarleyBreak {
    fn init(level: usize) -> Self {
        assert!(level <= 16 && level >= 2, "Level is incorrect");
        let mut field: Vec<Vec<u8>> = vec![vec![0; level.into()]; level.into()];
        let mut position: Vec<u8> = vec![0; level * level];
        for step in 1..position.len() {
            field[(step - 1) / level][(step - 1) % level] = step as u8;
            position[step] = (step - 1) as u8;
        }
        position[0] = (level * level - 1) as u8;
        let mut game = Self {
            field,
            position,
            size: level,
            steps: 0,
            solved: false,
        };
        game.rand();
        game.solved = false;
        game
    }


    fn rand(&mut self) {
        for _ in 0..RANDOM_STEPS {
            let direction = rand::thread_rng().gen_range(1..=4);
            self.step(direction);
        }
        self.steps = 0;
    }

    fn restart(&mut self) {
        let new_game = Self::init(self.size);
        self.field = new_game.field;
        self.position = new_game.position;
        self.steps = new_game.steps;
        self.solved = false;
    }

    fn is_solved(&mut self) -> bool {
        for step in 1..self.position.len() {
            if self.position[step] != (step - 1) as u8 {
                return false;
            }
        }
        return true;
    }

    fn check_solved(&mut self) {
        if self.is_solved() {
            self.solved = true;
        }
    }

    // direction:
    // 1 - Up
    // 2 - Right
    // 3 - Down
    // 4 - Left
    fn step(&mut self, direction: u8) {
        let new_position: u8;
        if direction == 1 && self.position[0] / (self.size as u8) < (self.size - 1) as u8 {
            new_position = self.position[0] + self.size as u8;
        } else if direction == 2 && (self.position[0] % (self.size as u8)) > 0 {
            new_position = self.position[0] - 1;
        } else if direction == 3 && (self.position[0] / (self.size as u8)) > 0 {
            new_position = self.position[0] - self.size as u8;
        } else if direction == 4 && self.position[0] % (self.size as u8) < (self.size - 1) as u8 {
            new_position = self.position[0] + 1;
        } else {
            return;
        }
        self.field[(self.position[0] / self.size as u8) as usize][(self.position[0] % self.size as u8) as usize ] = self.field[(new_position / self.size as u8) as usize][(new_position % self.size as u8) as usize];
        self.position[self.field[(new_position / self.size as u8) as usize][(new_position % self.size as u8) as usize] as usize] = self.position[0];
        self.field[(new_position / self.size as u8) as usize][(new_position % self.size as u8) as usize] = 0;
        self.position[0] = new_position;
        if !self.solved {
            self.steps += 1;
            self.check_solved();
        }
    }
}

impl fmt::Display for BarleyBreak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for step in 0..self.field.len() {
            for iter in 0..self.field[step].len() {
                res.push_str(&*show_cell(self.field[step][iter], self.field.len()));
            }
            res.push_str("\n");
        }
        if self.solved {
            return write!(f, "{}\nSteps: {}\n\n!!!Congratulation!!! You solved this puzzle with {} steps\n\n{}", res, self.steps, self.steps, "UP DOWN LEFT RIGHT - control the game\nR - restart\nL - change hardness level\nI - solve this puzzle\nEsc - exit");
        }
        return write!(f, "{}\nSteps: {}\n\n\n\n{}", res, self.steps, "UP DOWN LEFT RIGHT - control the game\nR - restart\nL - change hardness level\nI - solve this puzzle\nEsc - exit");
    }
}

fn show_cell(cell: u8, level: usize) -> String {
    let space = (&level * &level - 1).to_string().len() + 1;
    if cell == 0 {
        return " ".repeat(space);
    }
    let space_begin = (&space - &cell.to_string().len()) / 2; 
    let space_end = (&space - &cell.to_string().len()) / 2 + (&space - &cell.to_string().len()) % 2;
    return " ".repeat(space_begin) + &cell.to_string() + &" ".repeat(space_end);
}

fn main() {
    let mut game = BarleyBreak::init(4);
    let stdout = Term::buffered_stdout();

    Command::new("clear").status().unwrap();
    println!("{}", game);

    loop {
        if let Ok(character) = stdout.read_key() {
            match character {
                Key::ArrowUp => {game.step(1); Command::new("clear").status().unwrap(); println!("{}", game);},
                Key::ArrowRight => {game.step(2); Command::new("clear").status().unwrap(); println!("{}", game);},
                Key::ArrowDown => {game.step(3); Command::new("clear").status().unwrap(); println!("{}", game);},
                Key::ArrowLeft => {game.step(4); Command::new("clear").status().unwrap(); println!("{}", game);},
                Key::Char('r') => {game.restart(); Command::new("clear").status().unwrap(); println!("{}", game);},
                _ => break,
            }
        }
    }
}