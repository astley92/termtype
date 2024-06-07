use std::io::{Write, stdout};
use crossterm::{cursor, event, style};
use crossterm::style::{style, ResetColor, SetAttribute, Stylize};
use crossterm::terminal;
use crossterm::style::{Print, Color};
use crossterm::execute;
use crossterm::QueueableCommand;
use std::time::Instant;
use std::char;

extern crate fps_clock;

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, cursor::Hide).unwrap();

    const LIGHT_GREY: Color = Color::Rgb { r: 100, g: 100, b: 100 };
    const WHITE: Color = Color::Rgb { r: 255, g: 255, b: 255 };
    const BLACK: Color = Color::Rgb { r: 0, g: 0, b: 0 };
    const GREEN: Color = Color::Rgb { r: 0, g: 255, b: 0 };
    const RED: Color = Color::Rgb { r: 255, g: 0, b: 0 };

    const FPS: u32 = 30;
    let mut fps = fps_clock::FpsClock::new(FPS);
    const CORRECT_STRING_BYTES: &[u8] = "Hello, World!".as_bytes();
    let mut user_buff: [u8; CORRECT_STRING_BYTES.len()] = [0; CORRECT_STRING_BYTES.len()];
    let mut user_position: u32 = 0;
    let mut running: bool = true;

    let mut frame_count: u64 = 0;
    let start_time = Instant::now();
    let (width, height) = terminal::size().unwrap();

    while running {
        let achieved_fps = frame_count as f64 / start_time.elapsed().as_secs_f64();
        frame_count += 1;

        // UPDATE
        if event::poll(std::time::Duration::from_secs(0)).unwrap() {
            let event = event::read().unwrap();
            match event {
                event::Event::Key(event) => {
                    match event.code {
                        event::KeyCode::Esc => {
                            running = false
                        }
                        event::KeyCode::Backspace => {
                            if user_position == 0 {
                                continue;
                            }

                            user_position -= 1;
                            user_buff[user_position as usize] = 0;
                        }
                        event::KeyCode::Char(c) => {
                            if user_position < user_buff.len() as u32 {
                                user_buff[user_position as usize] = c as u8;
                                user_position += 1;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // DRAW
        // Clear the screen
        stdout
            .queue(terminal::Clear(terminal::ClearType::All)).unwrap()
            .queue(cursor::MoveTo(0, 0)).unwrap()
            .queue(ResetColor).unwrap();

        // Print the test string
        stdout.queue(cursor::MoveTo(0, 1)).unwrap();
        let mut str_index = 0;
        while str_index < CORRECT_STRING_BYTES.len() as u32 {
            let fg: Color;
            let mut bg = None;
            if str_index > user_position {
                fg = LIGHT_GREY;
            } else if str_index == user_position {
                fg = BLACK;
                bg = Some(WHITE);
            } else {
                if CORRECT_STRING_BYTES[str_index as usize] == user_buff[str_index as usize] {
                    fg = GREEN;
                } else {
                    fg = RED;
                }

                if CORRECT_STRING_BYTES[str_index as usize] == 32 {
                    stdout.queue(SetAttribute(style::Attribute::Underlined)).unwrap();
                } else {
                    stdout.queue(SetAttribute(style::Attribute::Reset)).unwrap();
                }
            }
            let ch = char::from_u32(CORRECT_STRING_BYTES[str_index as usize] as u32).unwrap();
            let mut styled = style(ch).with(fg);
            if bg.is_some() {
                styled = styled.on(bg.unwrap());
            }
            stdout.queue(Print(styled)).unwrap();
            str_index += 1;
        }

        // Print debugging information
        stdout
            .queue(cursor::MoveTo(width - 5, height - 1)).unwrap()
            .queue(Print(format!("{:.2}", achieved_fps))).unwrap()
            .queue(ResetColor).unwrap()
            .queue(cursor::MoveTo(0, 0)).unwrap()
            .queue(Print(format!("{:.2}", start_time.elapsed().as_secs()))).unwrap()
            .queue(cursor::MoveTo(0, 10)).unwrap()
            .queue(Print(format!("{}:{}", CORRECT_STRING_BYTES.len(), user_buff.len()))).unwrap();

        stdout.flush().unwrap();

        fps.tick();
    }

    execute!(stdout, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.queue(cursor::MoveTo(0,0)).unwrap();
    stdout.flush().unwrap();
}
