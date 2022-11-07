use rand::Rng;
use rand::distributions::{Distribution, Uniform};
pub struct Settings{
    pub width: usize,
    pub height: usize,
    pub initial_alive: usize,
    pub life_cycles: usize,
}

impl Settings {
    pub fn new(width: usize, height: usize, intial_alive: usize, num_cycles: usize) -> Self {
        assert!(intial_alive > 0 && width > 1 && height > 1);
        assert!(num_cycles > 0);
        assert!(intial_alive < width * height);
        Settings {
            width: width,
            height: height,
            initial_alive: intial_alive,
            life_cycles: num_cycles,
        }
    }
}

pub struct GameState {
    pub game_settings: Settings,

    pub board: Vec<Vec<u32>>,

    pub num_alive: usize,
    pub num_dead: usize,
    pub total: usize,

    pub curr_cycle: usize,
}

impl GameState{

    pub fn new(settings: Settings) -> Self {
        GameState{
            num_alive: settings.initial_alive,
            total: settings.initial_alive,
            board: vec![vec![0; settings.width]; settings.height],
            game_settings: settings,
            num_dead: 0,
            curr_cycle: 0,
        }
    }

    pub fn initialize_game(&mut self){
        
        
        let step: Uniform<u32> = Uniform::new(1, 100);
        let mut rng = rand::thread_rng();
        let mut live_cells_placed: usize = 0;
        
        for i in 0..self.game_settings.height{
            for j in 0..self.game_settings.width{
                
                let outcome = step.sample(&mut rng);
                if outcome >= 50 {
                    self.board[i][j] = 1;
                    live_cells_placed += 1;
                }

                if live_cells_placed >= self.num_alive{
                    return;
                }
            }
        }
    }

    pub fn print_board(&mut self){
        for i in 0..self.game_settings.height{
            for j in 0..self.game_settings.width {
                print!("{} ", self.board[i][j]);
            }
            print!("\n");
        }
    }

    pub fn iterate(&mut self){
        
        self.curr_cycle += 1;
    }

    pub fn print_cycle(&mut self){
        println!("Life Cycle: {}", self.curr_cycle);
        println!("Total Cells: {}", self.total);
        println!("Alive Cells: {}", self.num_alive);
        println!("Dead Cells: {}", self.num_dead);
        self.print_board();
        print!("\n");
    }

}

fn main() {
    let game_settings: Settings = Settings::new(8, 8, 10, 10);

    let mut game_state = GameState::new(game_settings);
    game_state.initialize_game();
    
    for cycle in 0..game_state.game_settings.life_cycles{
        game_state.iterate();
        game_state.print_cycle();
    }

}
