#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;
use crate::user_lib::console::getchar;

///
/// Dinosaur Game in Rust and GardenerOS
///     --- Made by YXHXianYu (github.com/YXHXianYu/GardenerOS)
///
/// Hope you enjoy this game and this CODE!
///

// === Constants ===
const IMAGE_WIDTH:  usize = 50;
const IMAGE_HEIGHT: usize = 5;
const MAP_LENGTH:   usize = 150;
const OBSTACLE_NUM: usize = 10;

// === Utility Functions ===
fn clear() {
    for i in 0..30 {
        println!("                                                  ");
    }
}
fn recursive_fibonacci(x: u64) -> u64 {
    if x == 0 { 0 }
    else if x == 1 { 1 }
    else if x == 2 { 2 }
    else { recursive_fibonacci(x - 1) + recursive_fibonacci(x - 2) }
}
fn sleep() -> u64 {
    recursive_fibonacci(32)
}
fn usize_sub(v1: usize, v2: usize) -> usize {
    if v1 < v2 {
        0
    } else {
        v1 - v2
    }
}

// === Main Functions ===
fn print(img: &[[u8; IMAGE_WIDTH]; IMAGE_HEIGHT]) {
    clear();

    for _ in 0..IMAGE_WIDTH+2 { print!("="); } print!("\n");
    for i in (0..IMAGE_HEIGHT).rev() {
        print!("=");
        for j in 0..IMAGE_WIDTH { print!("{}", img[i][j] as char); }
        print!("=\n");
    }
    for _ in 0..IMAGE_WIDTH+2 { print!("="); } print!("\n");
}

// === World ===
struct World {
    obstacles: [(usize, usize); OBSTACLE_NUM],
    running: bool,
}
impl World {
    pub fn new() -> World {
        World {
            // OBSTACLE_NUM = 10
            obstacles: [
                (10, 0),
                (20, 0),
                (40, 0),
                (50, 1),
                (60, 0),
                (65, 0),
                (70, 0),
                (90, 1),
                (110, 2),
                (130, 2),
            ],
            running: true,
        }
    }
    pub fn flush(&mut self, player: &mut Player, input: usize, img: &mut [[u8; IMAGE_WIDTH]; IMAGE_HEIGHT]) {
        // === calc ===

        player.x += 1;
        match input {
            5 => player.y = 1,
            4 => player.y = 2,
            3 => player.y = 3,
            2 => player.y = 2,
            1 => player.y = 1,
            0 => player.y = 0,
            _ => panic!(),
        }

        let tx = player.x % MAP_LENGTH;
        let ty = player.y;
        for i in 0..OBSTACLE_NUM {
            if tx == self.obstacles[i].0 && ty <= self.obstacles[i].1 {
                self.running = false;
            }
        }

        // === render ===
        
        for i in 0..IMAGE_WIDTH {
            for j in 0..IMAGE_HEIGHT {
                img[j][i] = b' ';
            }
        }

        let tx = (player.x + MAP_LENGTH) % MAP_LENGTH;
        for i in 0..IMAGE_WIDTH {
            let cx = (player.x + MAP_LENGTH - 5 + i) % MAP_LENGTH;

            let mut obs: i32 = -1;
            for j in 0..OBSTACLE_NUM {
                if cx == self.obstacles[j].0 {
                    obs = self.obstacles[j].1 as i32;
                    break;
                }
            }

            if cx == tx {
                img[ty+1][i] = b'O';
                img[ty][i]   = b'I';
            }

            if obs >= 0 {
                for j in 0..=obs as usize {
                    img[j][i] = b'|';
                }
            }
        }
    }
}

// === Player ===
struct Player {
    pub x: usize,
    pub y: usize
}
impl Player {
    pub fn new() -> Player {
        Player { x: 0, y: 0 }
    }
}


#[no_mangle]
fn main() -> i32 {

    let mut count = 0;
    let mut count_limit = 1000;

    let mut img = [[b' '; IMAGE_WIDTH]; IMAGE_HEIGHT];

    let mut player = Player::new();
    let mut world = World::new();
    let mut input = 0;
    
    loop {

        let input_raw = getchar();
        if input > 0 {
            input -= 1;
        } else {
            input = match input_raw {
                b' ' => 5,
                _ => 0,
            };
        }

        world.flush(&mut player, input, &mut img);

        if count > 0 { sleep(); }

        print(&img);

        count += 1;
        if world.running == false || count >= count_limit { break; }
    }

    0
}
