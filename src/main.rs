use std::{process::Command, fmt, collections::VecDeque, thread, time::Duration};
use console::{Term, Key};
use rand::Rng;

const RANDOM_STEPS: usize = 100_000;
const UP: u8 = 1;
const RIGHT: u8 = 2;
const DOWN: u8 = 3;
const LEFT: u8 = 4;
const MIN_LEVEL: usize = 2;
const MAX_LEVEL: usize = 16;
const DEFAULT_LEVEL: usize = 4;
const DEFAULT_AUTO_REFRAME: u64 = 100;

struct BarleyBreak {
    field: Vec<Vec<u8>>,
    position: Vec<u8>,
    size: usize,
    steps: u64,
    solved: bool,
    moving: u8,
}

#[derive(PartialEq)]
enum Mode {
    Game,
    ChangeLevel,
}

impl BarleyBreak {
    fn init(level: usize) -> Self {
        assert!(level <= MAX_LEVEL && level >= MIN_LEVEL, "Level is incorrect");
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
            moving: 0
        };
        while game.is_solved() {
            game.rand();
        }
        game.solved = false;
        game
    }


    fn rand(&mut self) {
        for _ in 0..RANDOM_STEPS {
            let direction = rand::thread_rng().gen_range(UP..=LEFT);
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
        true
    }

    fn check_solved(&mut self) {
        if self.is_solved() {
            self.solved = true;
        }
    }

    fn step(&mut self, direction: u8) {
        let new_position: u8;
        if direction == UP && self.position[0] / (self.size as u8) < (self.size - 1) as u8 {
            new_position = self.position[0] + self.size as u8;
        } else if direction == RIGHT && (self.position[0] % (self.size as u8)) > 0 {
            new_position = self.position[0] - 1;
        } else if direction == DOWN && (self.position[0] / (self.size as u8)) > 0 {
            new_position = self.position[0] - self.size as u8;
        } else if direction == LEFT && self.position[0] % (self.size as u8) < (self.size - 1) as u8 {
            new_position = self.position[0] + 1;
        } else {
            return;
        }
        self.field[(self.position[0] / self.size as u8) as usize][(self.position[0] % self.size as u8) as usize ] =
            self.field[(new_position / self.size as u8) as usize][(new_position % self.size as u8) as usize];
        self.position[self.field[(new_position / self.size as u8) as usize][(new_position % self.size as u8) as usize] as usize] =
            self.position[0];
        self.field[(new_position / self.size as u8) as usize][(new_position % self.size as u8) as usize] = 0;
        self.position[0] = new_position;
        if !self.solved {
            self.steps += 1;
            self.check_solved();
        }
    }

    fn reverse_step(direction: u8) -> u8 {
        (direction + 2) % 5 + (direction + 2) / 5
    }

    fn show_cell(&self, cell: u8) -> String {
        let character = " ";
        let space = (self.size * self.size - 1).to_string().len() + 1;
        if cell == 0 || (!self.solved && self.moving > 0 && self.moving == cell && self.steps % 5 == 0 ) {
            return character.repeat(space);
        } 
        let space_begin = (&space - &cell.to_string().len()) / 2; 
        let space_end = (&space - &cell.to_string().len()) / 2 + (&space - &cell.to_string().len()) % 2;
        character.repeat(space_begin) + &cell.to_string() + &character.repeat(space_end)
    }

    fn print(&self) {
        Command::new("clear").status().unwrap();
        println!("{}", self);
    }

    fn solve(&mut self) {
        for step in (MIN_LEVEL..=self.size).rev() {
            self.solve_by_layer(step);
        }
    }

    fn solve_by_layer(&mut self, layer: usize) {
        let mut order: Vec<u8> = Vec::new();
        for i in 0..layer {
            order.push((self.size - layer) as u8 * (self.size + 1) as u8 + i as u8 + 1);
        }
        for i in 1..layer {
            order.push(order[0] + (i * self.size) as u8);
        }
        self.solve_with_order(layer, order);
    }

    fn solve_with_order(& mut self, layer: usize, order: Vec<u8>) {
        let mut block = vec![false; self.size * self.size];
        for elem in order.iter() {
            if self.position[*elem as usize] != elem - 1 {
                self.moving = *elem;
                let mut final_position = elem - 1;
                let mut final_position_for_zero = 0;
                let mut final_steps = Vec::<u8>::new();
                if
                    layer > MIN_LEVEL &&
                    elem % self.size as u8 == 0 &&
                    !(self.position[0] == elem - 1 && self.position[*elem as usize] == elem + self.size as u8 - 1)
                {
                    final_position = elem + self.size as u8 * 2 - 1;
                    final_position_for_zero = elem + self.size as u8 - 1;
                    final_steps.push(3);
                    final_steps.push(2);
                    final_steps.push(1);
                    final_steps.push(4);
                    final_steps.push(1);
                    final_steps.push(2);
                    final_steps.push(3);
                    final_steps.push(3);
                    final_steps.push(4);
                    final_steps.push(1);

                } else if
                    layer > MIN_LEVEL &&
                    elem / self.size as u8 == self.size as u8 - 1 &&
                    !(self.position[0] == elem - 1 && self.position[*elem as usize] == *elem)
                {
                    final_position = elem + 1;
                    final_position_for_zero = *elem;
                    final_steps.push(2);
                    final_steps.push(3);
                    final_steps.push(4);
                    final_steps.push(1);
                    final_steps.push(4);
                    final_steps.push(3);
                    final_steps.push(2);
                    final_steps.push(2);
                    final_steps.push(1);
                    final_steps.push(4);
                }
                let path = self.find_shortest_way(*elem, final_position, layer, block.clone());

                for i in path.iter().rev() {
                    let zero_position = if *i == UP {
                        self.position[*elem as usize] as usize + self.size
                    } else if *i == RIGHT {
                        self.position[*elem as usize] as usize - 1
                    } else if *i == DOWN {
                        self.position[*elem as usize] as usize - self.size
                    } else {
                        self.position[*elem as usize] as usize + 1
                    };
                    let mut block_for_zero = block.clone();
                    block_for_zero[self.position[*elem as usize] as usize] = true;
                    let path_for_zero = self.find_shortest_way(
                        0,
                        zero_position as u8,
                        layer,
                        block_for_zero
                    );
                    for j in path_for_zero.iter().rev() {
                        self.step(*j);   
                        self.print();
                        thread::sleep(Duration::from_millis(DEFAULT_AUTO_REFRAME)); 
                    }
                    self.step(Self::reverse_step(*i));
                    self.print();
                    thread::sleep(Duration::from_millis(DEFAULT_AUTO_REFRAME));
                }
                if final_position_for_zero > 0 {
                    let mut block_for_zero = block.clone();
                    block_for_zero[self.position[*elem as usize] as usize] = true;
                    let path_for_zero = self.find_shortest_way(
                        0,
                        final_position_for_zero as u8,
                        layer,
                        block_for_zero
                    );
                    for j in path_for_zero.iter().rev() {
                        self.step(*j);   
                        self.print();
                        thread::sleep(Duration::from_millis(DEFAULT_AUTO_REFRAME)); 
                    }
                    for i in final_steps.iter() {
                        self.step(*i);   
                        self.print();
                        thread::sleep(Duration::from_millis(DEFAULT_AUTO_REFRAME)); 
                    }
                }
                self.moving = 0;
            }
            block[self.position[*elem as usize] as usize] = true;
        }
    }

    fn find_shortest_way(&self, cell: u8, position: u8, layer: usize, mut block: Vec<bool>) -> Vec<u8> {
        if self.position[cell as usize] == position {
            return vec![];
        }
        let mut path = Vec::<(u8, usize)>::new();
        let mut queue = VecDeque::<u8>::new();
        path.push((0, 0));
        queue.push_back(self.position[cell as usize]);
        let mut counter: usize = 0;
        let mut stop = false;
        loop {
            let pointer = queue.pop_front().unwrap();
            block[pointer as usize] = true;
            if
                pointer / (self.size as u8) < (self.size - 1) as u8 &&
                path[counter].0 != DOWN &&
                !block[pointer as usize + self.size]
            {
                if position == pointer + self.size as u8 {
                    stop = true;
                }
                queue.push_back(pointer + self.size as u8);
                path.push((UP, counter));
                if stop { break; }
            }
            if
                pointer % (self.size as u8) > (self.size - layer) as u8 &&
                path[counter].0 != LEFT &&
                !block[pointer as usize - 1]
            {
                if position == pointer - 1 {
                    stop = true;
                }
                queue.push_back(pointer - 1);
                path.push((RIGHT, counter));
                if stop { break; }
            }
            if
                pointer / (self.size as u8) > (self.size - layer) as u8 &&
                path[counter].0 != UP &&
                !block[pointer as usize - self.size]
            {
                if position == pointer - self.size as u8 {
                    stop = true;
                }
                queue.push_back(pointer - self.size as u8);
                path.push((DOWN, counter));
                if stop { break; }
            }
            if
                pointer % (self.size as u8) < (self.size - 1) as u8 &&
                path[counter].0 != RIGHT &&
                !block[pointer as usize + 1]
            {
                if position == pointer + 1 {
                    stop = true;
                }
                queue.push_back(pointer + 1);
                path.push((LEFT, counter));
                if stop { break; }
            }
            counter += 1;
        };
        let mut step = path[path.len() - 1];
        let mut steps = Vec::<u8>::new();
        while step.0 != 0 {
            steps.push(step.0);
            step = path[step.1 as usize];
        }
        steps
    }
}

