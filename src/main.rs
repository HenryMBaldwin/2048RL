use rand::{rngs::ThreadRng, Rng};
use anyhow::{anyhow, Ok, Result};
mod GUI;
use GUI::*;
struct State {
    //contains actual numbers on board
    board: [[u16; 4]; 4],
    //contains whether or not each tile is mergable
    board_mergable: [[bool; 4]; 4],
    //current store
    score: u32,
}

impl State {
    
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut board = [[0; 4]; 4];

        let x0 = rng.gen_range(0..4);
        let y0 = rng.gen_range(0..4);

        let mut x1;
        let mut y1;
        loop {
            x1 = rng.gen_range(0..4);
            y1 = rng.gen_range(0..4);
            if x1 != x0 && y1 != y0 {
                break;
            }
        }

        board[x0][y0] = State::gen_tile_value();
        board[x1][y1] = State::gen_tile_value();

        State { board, board_mergable: [[true; 4]; 4], score: 0}
    }

    //prints the board for debugging
    fn print_board(&self) {
        for y in (0..4).rev() {
            for x in 0..4 {
                print!("{:4} ", self.board[x][y]);
            }
            println!();
        }
        println!("--------------------------")
    }
    //check for game over
    fn is_this_loss(&self) -> bool {
        for x in 0..4{
            for y in 0..4{
                //get value for every side
                let tile_val = self.get_tile_value(x, y);
                let up_tile_val = self.get_tile_value(x, y+1);
                let down_tile_val = self.get_tile_value(x, y-1);
                let right_tile_val = self.get_tile_value(x+1, y);
                let left_tile_val = self.get_tile_value(x-1, y);

                match tile_val{
                    0 => return false,
                    _ if tile_val == up_tile_val => return false,
                    _ if tile_val == down_tile_val => return false,
                    _ if tile_val == right_tile_val => return false,
                    _ if tile_val == left_tile_val => return false,
                    _ => {}
                }
            }
        }
        true
    }


    fn gen_tile_value() -> u16 {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..10) == 0 {
            4
        } else {
            2
        }
    }
    fn gen_new_tile(&mut self) -> Result<()> {
        let mut rng = rand::thread_rng();
        //check that there are zeros
        let mut zeros = false;
        'outer:for x in 0..4{
            for y in 0..4{
                if self.get_tile_value(x,y) == 0 {
                    zeros = true;
                    break 'outer;
                }

            }
        }
        if !zeros {
            return Ok(())
        }
        //this is terrible and I should probably be recording which tiles are actually zeros and randomly choosing between those. Hopefully this is fast enough that it doesn't matter.
        loop {
            let x = rng.gen_range(0..4);
            let y = rng.gen_range(0..4);

            if self.get_tile_value(x, y) == 0 {
                self.board[x][y] = State::gen_tile_value();
                break;
            }
        }
        Ok(())
    }
    // 0: up, 1: right, 2: down, 3: left
    fn slide_board(&mut self, direction: u8) -> Result<()>{
        

        let old_board = self.board.clone();
        //priotize merging at the direction then top to bottom if left or right and left to right if up or down
        match direction{
            //up
            0 => {
                    for x in 0..4 {
                        for y in (0..4).rev(){
                            self.slide_tile(direction, x, y)?;
                        }
                    }
            }
            //right
            1 => {
                    for y in (0..4).rev(){
                        for x in (0..4).rev(){
                            self.slide_tile(direction, x, y)?;
                        }
                    }
                },
            //down
            2 => {
                    for x in 0..4{
                        for y in 0..4{
                            self.slide_tile(direction, x, y)?;
                        }
                    }
            },
            //left
            3 => {
                    for y in (0..4).rev(){
                        for x in 0..4 {
                            self.slide_tile(direction, x, y)?;
                        }
                    }
            },
            _ => return Err(anyhow!(format!("Invalid direction: {}", direction))),
        }

        self.board_mergable = [[true; 4]; 4];
        if self.is_this_loss(){
            println!("This is loss")
        }
        else{
            if(old_board != self.board){ 
                self.gen_new_tile()?
            }
        }
        
        self.print_board();
        Ok(())

    }

    // 0: up, 1: right, 2: down, 3: left
    fn slide_tile(&mut self, direction: u8, x: usize, y: usize) -> Result<()>{

        
        
        let tile_value = self.get_tile_value(x, y);

        if &tile_value == &(0 as u16)|| &tile_value == &(1 as u16) {
            return Ok(());
        }

        if !(0..4).contains(&direction) {
            return Err(anyhow!(format!("Invalid direction {}", direction)));
        }

        
        match direction{
            //up
            0 => {
                let next_tile = self.get_tile_value(x, y+1);
                match next_tile{
                    0 => {
                        //move
                        self.board[x][y+1] = tile_value;
                        self.board[x][y] = 0;
                        self.print_board();
                        self.slide_tile(direction, x, y+1)?;
                    },
                    _ if next_tile == tile_value => {
                        //merge
                        //check mergable
                        if !self.board_mergable[x][y+1] {
                            //if not, we're done here
                            return Ok(())
                        }
                        self.board[x][y+1] = next_tile * 2;
                        self.board[x][y] = 0;
                        self.board_mergable[x][y+1] = false;

                        //we're done here
                        return Ok(())
                    },
                    _ => {
                        //we're done here
                        return Ok(())
                    }
                }
                Ok(())
            },
            //right
            1 => {
                let next_tile = self.get_tile_value(x+1, y);
                match next_tile{
                    0 => {
                        //move
                        self.board[x+1][y] = tile_value;
                        self.board[x][y] = 0;

                        self.slide_tile(direction, x+1, y)?;
                        Ok(())
                    },
                    _ if next_tile == tile_value => {
                        //merge
                        //check mergable
                        if !self.board_mergable[x+1][y] {
                            //if not, we're done here
                            return Ok(())
                        }
                        self.board[x+1][y] = next_tile * 2;
                        self.board[x][y] = 0;
                        self.board_mergable[x+1][y] = false;

                        Ok(())
                    },
                    _ => {
                        //we're done here
                        Ok(())
                    }
                }
            },
            //down
            2 => {
                let next_tile = self.get_tile_value(x, y-1);
                match next_tile {
                    0 => {
                        //move
                        self.board[x][y-1] = tile_value;
                        self.board[x][y] = 0;

                        self.slide_tile(direction, x, y-1)?;
                        Ok(())
                    },
                    _ if next_tile == tile_value => {
                        //merge
                        //check mergable
                        if !self.board_mergable[x][y-1] {
                            //if not, we're done here
                            return Ok(())
                        }
                        self.board[x][y-1] = next_tile * 2;
                        self.board[x][y] = 0;
                        self.board_mergable[x][y-1] = false;

                        Ok(())
                    },
                    _ => {
                        //we're done here
                        Ok(())
                    }
                }
            },
            //left
            3 => {
                let next_tile = self.get_tile_value(x-1, y);
                match(next_tile){
                    0 => {
                        //move
                        self.board[x-1][y] = tile_value;
                        self.board[x][y] = 0;

                        self.slide_tile(direction, x-1, y)?;
                        Ok(())
                    },
                    _ if next_tile == tile_value => {
                        //merge
                        //check mergable
                        if !self.board_mergable[x-1][y] {
                            //if not, we're done here
                            return Ok(())
                        }
                        self.board[x-1][y] = next_tile * 2;
                        self.board[x][y] = 0;
                        self.board_mergable[x-1][y] = false;

                        Ok(())
                    },
                    _ => {
                        //we're done here
                        Ok(())
                    }
                }
            },
            _ => return Err(anyhow!(format!("Invalid direction: {}", direction))),
        }

    }


    //gets value of a given tile, returns 1 for invalid tiles for convenience
    //this should handle underflows on x or y nicely when sliding tiles
    fn get_tile_value(&self, x: usize, y: usize) -> u16 {
        
        if !(0..4).contains(&x) || !(0..4).contains(&y) {
            return 1;
        }

        self.board[x][y]
    }


}

