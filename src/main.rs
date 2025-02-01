use clap::Parser;
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, Instant};

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

fn main() {
    let args = Args::parse();
    let work = args.work;
    let rest = args.rest;
    let sessions = args.sessions;

    println!("Pomodoro session started!");

    let work_duration = Duration::from_secs(work as u64 * 60);
    let rest_duration = Duration::from_secs(rest as u64 * 60);
    for _ in 0..sessions {
        let work_timer = Instant::now() + work_duration;
        while work_timer > Instant::now() {
            let timer = work_timer - Instant::now();
            if timer.as_secs() % 10 == 0 {
                print!(">");
                let _ = std::io::stdout().flush();
            }
            sleep(Duration::new(1, 0));
        }
        let rest_timer = Instant::now() + rest_duration;
        println!("\n\nTime's up! Take a break! 🎉\n");
        while rest_timer > Instant::now() {
            let timer = rest_timer - Instant::now();
            if timer.as_secs() % 10 == 0 {
                print!("{}", timer.as_secs());
                let _ = std::io::stdout().flush();
            }
            sleep(Duration::new(1, 0));
        }
        println!("\n\nThe session is up!!!\n");
    }
}