impl fmt::Display for BarleyBreak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for step in 0..self.field.len() {
            for iter in 0..self.field[step].len() {
                res.push_str(&self.show_cell(self.field[step][iter]));
            }
            res.push_str("\n");
        }
        if self.solved {
            return write!(
                f,
                "{}\nSteps: {}\n\n !!!Congratulations!!! You solved this puzzle with {} steps\n\n{}\n{}\n{}\n{}\n{}",
                res,
                self.steps,
                self.steps,
                "UP DOWN LEFT RIGHT - control the game",
                "R - restart",
                "L - change level",
                "I - solve this puzzle",
                "Esc - exit"
            );
        }
        if self.moving > 0 {
            return write!(
                f,
                "{}\nSteps: {}\n\nMoving the number {}\n\n{}\n{}\n{}\n{}\n{}",
                res,
                self.steps,
                self.moving,
                "UP DOWN LEFT RIGHT - control the game",
                "R - restart",
                "L - change level",
                "I - solve this puzzle",
                "Esc - exit"
            );
        }
        write!(
            f,
            "{}\nSteps: {}\n\n\n\n{}\n{}\n{}\n{}\n{}",
            res,
            self.steps,
            "UP DOWN LEFT RIGHT - control the game",
            "R - restart",
            "L - change level",
            "I - solve this puzzle",
            "Esc - exit"
        )
    }
}

