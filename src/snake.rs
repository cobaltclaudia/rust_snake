use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Events, EventSettings, RenderArgs, RenderEvent, WindowSettings};
use piston_window::{clear, color, PistonWindow};

pub fn snake_game(){

    pub struct Game {
        // game components
        graphics:GlGraphics
        // TODO add snake
    }

    impl Game{
        // render game components
        pub fn render(&mut self, args: &RenderArgs){
            self.graphics.draw(args.viewport(), |_context, graphics| {
                clear(color::NAVY, graphics);
            // TODO render snake
        });
    }

    }

    // set it up
    let size = [640, 480];
    let title = "Snake";
    let mut window: PistonWindow = WindowSettings::new(title, size)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let opengl = OpenGL::V3_2;

    let mut game = Game{
        graphics:GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args(){
            game.render(&r)
        }
    }

}