// For the client line arguments
use clap::Parser;

// For the terminal
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::{Color, Print, SetForegroundColor, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, terminal};
use std::io::{self, stdout};

// To manage the time
use std::thread::sleep;
use std::time::{Duration, SystemTime};

// For the sound
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

const MAX_SIZE: usize = 25;
const WORK_DEFAULT: usize = 25;
const REST_DEFAULT: u8 = 5;
const SESSIONS_DEFAULT: u8 = 1;
const PROGRESS_UNIT: &str = "█";
const SECONDS_IN_MINUTE: u64 = 60;
const TERMINAL_WIDTH: u16 = 42;
const END_WORKING_TIME: &[u8] = include_bytes!("school-bell.mp3");
const END_RESTING_TIME: &[u8] = include_bytes!("bike-bell.mp3");

/// Pomodoro Client Application
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// working minutes
    #[arg(short, long, default_value_t = WORK_DEFAULT as u8)]
    work: u8,

    /// resting minutes
    #[arg(short, long, default_value_t = REST_DEFAULT)]
    rest: u8,

    /// number of sessions
    #[arg(short, long, default_value_t = SESSIONS_DEFAULT)]
    sessions: u8,
}

fn calculate_position(size: u16) -> u16 {
    (TERMINAL_WIDTH / 2) - (size / 2)
}

fn show_header() -> io::Result<()> {
    // run a sequence of instructions on the standard output
    execute!(stdout(), Clear(ClearType::All),)?;
    let text = "Pomodoro Timer";
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        MoveTo(0, 8),
        Clear(ClearType::CurrentLine),
        MoveTo(calculate_position(text.len() as u16), 0),
        Print(text.underlined().bold()),
    )?;

    Ok(())
}

fn show_sessions(current_session: u8, max_sessions: u8) -> io::Result<()> {
    let sessions_text = format!("Session {} of {}", current_session, max_sessions);
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        MoveTo(calculate_position(sessions_text.len() as u16), 2),
        Print(sessions_text),
        MoveTo(0, 8),
        Clear(ClearType::CurrentLine),
    )?;
    Ok(())
}

fn show_message(text: &str) -> io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetForegroundColor(Color::Blue),
        MoveTo(calculate_position(text.len() as u16), 7),
        Clear(ClearType::CurrentLine),
        Print(text.to_string()),
        Print("\n\n"),
    )?;
    Ok(())
}

fn show_footer(text: &str) -> io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetForegroundColor(Color::Blue),
        MoveTo(calculate_position(text.len() as u16), 8),
        Clear(ClearType::CurrentLine),
        Print(text.to_string()),
        Print("\n\n"),
    )?;
    Ok(())
}

fn show_progress_bars(work: String, rest: String) -> io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        MoveTo(0, 4),
        Print(work),
        SetForegroundColor(Color::Red),
        MoveTo(0, 5),
        Print(rest),
        Print("\n\n"),
    )?;

    Ok(())
}

fn get_progress_bar_text(val: usize, max_val: usize) -> String {
    let mut corrected_max_val = max_val;
    let mut corrected_val = val;
    if max_val > MAX_SIZE {
        corrected_max_val = MAX_SIZE;
        corrected_val = max_val - (max_val - ((val * MAX_SIZE) / max_val));
    }
    // Using the repeat method, we can print a specific number of characters to the stdout
    format!(
        "[{}{}]   {:02} of {:02} minutes left. ",
        PROGRESS_UNIT.repeat(corrected_val),
        " ".repeat(corrected_max_val - corrected_val),
        max_val - val,
        max_val
    )
}

fn play_sound(sound_data: &[u8]) {
    // Create an audio output stream
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Failed to create output stream");

    // Create a sink to manage playback
    let sink = Sink::try_new(&stream_handle).expect("Failed to create sink");

    // Clone sound_data into a Vec<u8> to own the data
    let sound_data_owned = sound_data.to_vec();
    let cursor = Cursor::new(sound_data_owned);

    // Decode the audio file
    let source = Decoder::new(cursor).expect("Failed to decode audio");

    // Play the sound
    sink.append(source);
    sink.sleep_until_end(); // Wait until playback finishes
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let working_max = args.work;
    let resting_max = args.rest;
    let max_sessions = args.sessions;
    let quit_event =
        KeyEvent::new_with_kind(KeyCode::Char('q'), KeyModifiers::NONE, KeyEventKind::Press);
    let pause_event =
        KeyEvent::new_with_kind(KeyCode::Char('p'), KeyModifiers::NONE, KeyEventKind::Press);
    let mut minutes = 0;
    let mut start = SystemTime::now();
    let mut current_session = 1;
    let mut timer_active = true;
    let mut is_work_time = true;
    terminal::enable_raw_mode()?;
    execute!(stdout(), Hide, Clear(ClearType::All),)?;

    show_header()?;

    loop {
        if poll(Duration::from_millis(100))? {
            match read()? {
                Event::Key(event_key) => {
                    if event_key == quit_event {
                        execute!(
                            stdout(),
                            MoveTo(0, 7),
                            Print("Timer Interrupted!! See you soon!!\n"),
                        )?;
                        break;
                    } else if event_key == pause_event {
                        timer_active = !timer_active;
                        if timer_active {
                            execute!(stdout(), MoveTo(0, 7), Clear(ClearType::CurrentLine),)?;
                            start = SystemTime::now();
                        } else {
                            execute!(stdout(), MoveTo(0, 7), Print("Timer Paused..."),)?;
                        }
                    }
                }
                Event::FocusGained => continue,
                Event::FocusLost => continue,
                Event::Mouse(_) => continue,
                Event::Paste(_) => continue,
                Event::Resize(_, _) => continue,
            }
        }

        let text = if timer_active {
            "Press p to Pause, q to Quit!!\n"
        } else {
            "Press p to Restart, q to Quit!!\n"
        };
        let _ = show_message(text);
        if timer_active {
            let end = SystemTime::now();
            let since_start = end.duration_since(start).expect("Time went backwards");
            if since_start.as_secs() >= SECONDS_IN_MINUTE {
                start = SystemTime::now();
                minutes += 1;
            }
            show_sessions(current_session, max_sessions)?;

            if is_work_time {
                let work = get_progress_bar_text(minutes as usize, working_max as usize);
                let rest = get_progress_bar_text(0, resting_max as usize);
                show_progress_bars(work, rest)?;
                if minutes >= working_max {
                    show_footer("Time's up! Take a break! 🎉")?;
                    play_sound(END_WORKING_TIME);
                    is_work_time = !is_work_time;
                    minutes = 0;
                }
            } else {
                let work = get_progress_bar_text(working_max as usize, working_max as usize);
                let rest = get_progress_bar_text(minutes as usize, resting_max as usize);
                show_progress_bars(work, rest)?;
                if minutes >= resting_max {
                    let session_text = format!("Session {} Finished!!", current_session);
                    show_footer(&session_text)?;
                    play_sound(END_RESTING_TIME);
                    current_session += 1;
                    is_work_time = !is_work_time;
                    minutes = 0;
                    if current_session > max_sessions {
                        execute!(stdout(), MoveTo(0, 7), Clear(ClearType::CurrentLine),)?;
                        show_footer("See You Soon!!\n")?;
                        sleep(Duration::from_secs(5));
                        break;
                    }
                }
            }
        }
    }

    execute!(stdout(), Show, Clear(ClearType::All), MoveTo(0, 0),)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
