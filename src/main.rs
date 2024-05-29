use ggez::*;
use rand::Rng;

struct State {
    //4x4 array of u16
    board: [[u16; 4]; 4],
    score: u32,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        
        
        Ok(())
    }
}

fn gen_tile() -> u16 {
    //generate a 2 or 4 with 90% and 10% probability respectively
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..10);
    if x == 0 {
        return 4;
    } else {
        return 2;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    
    let mut state = State {
        board: [[0; 4]; 4],
        score: 0,
    };

    //generate initial two tiles            
    let x0 = rng.gen_range(0..4);
    let y0 = rng.gen_range(0..4);

    let mut x1;
    let mut y1;
    loop {
        x1 = rng.gen_range(0..4);
        y1 = rng.gen_range(0..4);

        if x0 != x1 && y0 != y1 {
            break;
        }   
    }

    state.board[x0][y0] = gen_tile();
    state.board[x1][y1] = gen_tile();

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("2048", "Henry Baldwin")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
