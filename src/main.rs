use clap::Parser;
use crossterm::style::{Color, SetForegroundColor, Stylize};
use crossterm::{cursor, execute, style::Print, terminal};
use std::io::{self, stdout};
use std::thread::sleep;
use std::time::Duration;

const WORK_DEFAULT: u8 = 25;
const REST_DEFAULT: u8 = 5;
const SESSIONS_DEFAULT: u8 = 1;
const PROGRESS_UNIT: &str = "█";
const SECONDS_DELAY: u64 = 1;

/// Pomodoro Client Application
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// working minutes
    #[arg(short, long, default_value_t = WORK_DEFAULT)]
    work: u8,

    /// resting minutes
    #[arg(short, long, default_value_t = REST_DEFAULT)]
    rest: u8,

    /// number of sessions
    #[arg(short, long, default_value_t = SESSIONS_DEFAULT)]
    sessions: u8,
}

struct Timer {
    minutes: u8,
    seconds: u8,
}

impl Timer {
    fn new(minutes: u8, seconds: u8) -> Self {
        Self { minutes, seconds }
    }

    fn get_remaining_time(&self) -> String {
        let remaiming_time = format!("{:02}:{:02}", self.minutes, self.seconds);
        remaiming_time
    }

    fn subtract_second(&mut self) {
        if self.seconds > 0 {
            self.seconds -= 1;
        } else if self.minutes > 0 {
            self.seconds = 59;
            self.minutes -= 1;
        } else {
            self.minutes = 0;
            self.seconds = 0;
        }
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let working_max = args.work;
    let resting_max = args.rest;
    let max_sessions = args.sessions;
    let current_session = 1;

    let sessions_text = format!("Session {} of {}", current_session, max_sessions);
    // run a sequence of instructions on the standard output
    execute!(stdout(), terminal::Clear(terminal::ClearType::All),)?;

    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        cursor::MoveTo(18, 0),
        Print("Pomodoro Timer".underlined().bold()),
        SetForegroundColor(Color::Blue),
        cursor::MoveTo(18, 2),
        Print(sessions_text),
    )?;

    let work_duration = Duration::from_secs(working_max as u64 * 60);
    let rest_duration = Duration::from_secs(resting_max as u64 * 60);
    let mut work_timer = Timer::new(working_max, 0);
    let mut rest_timer = Timer::new(resting_max, 0);
    for _ in 0..max_sessions {
        for i in 0..=working_max {
            // Using the repeat method, we can print a specific number of characters to the stdout
            let work = format!(
                "[{}{}]   {} of {} minutes left. ",
                PROGRESS_UNIT.repeat(i as usize),
                " ".repeat((working_max - i) as usize),
                working_max - i,
                working_max
            );
            let rest = format!(
                "[{}{}]   {} of {} minutes left. ",
                PROGRESS_UNIT.repeat(0),
                " ".repeat(resting_max as usize),
                resting_max,
                resting_max
            );

            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                cursor::MoveTo(0, 4),
                Print(work.to_string()),
                SetForegroundColor(Color::Red),
                cursor::MoveTo(0, 5),
                Print(rest.to_string()),
                Print("\n\n"),
            )?;

            if i < working_max {
                sleep(Duration::from_secs(SECONDS_DELAY));
            }
        }

        for i in 0..=resting_max {
            // Using the repeat method, we can print a specific number of characters to the stdout
            let work = format!(
                "[{}{}]   {} of {} minutes left. ",
                PROGRESS_UNIT.repeat(working_max as usize),
                " ".repeat(0),
                0,
                working_max
            );
            let rest = format!(
                "[{}{}]   {} of {} minutes left. ",
                PROGRESS_UNIT.repeat(i as usize),
                " ".repeat((resting_max - i) as usize),
                resting_max - i,
                resting_max
            );

            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                cursor::MoveTo(0, 4),
                Print(work.to_string()),
                SetForegroundColor(Color::Red),
                cursor::MoveTo(0, 5),
                Print(rest.to_string()),
                Print("\n\n"),
            )?;

            if i < resting_max {
                sleep(Duration::from_secs(SECONDS_DELAY));
            }
        }
        execute!(
            stdout(),
            SetForegroundColor(Color::Blue),
            SetForegroundColor(Color::Blue),
            cursor::MoveTo(18, 7),
            Print("Session finished!!".to_string()),
            Print("\n\n"),
        )?;
    }

    Ok(())
}
