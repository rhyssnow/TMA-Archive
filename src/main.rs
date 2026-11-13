#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod buttonFile;

use std::path::Path;
use std::time::Instant;
use sdl3::EventPump;
use sdl3::gamepad::Button;
use sdl3::keyboard::{Scancode};
use sdl3::pixels::Color;
use crate::buttonFile::buttonMod::buttonTrait;
use crate::buttonFile::buttonMod::button;
use sdl3::render::{create_renderer, WindowCanvas, Texture, FRect};
use sdl3::surface::Surface;
use sdl3::video::Window;
use lopdf::Document;
use sdl3::event::Event;
use const_hashmap::{ConstMap, build_map, Bucket};

static MAG_MAP: ConstMap<i32, &str, 200> = ConstMap::new([
    Bucket::Filled { key: 1, val: "Anglerfish" },
    Bucket::Filled { key: 2, val: "Do Not Open" },
    Bucket::Filled { key: 3, val: "Across The Street" },
    Bucket::Filled { key: 4, val: "Page Turner" },
    Bucket::Filled { key: 5, val: "Thrown Away" },
    Bucket::Filled { key: 6, val: "Squirm" },
    Bucket::Filled { key: 7, val: "The Piper" },
    Bucket::Filled { key: 8, val: "Burned Out" },
    Bucket::Filled { key: 9, val: "A Father's Love" },
    Bucket::Filled { key: 10, val: "Vampire Killer" },
    Bucket::Filled { key: 11, val: "Dreamer" },
    Bucket::Filled { key: 12, val: "First Aid" },
    Bucket::Filled { key: 13, val: "Alone" },
    Bucket::Filled { key: 14, val: "Piecemeal" },
    Bucket::Filled { key: 15, val: "Lost John's Cave" },
    Bucket::Filled { key: 16, val: "Arachnophobia" },
    Bucket::Filled { key: 17, val: "The Bone Turner's Tale" },
    Bucket::Filled { key: 18, val: "The Man Upstairs" },
    Bucket::Filled { key: 19, val: "Confession" },
    Bucket::Filled { key: 20, val: "Desecrated Host" },
    Bucket::Filled { key: 21, val: "Freefall" },
    Bucket::Filled { key: 22, val: "Colony" },
    Bucket::Filled { key: 23, val: "Schwartzwald" },
    Bucket::Filled { key: 24, val: "Strange Music" },
    Bucket::Filled { key: 25, val: "Growing Dark" },
    Bucket::Filled { key: 26, val: "A Distortion" },
    Bucket::Filled { key: 27, val: "A Sturdy Lock" },
    Bucket::Filled { key: 28, val: "Skintight" },
    Bucket::Filled { key: 29, val: "Cheating Death" },
    Bucket::Filled { key: 30, val: "Killing Floor" },
    Bucket::Filled { key: 31, val: "First Hunt" },
    Bucket::Filled { key: 32, val: "Hive" },
    Bucket::Filled { key: 33, val: "Boatswain's Call" },
    Bucket::Filled { key: 34, val: "Anatomy Class" },
    Bucket::Filled { key: 35, val: "Old Passages" },
    Bucket::Filled { key: 36, val: "Taken Ill" },
    Bucket::Filled { key: 37, val: "Burnt Offering" },
    Bucket::Filled { key: 38, val: "Lost and Found" },
    Bucket::Filled { key: 39, val: "Infestation" },
    Bucket::Filled { key: 40, val: "Human Remains" },
    Bucket::Filled { key: 41, val: "Too Deep" },
    Bucket::Filled { key: 42, val: "Grifter's Bone" },
    Bucket::Filled { key: 43, val: "Section 31" },
    Bucket::Filled { key: 44, val: "Tightrope" },
    Bucket::Filled { key: 45, val: "Blood Bag" },
    Bucket::Filled { key: 46, val: "Literary Heights" },
    Bucket::Filled { key: 47, val: "The New Door" },
    Bucket::Filled { key: 48, val: "Lost in The Crowd" },
    Bucket::Filled { key: 49, val: "The Butcher's Window" },
    Bucket::Filled { key: 50, val: "Foundations" },
    Bucket::Filled { key: 51, val: "High Pressure" },
    Bucket::Filled { key: 52, val: "Exceptional Risk" },
    Bucket::Filled { key: 53, val: "Crusader" },
    Bucket::Filled { key: 54, val: "Still Life" },
    Bucket::Filled { key: 55, val: "Pest Control" },
    Bucket::Filled { key: 56, val: "Children of the Night" },
    Bucket::Filled { key: 57, val: "Personal Space" },
    Bucket::Filled { key: 58, val: "Trail Rations" },
    Bucket::Filled { key: 59, val: "Recluse" },
    Bucket::Filled { key: 60, val: "Observer Effect" },
    Bucket::Filled { key: 61, val: "Hard Shoulder" },
    Bucket::Filled { key: 62, val: "First Edition" },
    Bucket::Filled { key: 63, val: "The End of the Tunnel" },
    Bucket::Filled { key: 64, val: "Burial Rites" },
    Bucket::Filled { key: 65, val: "Binary" },
    Bucket::Filled { key: 66, val: "Held in Customs" },
    Bucket::Filled { key: 67, val: "Burning Desire" },
    Bucket::Filled { key: 68, val: "The Tale of a Field Hospital" },
    Bucket::Filled { key: 69, val: "Thought for the Day" },
    Bucket::Filled { key: 70, val: "The Book of the Dead" },
    Bucket::Filled { key: 71, val: "Underground" },
    Bucket::Filled { key: 72, val: "Takeaway" },
    Bucket::Filled { key: 73, val: "Police Lights" },
    Bucket::Filled { key: 74, val: "Fatigue" },
    Bucket::Filled { key: 75, val: "A Long Way Down" },
    Bucket::Filled { key: 76, val: "The Smell of Blood" },
    Bucket::Filled { key: 77, val: "The Kind Mother" },
    Bucket::Filled { key: 78, val: "Distant Cousin" },
    Bucket::Filled { key: 79, val: "Hide and Seek" },
    Bucket::Filled { key: 80, val: "The Librarian" },
    Bucket::Filled { key: 81, val: "A Guest for Mr Spider" },
    Bucket::Filled { key: 82, val: "The Eyewitnesses" },
    Bucket::Filled { key: 83, val: "Drawing a Blank" },
    Bucket::Filled { key: 84, val: "Possessive" },
    Bucket::Filled { key: 85, val: "Upon the Stair" },
    Bucket::Filled { key: 86, val: "Tucked in" },
    Bucket::Filled { key: 87, val: "Uncanny Valley" },
    Bucket::Filled { key: 88, val: "Dig" },
    Bucket::Filled { key: 89, val: "Twice as Bright" },
    Bucket::Filled { key: 90, val: "Body Builder" },
    Bucket::Filled { key: 91, val: "The Coming Storm" },
    Bucket::Filled { key: 92, val: "Nothing Beside Remains" },
    Bucket::Filled { key: 93, val: "Contaminant" },
    Bucket::Filled { key: 94, val: "Dead Woman Walking" },
    Bucket::Filled { key: 95, val: "Absent Without Leave" },
    Bucket::Filled { key: 96, val: "Return to Sender" },
    Bucket::Filled { key: 97, val: "We All Ignore The Pit" },
    Bucket::Filled { key: 98, val: "Lights Out" },
    Bucket::Filled { key: 99, val: "Dust to Dust" },
    Bucket::Filled { key: 100, val: "I Guess You Had to be There" },
    Bucket::Filled { key: 101, val: "Another Twist" },
    Bucket::Filled { key: 102, val: "Nesting Instinct" },
    Bucket::Filled { key: 103, val: "Cruelty Free" },
    Bucket::Filled { key: 104, val: "Sneak Preview" },
    Bucket::Filled { key: 105, val: "Total War" },
    Bucket::Filled { key: 106, val: "A Matter of Perspective" },
    Bucket::Filled { key: 107, val: "Third Degree" },
    Bucket::Filled { key: 108, val: "Monologue" },
    Bucket::Filled { key: 109, val: "Nightfall" },
    Bucket::Filled { key: 110, val: "Creature Feature" },
    Bucket::Filled { key: 111, val: "Family Business" },
    Bucket::Filled { key: 112, val: "The Thrill of the Chase" },
    Bucket::Filled { key: 113, val: "Breathing Room" },
    Bucket::Filled { key: 114, val: "Cracked Foundation" },
    Bucket::Filled { key: 115, val: "Taking Stock" },
    Bucket::Filled { key: 116, val: "The Show Must Go On" },
    Bucket::Filled { key: 117, val: "Testament" },
    Bucket::Filled { key: 118, val: "The Masquerade" },
    Bucket::Filled { key: 119, val: "Stranger and Stranger" },
    Bucket::Filled { key: 120, val: "Eye Contact" },
    Bucket::Filled { key: 121, val: "Far Awary" },
    Bucket::Filled { key: 122, val: "Zombie" },
    Bucket::Filled { key: 123, val: "Web Development" },
    Bucket::Filled { key: 124, val: "Left Hanging" },
    Bucket::Filled { key: 125, val: "Civilian Casualties" },
    Bucket::Filled { key: 126, val: "Sculptor's Tool" },
    Bucket::Filled { key: 127, val: "Remains to be Seen" },
    Bucket::Filled { key: 128, val: "Heavy Goods" },
    Bucket::Filled { key: 129, val: "Submerged" },
    Bucket::Filled { key: 130, val: "Meat" },
    Bucket::Filled { key: 131, val: "Flesh" },
    Bucket::Filled { key: 132, val: "Entombed" },
    Bucket::Filled { key: 133, val: "Dead Horse" },
    Bucket::Filled { key: 134, val: "Time of Revelation" },
    Bucket::Filled { key: 135, val: "Dark Matter" },
    Bucket::Filled { key: 136, val: "The Puppeteer" },
    Bucket::Filled { key: 137, val: "Nemesis" },
    Bucket::Filled { key: 138, val: "The Architecture of Fear" },
    Bucket::Filled { key: 139, val: "Chosen" },
    Bucket::Filled { key: 140, val: "The Movement of the Heavens" },
    Bucket::Filled { key: 141, val: "Doomed Voyage" },
    Bucket::Filled { key: 142, val: "Scrutiny" },
    Bucket::Filled { key: 143, val: "Heart of Darkness" },
    Bucket::Filled { key: 144, val: "Decrypted" },
    Bucket::Filled { key: 145, val: "Infectious Doubts" },
    Bucket::Filled { key: 146, val: "Threshold" },
    Bucket::Filled { key: 147, val: "Weaver" },
    Bucket::Filled { key: 148, val: "Extended Surveillance" },
    Bucket::Filled { key: 149, val: "Concrete Jungle" },
    Bucket::Filled { key: 150, val: "Cul-de-Sac" },
    Bucket::Filled { key: 151, val: "Big Picture" },
    Bucket::Filled { key: 152, val: "A Gravedigger's Envy" },
    Bucket::Filled { key: 153, val: "Love Bombing" },
    Bucket::Filled { key: 154, val: "Bloody Mary" },
    Bucket::Filled { key: 155, val: "Cost of Living" },
    Bucket::Filled { key: 156, val: "Reflection" },
    Bucket::Filled { key: 157, val: "Rotten Core" },
    Bucket::Filled { key: 158, val: "Panopticon" },
    Bucket::Filled { key: 159, val: "The Last" },
    Bucket::Filled { key: 160, val: "The Eye Opens" },
    Bucket::Filled { key: 161, val: "Dwelling" },
    Bucket::Filled { key: 162, val: "A Cosy Cabin" },
    Bucket::Filled { key: 163, val: "In the Trenches" },
    Bucket::Filled { key: 164, val: "The Sick Village" },
    Bucket::Filled { key: 165, val: "Revolutions" },
    Bucket::Filled { key: 166, val: "The Worms" },
    Bucket::Filled { key: 167, val: "Curiosity" },
    Bucket::Filled { key: 168, val: "Roots" },
    Bucket::Filled { key: 169, val: "Fire Escape" },
    Bucket::Filled { key: 170, val: "Recollection" },
    Bucket::Filled { key: 171, val: "The Gardener" },
    Bucket::Filled { key: 172, val: "Strung Out" },
    Bucket::Filled { key: 173, val: "Night Night" },
    Bucket::Filled { key: 174, val: "The Great Beast" },
    Bucket::Filled { key: 175, val: "Epoch" },
    Bucket::Filled { key: 176, val: "Blood Ties" },
    Bucket::Filled { key: 177, val: "Wonderland" },
    Bucket::Filled { key: 178, val: "The Processing Line" },
    Bucket::Filled { key: 179, val: "Accomplice" },
    Bucket::Filled { key: 180, val: "Moving On" },
    Bucket::Filled { key: 181, val: "Ignorance" },
    Bucket::Filled { key: 182, val: "Wellbeing" },
    Bucket::Filled { key: 183, val: "Monument" },
    Bucket::Filled { key: 184, val: "Like Ants" },
    Bucket::Filled { key: 185, val: "Locked In" },
    Bucket::Filled { key: 186, val: "Quiet" },
    Bucket::Filled { key: 187, val: "Checking Out" },
    Bucket::Filled { key: 188, val: "Centre of Attention" },
    Bucket::Filled { key: 189, val: "Peers" },
    Bucket::Filled { key: 190, val: "Scavengers" },
    Bucket::Filled { key: 191, val: "What We Lose" },
    Bucket::Filled { key: 192, val: "An Appointment" },
    Bucket::Filled { key: 193, val: "A Stern Look" },
    Bucket::Filled { key: 194, val: "Parting" },
    Bucket::Filled { key: 195, val: "Adrift" },
    Bucket::Filled { key: 196, val: "This Old House" },
    Bucket::Filled { key: 197, val: "Connected" },
    Bucket::Filled { key: 198, val: "Precipice" },
    Bucket::Filled { key: 199, val: "Seeing it Through" },
    Bucket::Filled { key: 200, val: "LastWords" }
]);

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