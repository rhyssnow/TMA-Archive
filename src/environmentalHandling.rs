use sdl3::render::{Texture, WindowCanvas};
use sdl3::surface::Surface;
use std::fs;
use std::io::Write;
use lopdf::Document;
use crate::{magButton, meshBox, sceneChangeButton, searchButton, MAG_MAP};
use crate::buttonFile::buttonMod::buttonTrait;

pub fn setupFiles() {
    for i in 1..201 {
        let mut magNum = i.to_string();
        if i < 10 {
            magNum = "00".to_string() + i.to_string().as_str();
        } else if i < 100 {
            magNum = "0".to_string() + i.to_string().as_str();
        }
        let magName = "MAG".to_string() + magNum.as_str() + " - " + MAG_MAP[i];
        if !fs::exists("src/Assets/MagnusArchivesTranscripts/".to_string() + magName.as_str() + ".txt").unwrap(){
            let mut file = fs::File::create("src/Assets/MagnusArchivesTranscripts/".to_string() + magName.as_str() + ".txt").unwrap();
            let mut text: String = "".to_string();
            for (page, _) in Document::load("src/Assets/MagnusArchivesTranscripts/".to_string() +magName.as_str() + ".pdf").unwrap().get_pages().iter().enumerate() {
                text+= &*Document::load("src/Assets/MagnusArchivesTranscripts/".to_string() + magName.as_str() + ".pdf").unwrap().extract_text(&[(page + 1) as u32]).unwrap();
            }
            file.write_all(text.as_bytes()).unwrap();
        }
    }
}

pub fn setupEnvironment (mode: &String, magButtons: &mut Vec<magButton>, sceneButtons: &mut Vec<sceneChangeButton>, searchButtons: &mut Vec<searchButton>, meshBox: & meshBox, textureVec: &mut Vec<Texture>, renderer: &mut WindowCanvas, maxHeight: &mut f32) {
    magButtons.clear();
    sceneButtons.clear();
    searchButtons.clear();
    textureVec.clear();

    match mode.as_str() {
        "home" => {
            let surface = Surface::load_bmp("src/Assets/magnusbackground.bmp").unwrap();
            textureVec.push(renderer.create_texture_from_surface(&surface).unwrap());

            let sceneButton1 = sceneChangeButton::new(
                "archive".to_string(),
                20.0,
                meshBox.height - 120.0,
                meshBox.width/2.0 - 30.0,
                100.0,
                1,
                "Archive".to_string(),
                renderer,
                textureVec
            );
            let sceneButton2 = sceneChangeButton::new(
                "search".to_string(),
                meshBox.width/2.0 + 30.0,
                meshBox.height - 120.0,
                meshBox.width/2.0 - 30.0,
                100.0,
                2,
                "Search".to_string(),
                renderer,
                textureVec
            );
            sceneButtons.push(sceneButton1);
            sceneButtons.push(sceneButton2);
        }
        "archive" => {
            let surface = Surface::load_png("src/Assets/blank.png").unwrap();
            textureVec.push(renderer.create_texture_from_surface(&surface).unwrap());
            let homeButton = sceneChangeButton::new(
                "home".to_string(),
                20.0,
                20.0,
                240.0,
                80.0,
                1,
                "Return".to_string(),
                renderer,
                textureVec
            );
            sceneButtons.push(homeButton);

            for texturePos in 1..201 {
                let mut magNum = texturePos.to_string();
                if texturePos < 10 {
                    magNum = "00".to_string() + texturePos.to_string().as_str();
                } else if texturePos < 100 {
                    magNum = "0".to_string() + texturePos.to_string().as_str();
                }
                let magName = "MAG".to_string() + magNum.as_str() + " - " + MAG_MAP[texturePos];
                magButtons.push(magButton::new(
                    magName.clone() + ".pdf",
                    40.0,
                    140.0 * texturePos as f32,
                    meshBox.width - 80.0,
                    80.0,
                    texturePos as usize + 1,
                    magName,
                    renderer,
                    textureVec
                ))
            }
            *maxHeight = orderButtons(magButtons);
        }
        "search" => {
            let surface = Surface::load_png("src/Assets/blank.png").unwrap();
            textureVec.push(renderer.create_texture_from_surface(&surface).unwrap());
            let homeButton = sceneChangeButton::new(
                "home".to_string(),
                20.0,
                20.0,
                240.0,
                80.0,
                1,
                "Return".to_string(),
                renderer,
                textureVec
            );
            sceneButtons.push(homeButton);

            let searchButton = searchButton::new(
                300.0,
                20.0,
                meshBox.width - 600.0,
                80.0,
                2,
                renderer,
                textureVec
            );
            searchButtons.push(searchButton);

            for texturePos in 1..201 {
                let mut magNum = texturePos.to_string();
                if texturePos < 10 {
                    magNum = "00".to_string() + texturePos.to_string().as_str();
                } else if texturePos < 100 {
                    magNum = "0".to_string() + texturePos.to_string().as_str();
                }
                let magName = "MAG".to_string() + magNum.as_str() + " - " + MAG_MAP[texturePos];
                magButtons.push(magButton::new(
                    magName.clone() + ".pdf",
                    40.0,
                    140.0 * texturePos as f32,
                    meshBox.width - 80.0,
                    80.0,
                    texturePos as usize + 2,
                    magName,
                    renderer,
                    textureVec
                ))
            };
            *maxHeight = orderButtons(magButtons);
        }
        _ => {}
    }
}

pub fn orderButtons (magButtons: &mut Vec<magButton>) -> f32 {
    let mut maxHeight = 140.0;

    for magButton in magButtons.iter_mut() {
        if magButton.active {
            magButton.asButton().hitbox.y = maxHeight;
            maxHeight += 140.0;
        }
    }

    maxHeight -= 140.0*8.0;

    -maxHeight
}