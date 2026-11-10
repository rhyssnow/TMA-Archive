#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use sdl3::gamepad::Button;
use sdl3::render::{create_renderer, WindowCanvas, Texture, FRect};
use sdl3::video::Window;

struct button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
}

struct meshBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    endOfDirection: Vec<String>,
    texturePos: usize,
}

impl meshBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> meshBox {meshBox{x, y, width, height, endOfDirection: Vec::new(), texturePos: 4}}
    pub fn render(&mut self, textureVec: &Vec<Texture>, renderer: &mut WindowCanvas, renderArea: FRect) {
        let textureArea = FRect::new(self.x, self.y, renderArea.w / 6.0, renderArea.h / 6.0 );

        self.width = renderArea.w /6.0;
        self.height = renderArea.h /6.0;

        renderer.copy(&textureVec[self.texturePos], textureArea, renderArea).unwrap();
    }
    pub fn updatePosition(&mut self, mx: f32, my: f32, texture: &Texture) {
        self.x += mx;
        self.y += my;
        self.endOfDirection.clear();

        if self.x <= 0.0 {
            self.x = 0.0;
            self.endOfDirection.push("left".to_string());
        }

        if self.x + self.width >= texture.width() as f32 {
            self.x = texture.width() as f32 - self.width;
            self.endOfDirection.push("right".to_string());
        }

        if self.y <= 0.0 {
            self.y = 0.0;
            self.endOfDirection.push("up".to_string());
        }

        if self.y + self.height >= texture.height() as f32 {
            self.y = texture.height() as f32 - self.height;
            self.endOfDirection.push("down".to_string());
        }
    }
}

fn main() {
    let mut running = true;
    let videoSubsystem = sdl3::init().unwrap().video().unwrap();
    let mut window = videoSubsystem.window("Game", 1920, 1080).position_centered().resizable().opengl().build().unwrap();
    let mut renderer = create_renderer(window.clone(), None).unwrap();


}

fn logicLoop (running: &mut bool,  window: &mut Window, renderer: &mut WindowCanvas) {
    let mut mode = "home";
    let meshBox = meshBox::new(0.0, 0.0, window.size_in_pixels().0 as f32, window.size_in_pixels().1 as f32);

    while *running {

    }
}

fn update () {

}

fn input(buttons: &mut Vec<Button>, running: &mut bool) {
    let mut eventPump = sdl3::init().unwrap().event_pump().unwrap();

    for event in eventPump.poll_iter() {
        use sdl3::event::Event;
        match event {
            Event::Quit { .. } =>  {
                *running = false;
            }
            _ => {}
        }
    }
}

fn render(renderer: &mut WindowCanvas, meshBox: &mut meshBox) {

}