#[macroquad::main("rl2048")]
async fn main() {
    let state = State::new();
    let mut state = state;
    state.print_board();
    loop {
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::W) {
            state.slide_board(0).unwrap();
        }
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::D) {
            state.slide_board(1).unwrap();
        }
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::S) {
            state.slide_board(2).unwrap();
        }
        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::A) {
            state.slide_board(3).unwrap();
        }
        
        for y in (0..4) {
            for x in 0..4 {
                let tile_value = state.get_tile_value(x, y);
                let color = match tile_value {
                    0 => macroquad::color::WHITE,
                    2 => macroquad::color::RED,
                    4 => macroquad::color::BLUE,
                    8 => macroquad::color::GREEN,
                    16 => macroquad::color::YELLOW,
                    32 => macroquad::color::PURPLE,
                    64 => macroquad::color::ORANGE,
                    128 => macroquad::color::PINK,
                    256 => macroquad::color::GRAY,
                    512 => macroquad::color::BROWN,
                    1024 => macroquad::color::MAGENTA,
                    2048 => macroquad::color::GOLD,
                    _ => macroquad::color::BLACK,
                };
                macroquad::shapes::draw_rectangle(
                    x as f32 * 100.0,
                    (3-y) as f32 * 100.0,
                    100.0,
                    100.0,
                    color,
                );
                macroquad::text::draw_text(
                    &tile_value.to_string(),
                    x as f32 * 100.0 + 50.0,
                    (3-y) as f32 * 100.0 + 50.0,
                    25.0,
                    macroquad::color::BLACK,
                );
            }
        }
        macroquad::window::next_frame().await
    }
}