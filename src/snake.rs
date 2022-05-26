
pub fn snake_game(){
    extern crate piston_window;

    use piston_window::*;


    // create window
    let size = [640, 480];
    let title = "Snake";
    let mut window: PistonWindow = WindowSettings::new(title, size)
        .exit_on_esc(true)
        .build()
        .unwrap();


    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear(color::NAVY, graphics);


            // create snake square
            let rect = [0.0, 0.0, 20.0, 20.0];
            rectangle(color::WHITE, rect,
                      context.transform,
                      graphics);
        });
    }
}