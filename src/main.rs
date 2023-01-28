use std::process::exit;
use rich_sdl2_rust::{delay, EventBox, Sdl, Video};
use rich_sdl2_rust::color::Rgb;
use rich_sdl2_rust::geo::Rect;
use rich_sdl2_rust::renderer::pen::Pen;
use rich_sdl2_rust::renderer::Renderer;
use rich_sdl2_rust::window::WindowBuilder;

fn main() {
    let sdl = Sdl::new();
    let video = Video::new(&sdl);
    let window = WindowBuilder::builder()
        .width(300)
        .height(300)
        .resizable(true)
        .title("Hello, World!".to_string())
        .build()
        .new_window(&video);
    let renderer = Renderer::new(&window);

    macro_rules! use_pen {
        ($closure:expr) => {
            const fn assert<T: FnOnce(&Pen)>(_: &T) {}

            assert(&$closure);
            let pen = Pen::new(&renderer);
            $closure(&pen);
            drop(pen);
        };
    }

    let mut event_box = EventBox::new(&video);
    event_box.handle_quit(Box::new(|x| {
        println!("exit");
        exit(0)
    }));

    use_pen!(|pen: &Pen| {
        pen.set_color(Rgb::from(0xFFFFFF));
        pen.fill_rect(Rect::from_xs_ys([0, 100], [0, 100]));
    });

    window.show();
    loop {
        event_box.poll();
        use_pen!(|pen: &Pen| {
            pen.set_color(Rgb::from(0xFF0000));
            pen.fill_rect(Rect::from_xs_ys([0, 100], [0, 100]));
        });
        delay(16);
    }
}
