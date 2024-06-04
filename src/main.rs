use std::io::{Write, stdout};
use crossterm::{cursor, event};
use crossterm::style::{ResetColor, SetForegroundColor};
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
    const FPS: u32 = 30;
    let mut fps = fps_clock::FpsClock::new(FPS);
    let test_string_bytes = "Hello, World!".as_bytes();

    let mut frame_count: u64 = 0;
    let start_time = Instant::now();
    let (width, height) = terminal::size().unwrap();

    loop {
        let achieved_fps = frame_count as f64 / start_time.elapsed().as_secs_f64();
        frame_count += 1;

        // UPDATE

        if event::poll(std::time::Duration::from_secs(0)).unwrap() {
            let event = event::read().unwrap();
            match event {
                event::Event::Key(event) => {
                    if event.code == event::KeyCode::Esc {
                        break;
                    }
                }
                _ => {}
            }
        }

        // DRAW
        // Clear the screen
        stdout
            .queue(terminal::Clear(terminal::ClearType::All)).unwrap()
            .queue(ResetColor).unwrap();

        // Print the test string
        stdout
            .queue(SetForegroundColor(LIGHT_GREY)).unwrap()
            .queue(cursor::MoveTo(0, 1)).unwrap();

        let mut str_index = 0;
        while str_index < test_string_bytes.len() as u32 {
            let ch = char::from_u32(test_string_bytes[str_index as usize] as u32).unwrap();
            stdout.queue(Print(ch)).unwrap();
            str_index += 1;
        }

        // Print debugging information
        stdout
            .queue(cursor::MoveTo(width - 5, height - 1)).unwrap()
            .queue(Print(format!("{:.2}", achieved_fps))).unwrap()
            .queue(ResetColor).unwrap()
            .queue(cursor::MoveTo(0, 0)).unwrap()
            .queue(Print(format!("{:.2}", start_time.elapsed().as_secs()))).unwrap();

        stdout.flush().unwrap();

        fps.tick();
    }

    execute!(stdout, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.queue(cursor::MoveTo(0,0)).unwrap();
    stdout.flush().unwrap();
}
