pub mod buttonMod {
    use std::path::Path;
    use sdl3::EventPump;
    use sdl3::pixels::Color;
    use sdl3::render::{FPoint, FRect, Texture, WindowCanvas};

    pub(crate) struct button {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        pub active: bool,
        pub pressed: bool,
        hitbox: FRect,
        pub textRenderArea: FRect,
        text: String,
        pub texturePos: usize,
    }

    impl button {
        pub fn new(x: f32, y: f32, width: f32, height: f32, texturePos: usize, textureVec: &mut Vec<Texture>, text: String, renderer: &WindowCanvas) -> button {
            let path = Path::new("src/Assets/DovesTypeText-Regular.ttf");
            let ttfContext = sdl3::ttf::init().map_err(|e| e.to_string()).unwrap();
            let font = ttfContext.load_font(path, 100.0).expect("SDL_TTF failed to load font.");

            let textSurface = font.render(&*text).blended(Color::RGB(255, 255, 255)).unwrap();
            let textTexture = renderer.create_texture_from_surface(&textSurface).unwrap();

            textureVec.push(textTexture);
            button { x, y, width, height,  text,active: false, pressed: false, hitbox: FRect::new(x, y, width, height), textRenderArea: FRect::new(0.0,0.0,0.0,0.0), texturePos }
        }

        pub(crate) fn input(&mut self, events: &EventPump) {
            if frectContains(events.mouse_state().x(), events.mouse_state().y(), self.hitbox) {
                self.active = true;
            } else {
                self.active = false;
            }
            if self.active && events.mouse_state().left() {
                self.pressed = true;
            } else {
                self.pressed = false;
            }
        }

        pub(crate) fn render(&mut self, renderer: &mut WindowCanvas, textureVec: &Vec<Texture>) {
            let radius = self.hitbox.h / 2.0;
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

            rectRenderRect.y += 3.0;
            rectRenderRect.h -= 6.0;
            rectRenderRect.x = rectRenderRect.x + rectRenderRect.w /2.0;
            rectRenderRect.w = rectRenderRect.h * 0.4 * self.text.len() as f32;
            rectRenderRect.x -= rectRenderRect.w /2.0;
            self.textRenderArea = rectRenderRect;
            renderer.copy(&textureVec[self.texturePos], None, rectRenderRect).unwrap();
        }
    }

    pub(crate) trait buttonTrait {
        fn asButton(&mut self) -> &mut button;
        fn input(&mut self, events: &EventPump) {
            self.asButton().input(events);
        }

        fn render(&mut self, renderer: &mut WindowCanvas, textureVec: &Vec<Texture>) {
            self.asButton().render(renderer, textureVec);
        }
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
}