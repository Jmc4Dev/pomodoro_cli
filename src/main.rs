use clap::Parser;
use crossterm::style::{Color, SetForegroundColor, Stylize};
use crossterm::{cursor, execute, style::Print, terminal};
use std::io::{self, stdout};
use std::thread::sleep;
use std::time::Duration;

const MAX_SIZE: usize = 25;
const WORK_DEFAULT: usize = 25;
const REST_DEFAULT: u8 = 5;
const SESSIONS_DEFAULT: u8 = 1;
const PROGRESS_UNIT: &str = "█";
const SECONDS_DELAY: u64 = 1;

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

fn show_header() -> io::Result<()> {
    // run a sequence of instructions on the standard output
    execute!(stdout(), terminal::Clear(terminal::ClearType::All),)?;

    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        cursor::MoveTo(18, 7),
        terminal::Clear(terminal::ClearType::CurrentLine),
        cursor::MoveTo(18, 0),
        Print("Pomodoro Timer".underlined().bold()),
    )?;

    Ok(())
}

fn show_sessions(current_session: u8, max_sessions: u8) -> io::Result<()> {
    let sessions_text = format!("Session {} of {}", current_session, max_sessions);
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        cursor::MoveTo(18, 2),
        Print(sessions_text),
        cursor::MoveTo(18, 7),
        terminal::Clear(terminal::ClearType::CurrentLine),
    )?;
    Ok(())
}

fn show_footer(text: &str) -> io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetForegroundColor(Color::Blue),
        cursor::MoveTo(18, 7),
        terminal::Clear(terminal::ClearType::CurrentLine),
        Print(text.to_string()),
        Print("\n\n"),
    )?;
    Ok(())
}

fn show_progress_bars(work: String, rest: String) -> io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        cursor::MoveTo(0, 4),
        Print(work),
        SetForegroundColor(Color::Red),
        cursor::MoveTo(0, 5),
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

fn main() -> io::Result<()> {
    let args = Args::parse();
    let working_max = args.work;
    let resting_max = args.rest;
    let max_sessions = args.sessions;

    show_header()?;

    for s in 1..=max_sessions {
        show_sessions(s, max_sessions)?;

        for i in 0..=working_max {
            let work = get_progress_bar_text(i as usize, working_max as usize);
            let rest = get_progress_bar_text(0, resting_max as usize);
            show_progress_bars(work, rest)?;
            if i < working_max {
                sleep(Duration::from_secs(SECONDS_DELAY));
            }
        }

        show_footer("Time's up! Take a break! 🎉")?;

        for i in 0..=resting_max {
            let work = get_progress_bar_text(working_max as usize, working_max as usize);
            let rest = get_progress_bar_text(i as usize, resting_max as usize);
            show_progress_bars(work, rest)?;
            if i < resting_max {
                sleep(Duration::from_secs(SECONDS_DELAY));
            }
        }
    }
    show_footer("Session Finished!!")?;

    Ok(())
}
