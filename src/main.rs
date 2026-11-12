#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod buttonFile;

use std::path::Path;
use std::time::Instant;
use sdl3::EventPump;
use sdl3::gamepad::Button;
use sdl3::keyboard::{KeyboardState, Scancode};
use sdl3::pixels::Color;
use sdl3::rect::Point;
use crate::buttonFile::buttonMod::buttonTrait;
use crate::buttonFile::buttonMod::button;
use sdl3::render::{create_renderer, WindowCanvas, Texture, FRect, FPoint};
use sdl3::surface::Surface;
use sdl3::video::Window;
use lopdf::Document;
use sdl3::event::Event;

struct sceneChangeButton {
    changeTo: String,
    button: button
}

impl buttonTrait for sceneChangeButton{
    fn asButton(&mut self) -> &mut button{
        &mut self.button
    }
}

impl sceneChangeButton{
    fn new(changeTo: String, x: f32, y: f32, width: f32, height: f32, texturePos: usize, text: String, renderer: &WindowCanvas, textureVec: &mut Vec<Texture>) -> sceneChangeButton{ sceneChangeButton{changeTo, button: button::new( x, y, width, height, texturePos, textureVec, text, renderer)} }
    fn update(&mut self, mode: &mut String) {
        if self.asButton().pressed{
            *mode = self.changeTo.clone();
        }
    }
}

struct searchButton {
    search: String,
    clickedIn: bool,
    results: Vec<String>,
    cursorBlink: Instant,
    renderCursor: bool,
    button: button
}

impl buttonTrait for searchButton{
    fn asButton(&mut self) -> &mut button{
        &mut self.button
    }
}

impl searchButton{
    fn new(x: f32, y: f32, width: f32, height: f32, texturePos: usize, renderer: &WindowCanvas, textureVec: &mut Vec<Texture>) -> searchButton{searchButton{search: "".to_string(), clickedIn: false, results: Vec::new(), cursorBlink: Instant::now(), renderCursor: true, button: button::new( x, y, width, height, texturePos, textureVec, "search".to_string(), renderer)}}

    fn searchRender (&mut self, renderer: &mut WindowCanvas) {
        if self.cursorBlink.elapsed().as_millis() > 100 {
            self.renderCursor = !self.renderCursor;
            self.cursorBlink = Instant::now();
        }
        if self.clickedIn && self.renderCursor {
            let cursorBox = FRect::new(
                self.asButton().textRenderArea.x + self.asButton().textRenderArea.w + 3.0,
                self.asButton().textRenderArea.y,
                5.0,
                self.asButton().textRenderArea.h
            );
            renderer.set_draw_color(Color::RGB(255, 255, 255));
            renderer.fill_rect(cursorBox).unwrap();
        }
    }
    fn update(&mut self, renderer: &WindowCanvas, textureVec: &mut Vec<Texture>){
        let path = Path::new("src/Assets/DovesTypeText-Regular.ttf");
        let ttfContext = sdl3::ttf::init().map_err(|e| e.to_string()).unwrap();
        let font = ttfContext.load_font(path, 100.0).expect("SDL_TTF failed to load font.");

        if self.asButton().pressed {
            self.clickedIn = true;
        }

        let textSurface: Surface;

        if self.search.is_empty() {
            textSurface = font.render("search").blended(Color::RGB(200, 200, 200)).unwrap();
        }
        else {
            textSurface = font.render(&*self.search).blended(Color::RGB(255, 255, 255)).unwrap();
        }
        let textTexture = renderer.create_texture_from_surface(&textSurface).unwrap();

        textureVec[self.asButton().texturePos] = textTexture;
    }

    fn searchInput (&mut self, events: &mut EventPump, magButtons: &mut Vec<magButton>){
        if self.clickedIn {
            if !self.asButton().active && events.mouse_state().left(){
                self.clickedIn = false;
            }
            if events.keyboard_state().is_scancode_pressed(Scancode::Return) {
                self.search(magButtons);
            }
            if events.keyboard_state().is_scancode_pressed(Scancode::Backspace) {
                self.search.pop();
            }
            for event in events.poll_iter() {
                match event {
                    Event::TextInput {text, ..} => {
                        self.search += &text;
                    }
                    _ => {}
                }
            }
        }
    }

    fn search (&mut self, magButtons: &mut Vec<magButton>, ) {
        for magButton in magButtons {
            let file = "src/Assets/MagnusArchivesTranscripts".to_string() + &*magButton.file.clone() + ".pdf";
            match Document::load(file) {
                Ok(document) => {
                    let pages = document.get_pages();
                    let mut texts = Vec::new();

                    for (i, _) in pages.iter().enumerate() {
                        let page_number = (i + 1) as u32;
                        let text = document.extract_text(&[page_number]);
                        texts.push(text.unwrap_or_default());
                    }

                    for text in texts {
                        if text.contains(self.search.as_str()) {
                            magButton.active = true;
                        }
                        else {
                            magButton.active = false;
                        }
                    }
                }
                Err(err) => eprintln!("Error: {}", err),
            }
        }
    }
}

struct magButton {
    file: String,
    button: button,
    active: bool
}

impl buttonTrait for magButton{
    fn asButton(&mut self) -> &mut button{
        &mut self.button
    }
}

impl magButton {
    fn new(file: String, x: f32, y: f32, width: f32, height: f32, texturePos: usize, text: String, renderer: &WindowCanvas, textureVec: &mut Vec<Texture>) -> magButton{magButton{file, active: true, button: button::new( x, y, width, height, texturePos, textureVec, text, renderer)}}

    fn update (&mut self, mode: &mut String) {
        if self.asButton().pressed {
            *mode = "viewer/".to_string() + &*self.file.clone();
        }
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


    while running {
        input(&mut running);
        update();
        render( renderer, &mut meshBox, &textureVec, window);
    }
}

fn input( running: &mut bool) {
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

fn update () {

}

fn render( renderer: &mut WindowCanvas, meshBox: &mut meshBox, textureVec: &Vec<Texture>, window: &Window) {
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    meshBox.render(&textureVec, renderer, window);

    renderer.present();
}

fn setupEnvironment (mode: String, buttons: &mut Vec<Button>) {

}