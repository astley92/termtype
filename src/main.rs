use std::io::{Write, stdout};
use crossterm::cursor;
use crossterm::style::{ResetColor, SetForegroundColor};
use crossterm::terminal;
use crossterm::style::{Print, Color};
use crossterm::execute;
use crossterm::QueueableCommand;
use std::time::{Duration, Instant};

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, cursor::Hide).unwrap();

    const LIGHT_GREY: Color = Color::Rgb { r: 100, g: 100, b: 100 };
    const FPS: u32 = 30;
    let mut frame_count: u64 = 0;
    let mut achieved_fps: f64;
    let mut current_frame_start_time: Instant;
    let mut current_frame_time_delta: Duration;
    let required_frame_time_delta: Duration = Duration::from_secs(1) / FPS;
    let test_string_bytes = "Hello, World!".as_bytes();
    let start_time = Instant::now();
    let (width, height) = terminal::size().unwrap();

    loop {
        achieved_fps = frame_count as f64 / start_time.elapsed().as_secs_f64();
        frame_count += 1;
        current_frame_start_time = Instant::now();

        // Clear the screen and move the cursor to the top left corner
        stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();

        // Print the test string
        stdout
            .queue(SetForegroundColor(LIGHT_GREY)).unwrap()
            .queue(cursor::MoveTo(0, 0)).unwrap()
            .queue(Print(std::str::from_utf8(test_string_bytes).unwrap())).unwrap()
            .queue(ResetColor).unwrap();

        // Print debugging information
        stdout
            .queue(cursor::MoveTo(width - 5, height - 1)).unwrap()
            .queue(Print(format!("{:.2}", achieved_fps))).unwrap();

        stdout.flush().unwrap();

        if frame_count > 100 {
            break;
        }

        current_frame_time_delta = Instant::now().duration_since(current_frame_start_time);
        std::thread::sleep(required_frame_time_delta - current_frame_time_delta);
    }

    execute!(stdout, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    stdout.queue(cursor::MoveTo(0,0)).unwrap();
    stdout.flush().unwrap();
}
