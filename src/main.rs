use clap::Parser;
use std::io::{self, stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossterm::terminal::{Clear, ClearType };
use crossterm::{QueueableCommand, cursor};

const WORK_DEFAULT: u8 = 25;
const REST_DEFAULT: u8 = 5;
const SESSIONS_DEFAULT: u8 = 1;

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
        Self {
            minutes,
            seconds,
        }
    }

    fn get_remaining_time(&self) -> String {
        let remaiming_time = format!("{:02}:{:02}", self.minutes, self.seconds);
        remaiming_time
    }

    fn subtract_second(&mut self) {
        if self.seconds > 0 {
            self.seconds -= 1;
        }
        else if self.minutes > 0 {
                self.seconds = 59;
                self.minutes -= 1;
        }
        else {
            self.minutes = 0;
                self.seconds = 0;
        }
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let work = args.work;
    let rest = args.rest;
    let sessions = args.sessions;
    let mut stdout = stdout();

    // Clear the screen and print at the top of the terminal
    stdout.queue(Clear(ClearType::All))?.queue(cursor::MoveTo(0, 0))?;
    stdout.flush()?;

    println!("Pomodoro session started!");

    let work_duration = Duration::from_secs(work as u64 * 60);
    let rest_duration = Duration::from_secs(rest as u64 * 60);
    let mut work_timer = Timer::new(work, 0);
    let mut rest_timer = Timer::new(rest, 0);
    for _ in 0..sessions {
        print!("{}", work_timer.get_remaining_time());
        let _ = std::io::stdout().flush();
        let w_time = Instant::now() + work_duration;
        while w_time > Instant::now() {
            let timer = w_time - Instant::now();
            work_timer.subtract_second();
            if timer.as_secs() % 5 == 0 {
                stdout.queue(Clear(ClearType::CurrentLine))?.queue(cursor::MoveTo(0, 1))?;
                stdout.flush()?;
                print!("{}", work_timer.get_remaining_time());
                let _ = std::io::stdout().flush();
            }
            sleep(Duration::new(1, 0));
        }
        let r_time = Instant::now() + rest_duration;
        println!("\n\nTime's up! Take a break! 🎉\n");
        stdout.queue(cursor::MoveTo(0, 5))?;
        stdout.flush()?;
        print!("{}", rest_timer.get_remaining_time());
        let _ = std::io::stdout().flush();
        while r_time > Instant::now() {
            let timer = r_time - Instant::now();
            rest_timer.subtract_second();
            if timer.as_secs() % 5 == 0 {
                stdout.queue(Clear(ClearType::CurrentLine))?.queue(cursor::MoveTo(0, 5))?;
                stdout.flush()?;
                print!("{}", rest_timer.get_remaining_time());
                let _ = std::io::stdout().flush();
            }
            sleep(Duration::new(1, 0));
        }
        println!("\n\nThe session is up!!!\n");
    }
    Ok(())
}
