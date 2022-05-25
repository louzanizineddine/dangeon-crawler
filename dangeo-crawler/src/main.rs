use bracket_lib::prelude::*;

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn main_play(&mut self, ctx: &mut BTerm ) {
        ctx.cls();
        ctx.print_centered(5, "Welcom to Flappy Dragon");
        ctx.print_centered(8, "(P) to Play Game");
        ctx.print_centered(8, "(Q) to Quit Game");
    }

    fn main_menu(&mut self, ctx: &mut BTerm)  {
        
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }

    }

    fn main_dead(&mut self, ctx : &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You Are Dead");
        ctx.print_centered(8, "(P) to Play Game");
        ctx.print_centered(8, "(Q) to Quit Game");

         if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        } 
    }

    fn play (&mut self , ctx:&mut BTerm ) {
        // TODO in the next chpter
    
        self.mode = GameMode::End;
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    } 
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.main_dead(ctx),
            GameMode::Playing => self.main_play(ctx),
        }
    }

    

}

enum GameMode {
    Menu, 
    Playing, 
    End,
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;

    // let gs: State = State {};
    main_loop(context, State::new())
}