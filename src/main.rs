// wrench2.png is taken from https://commons.wikimedia.org/wiki/File:Wrench-screwdriver-icon.png
// It is CC0

// This code is licensed under MIT License

use std::borrow::{Borrow};
use std::cell::RefCell;
use std::ops::{DerefMut};
use std::process::exit;
use rich_sdl2_rust::image::{Img, ImgInitFlag};
use rich_sdl2_rust::image::surface::ImgSurface;
use rich_sdl2_rust::{delay, EventBox, Sdl, Video};
use rich_sdl2_rust::geo::{Rect};
use rich_sdl2_rust::renderer::pen::Pen;
use rich_sdl2_rust::renderer::{PasteExt, Renderer};
use rich_sdl2_rust::texture::Texture;
use rich_sdl2_rust::window::{WindowBuilder};

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
    let mut event_box = EventBox::new(&video);
    event_box.handle_quit(Box::new(|_| {
        println!("exit");
        exit(0)
    }));

    window.show();
    {
        let x = RefCell::new(SettingButton::new(&renderer));
        let b = x.borrow_mut().deref_mut() as *mut SettingButton;
        loop {
            event_box.poll();
            unsafe { b.as_mut() }.unwrap().draw();
            delay(16);
        }
    }
}

struct SettingButton<'renderer, 'img> {
    _selected: bool,
    img: Img,
    cached_texture: Option<Texture<'renderer>>,
    cached_surface: Option<ImgSurface<'img>>,
    renderer: &'renderer Renderer<'renderer>,
}

impl<'r: 'i, 'i> SettingButton<'r, 'i> {
    fn new(renderer: &'r Renderer) -> Self {
        let img = Img::new(ImgInitFlag::PNG).expect("Img");

        Self {
            _selected: false,
            img,
            cached_texture: None,
            cached_surface: None,
            renderer,
        }
    }

    fn draw(&'r mut self) {
        // TODO: why image does not appear?
        let un_cached = self.cached_surface.is_none();

        if un_cached {
            self.cached_surface.replace(ImgSurface::new(&self.img, "assets/wrench2.png", None).expect("ImgSurface"));
        }

        let un_cached = self.cached_texture.as_ref().borrow().is_none();
        if un_cached {
            self.cached_texture.replace(Texture::from_surface(self.renderer, self.cached_surface.as_ref().unwrap()));
        }

        let temp_ref = self.cached_texture.as_ref().unwrap();
        self.renderer.paste(temp_ref, Some(Rect::from_xs_ys([0, 300], [0, 300])));

        let pen = Pen::new(self.renderer);
        drop(pen);
    }
}
