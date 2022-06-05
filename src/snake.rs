use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Events, EventSettings, RenderArgs, RenderEvent, WindowSettings};
use piston_window::{clear, color, PistonWindow};

pub fn snake_game(){

    pub struct Game {
        // game components
        graphics:GlGraphics,
        snake: Snake
    }

    impl Game{
        // render game components
        pub fn render(&mut self, args: &RenderArgs){
            self.graphics.draw(args.viewport(), |_context, graphics| { clear(color::NAVY, graphics)});

            self.snake.render( &mut self.graphics, args);

        }
    }

    // snake
    pub struct Snake{
        pos_x: i32,
        pos_y: i32
    }

    impl Snake{
        // render snake
        fn render(&mut self, graphics: &mut GlGraphics, args: &RenderArgs){
            let _red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
            let _square = graphics::rectangle::square(self.pos_x as f64,
                                                     self.pos_y as f64, 20_f64);
            graphics.draw(args.viewport(),|c,_graphics|{
                let _transform = c.transform;

                // TODO fix error here
                //graphics::rectangle(_red, _square, _transform, graphics);
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
        snake: Snake { pos_x:50, pos_y: 100 }
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args(){
            game.render(&r);
        }
    }

}