fn print_select_level(level: usize) {
    Command::new("clear").status().unwrap();
    for i in MIN_LEVEL..=MAX_LEVEL {
        if i == level {
            println!("[*] - {} * {}", i, i);
        } else {
            println!("[ ] - {} * {}", i, i);
        }
    }
}

fn main() {
    let mut level:usize = DEFAULT_LEVEL;
    let mut game = BarleyBreak::init(level);
    let mut mode: Mode = Mode::Game;

    let stdout = Term::buffered_stdout();

    game.print();

    loop {
        if let Ok(character) = stdout.read_key() {
            match character {
                Key::ArrowUp => {
                    if mode == Mode::Game {
                        game.step(UP);
                        game.print();
                    } else if mode == Mode::ChangeLevel && level > MIN_LEVEL {
                        level -= 1;
                        print_select_level(level);
                    }
                },
                Key::ArrowRight => {
                    if mode == Mode::Game {
                        game.step(RIGHT);
                        game.print();
                    }
                },
                Key::ArrowDown => {
                    if mode == Mode::Game {
                        game.step(DOWN);
                        game.print();
                    } else if mode == Mode::ChangeLevel && level < MAX_LEVEL {
                        level += 1;
                        print_select_level(level);
                    }
                },
                Key::ArrowLeft => {
                    if mode == Mode::Game {
                        game.step(LEFT);
                        game.print();
                    }
                },
                Key::Char('r') => {
                    if mode == Mode::Game {
                        game.restart();
                        game.print();
                    }
                },
                Key::Char('l') => {
                    if mode == Mode::Game {
                        mode = Mode::ChangeLevel;
                        print_select_level(level);
                    } else if mode == Mode::ChangeLevel {
                        mode = Mode::Game;
                        game.print();
                    }
                }
                Key::Char('i') => {
                    if mode == Mode::Game {
                        game.solve();
                    }
                }
                Key::Escape => {
                    if mode == Mode::Game {
                        break;
                    } else if mode == Mode::ChangeLevel {
                        mode = Mode::Game;
                        game.print();
                    }
                }
                Key::Enter => {
                    if mode == Mode::ChangeLevel {
                        mode = Mode::Game;
                        game = BarleyBreak::init(level);
                        game.print();
                    }
                }
                _ => {},
            }
        }
    }
}