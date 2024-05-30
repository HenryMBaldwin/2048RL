use macroquad::{color::*, text::draw_text, time::get_fps, window::clear_background};



const PADDING_TOP: f32 = 100.0;

//animations
enum Animation {
    Slide(Slide),
    Merge(Merge),
    Spawn(Spawn),
}

enum Direction {
    Up,
    Down,
    Left,
    Right

}

struct Slide {
    direction: Direction,
}

struct Merge {
    direction: Direction,
}

struct Spawn {
    value: u32,
}

//don't draw or acknowledge the 0 tiles
#[derive(Default)]
struct Tile {
    x: usize,
    y: usize,
    value: usize,
    animation_queue: Box<Vec<Box<dyn Fn(&Tile)>>>,
}

struct Board {
    tiles: [[Tile; 4]; 4],
    score: u32,
}


impl Tile{
    fn new(x: usize, y: usize) -> Tile{
        Tile{value: 0, animation_queue: Box::new(Vec::new()), x, y}
    }

    fn init(&mut self, x: usize, y: usize){
        self.x = x;
        self.y = y;
    }

    fn slide(&mut self, direction: Direction){
    }

    fn merge(&mut self, direction: Direction){

    }

    fn spawn(&mut self, value: u32){
        
        //spawn animation

    }

    fn draw(&self){
        if self.value != 0{
            //draw tile
        }
    }
}

impl Board {
    fn new() -> Board{
        let mut tiles: [[Tile; 4]; 4] = Default::default();
        for x in 0..4{
            for y in 0..4{
                tiles[x][y] = Tile::new(x, y);
            }
        }
        let board = Board{tiles, score: 0};
        board
    }

    fn set_score(&mut self, score: u32){
        self.score = score;
    }

    fn slide(&mut self, direction: Direction, x: usize, y: usize){
        self.tiles[x][y].slide(direction);
    }

    fn merge(&self, direction: Direction, x: usize, y: usize){

    }

    fn spawn(&self, x: usize, y: usize, value: u32){

    }

    fn draw(&self){
        clear_background(WHITE);
        //draw fps
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., GREEN);


        for x in 0..4{
            for y in 0..4{
                self.tiles[x][y].draw();
            }
        }

    }

}