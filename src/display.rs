use crate::Chip8;
use termkan;
use termkan::{rds::Renderer, math::*, img::Color, input::{InputEvent, Input, KeyEvent}};


impl Chip8 {
    pub fn buf_clear_screen(&mut self) {
        for x in 0..64 {
            for y in 0..32 {
                self.display_buf[x][y] = false;
            }
        }
    }

    pub fn buf_draw_sprite(&mut self, X: u8, Y: u8, N: u8) {
        // should set VF to 1 if something's XOR'd during drawing
        // println!("x={}, y={}, N={}", X, Y, N);
        self.V[0xF] = 0;
        let x = X as usize % 64;
        let y = Y as usize % 32;
        let mut mask;

        'outer: for k in 0..(N as usize) {
            mask = 1 << 7;
            for l in 0..8 {
                if self.memory[self.i as usize + k as usize] & mask == mask { // si le l-ième bit du row actuel est 1:
                    // println!("On y est!");
                    if self.display_buf[x + l][y + k] == true {
                        // println!("cancel");
                        self.V[0xF] = 1;
                        self.display_buf[x + l][y + k] = false;
                    }
                    else {
                        // println!("devient true");
                        self.display_buf[x + l][y + k] = true;
                    }
                }
                if x + l >= 64 - 1 { continue 'outer; }
                mask >>= 1;
            }
            if y + k >= 32 - 1 { break 'outer; }
        }
    }
}


pub struct Display {
    top_left: Vec2,
}


impl Display {
    pub fn new() -> Display {
        let rds = Renderer::get();
        let dim = Renderer::get_size();

        let top_left = Vec2 {
            x: (dim.x / 2) - 33,
            y: (dim.y / 2) - 17,
        };

        let size = Vec2 {
            x: 66,
            y: 34,
        };

        rds.begin_draw();
        rds.draw_rect(top_left, size, Color::WHITE);
        rds.end_draw();

        Display {
            top_left: Vec2 {
                x: top_left.x + 1,
                y: top_left.y + 1
            }
        }
    }

    // Detects if Ctrl+C is pressed
    pub fn should_quit(&self) -> bool {
        let input = Input::get();
        
        Some(InputEvent::Key(KeyEvent::Ctrl('c'))) == input.get_event()
    }

    pub fn is_key_down(&self, key: KeyEvent) -> bool {
        let input = Input::get();
        Some(InputEvent::Key(key)) == input.get_event()
    }

    pub fn is_key_up(&self, key: KeyEvent) -> bool {
        let input = Input::get();
        Some(InputEvent::Key(key)) != input.get_event()
    }

    pub fn any_key_pressed(&self) -> Option<u8> {
        let input = Input::get();
        if let Some(InputEvent::Key(key)) = input.get_event() {
            return Chip8::code_from_kkey(key);
        }
        return None;
    }

    pub fn draw_point(&self, vec2: Vec2, color: Color) {
        let rds = Renderer::get();
        let coords = Vec2 {
            x: self.top_left.x + vec2.x,
            y: self.top_left.y + vec2.y,
        };

        rds.draw_point(coords, color);
    }

    pub fn update(&mut self, buf: &[[bool; 32]; 64]) {
        let rds = Renderer::get();
        rds.begin_draw();
        for x in 0..64 {
            for y in 0..32 {
                if buf[x][y] {
                    self.draw_point(Vec2::new(x as i32, y as i32), Color::WHITE);
                }
                else {
                    self.draw_point(Vec2::new(x as i32, y as i32), Color::BLACK);
                }
            }
        }
        rds.end_draw();
    }
}