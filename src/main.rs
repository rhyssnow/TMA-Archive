#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use sdl3::EventPump;
use sdl3::gamepad::Button;
use sdl3::keyboard::KeyboardState;
use sdl3::pixels::Color;
use sdl3::rect::Point;
use sdl3::render::{create_renderer, WindowCanvas, Texture, FRect, FPoint};
use sdl3::surface::Surface;
use sdl3::video::Window;

struct button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    active: bool,
    pressed: bool,
    hitbox: FRect,
    text: String,
    texturePos: usize,
    buttonType: String
}

impl button {
    fn new(text: &str, x: f32, y: f32, width: f32, height: f32, buttonType: &str, texturePos: usize) -> button {button{text: text.to_string(), x, y, width, height, active: false, pressed: false, hitbox: FRect::new(x, y, width, height), buttonType: buttonType.to_string(), texturePos}}

    fn input(&mut self, events: &EventPump) {
        if frectContains(events.mouse_state().x(), events.mouse_state().y(), self.hitbox) {
            self.active = true;
        }
        else {
            self.active = false;
        }
        if self.active && events.mouse_state().left() {
            self.pressed = true;
        }
        else {
            self.pressed = false;
        }
    }

    fn render(&mut self, renderer: &mut WindowCanvas, textureVec: &Vec<Texture>) {
        let radius = self.hitbox.h/2.0;
        let borderColor = match self.active {
            true => Color::RGB(200, 200, 200),
            false => Color::RGB(150, 150, 150)
        };
        let color = match self.active {
            true => Color::RGB(125, 125, 125),
            false => Color::RGB(75, 75, 75)
        };
        let mut rectRenderRect = FRect::new(
            self.hitbox.x + radius,
            self.hitbox.y,
            self.hitbox.w - self.hitbox.h,
            self.hitbox.h + 1.0
        );
        renderCircle(renderer, rectRenderRect.x, rectRenderRect.y + radius, radius, borderColor);
        renderCircle(renderer, rectRenderRect.x + rectRenderRect.w, rectRenderRect.y + radius, radius, borderColor);
        renderer.set_draw_color(borderColor);
        renderer.fill_rect(rectRenderRect).unwrap();

        renderCircle(renderer, rectRenderRect.x, rectRenderRect.y + radius, radius - 2.0, color);
        renderCircle(renderer, rectRenderRect.x + rectRenderRect.w, rectRenderRect.y + radius, radius - 2.0, color);

        rectRenderRect.x += 2.0;
        rectRenderRect.y += 2.0;
        rectRenderRect.w -= 4.0;
        rectRenderRect.h -= 4.0;

        renderer.set_draw_color(color);
        renderer.fill_rect(rectRenderRect).unwrap();
    }
}

struct meshBox {
    width: f32,
    height: f32,
    texturePos: usize,
}

impl meshBox {
    pub fn new( width: f32, height: f32) -> meshBox {meshBox{ width, height, texturePos: 0}}
    pub fn render(&mut self, textureVec: &Vec<Texture>, renderer: &mut WindowCanvas, window: &Window) {
        self.width = window.size_in_pixels().0 as f32;
        self.height = window.size_in_pixels().1 as f32;

        renderer.copy(&textureVec[self.texturePos], None, None).unwrap();
    }
}

fn main() {
    let videoSubsystem = sdl3::init().unwrap().video().unwrap();
    let mut window = videoSubsystem.window("The Magnus Archive Archive", 1920, 1080).position_centered().resizable().opengl().build().unwrap();
    let mut renderer = create_renderer(window.clone(), None).unwrap();

    logicLoop(&mut window, &mut renderer);
}

fn logicLoop (  window: &mut Window, renderer: &mut WindowCanvas) {
    let mut mode = "home";
    let mut running = true;
    let mut textureVec = Vec::new();
    let surface = Surface::load_bmp("src/Assets/magnusbackground.bmp").unwrap();
    textureVec.push(renderer.create_texture_from_surface(&surface).unwrap());
    let mut meshBox = meshBox::new(window.size_in_pixels().0 as f32, window.size_in_pixels().1 as f32);

    let mut button = button::new("y", 200.0, 300.0, 400.0, 80.0, "y", 0);

    while running {
        input(&mut button, &mut running);
        update();
        render(&mut button, renderer, &mut meshBox, &textureVec, window);
    }
}

fn input(button: &mut button, running: &mut bool) {
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
    button.input(&eventPump);
}

fn update () {

}

fn render(button: &mut button, renderer: &mut WindowCanvas, meshBox: &mut meshBox, textureVec: &Vec<Texture>, window: &Window) {
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    meshBox.render(&textureVec, renderer, window);
    button.render(renderer, textureVec);

    renderer.present();
}

fn setupEnvironment (mode: String, buttons: &mut Vec<Button>){

}

fn frectContains (pointX: f32, pointY: f32, rect: FRect) -> bool{
    let mut isWithin = true;

    if pointX < rect.x || pointX > rect.x + rect.w {
        isWithin = false;
    }
    if pointY < rect.y || pointY > rect.y + rect.h {
        isWithin = false;
    }

    isWithin
}

fn renderCircle(renderer: &mut WindowCanvas, centerX: f32, centerY: f32, radius:f32, color: Color) {
    renderer.set_draw_color(color);

    let mut offsetx = 0.0;
    let mut offsety = radius;
    let mut d = radius - 1.0;
    let status = 0.0;

    while offsety >= offsetx {
        renderer.draw_line(FPoint::new(centerX - offsety, centerY + offsetx), FPoint::new(centerX + offsety, centerY + offsetx)).unwrap();
        renderer.draw_line(FPoint::new(centerX - offsetx, centerY + offsety), FPoint::new(centerX + offsetx, centerY + offsety)).unwrap();
        renderer.draw_line(FPoint::new(centerX - offsetx, centerY - offsety), FPoint::new(centerX + offsetx, centerY - offsety)).unwrap();
        renderer.draw_line(FPoint::new(centerX - offsety, centerY - offsetx), FPoint::new(centerX + offsety, centerY - offsetx)).unwrap();


        if status < 0.0{
            break;
        }
        if d >= offsetx*2.0 {
            d -= offsetx*2.0 + 1.0;
            offsetx += 1.0;
        }
        else if d > 2.0 * (radius - offsety) {
            d += offsety*2.0 - 1.0;
            offsety -= 1.0;
        }
        else {
            d += 2.0*(offsety-offsetx-1.0);
            offsety -= 1.0;
            offsetx += 1.0;
        }
    }
}