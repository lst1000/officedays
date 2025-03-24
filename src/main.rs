use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde::Deserialize;
use chrono::{Datelike, NaiveDate};

#[derive(Debug, Deserialize)]
struct Config {
    required_quarterly_attendance: u8,
    bank_holidays: BankHolidays,
    leave: Leave,
    office_days: OfficeDays,
}

#[derive(Debug, Deserialize)]
struct BankHolidays {
    q1: Option<Vec<String>>,
    q2: Option<Vec<String>>,
    q3: Option<Vec<String>>,
    q4: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Leave {
    q1: Option<Vec<String>>,
    q2: Option<Vec<String>>,
    q3: Option<Vec<String>>,
    q4: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct OfficeDays {
    q1: Option<Vec<String>>,
    q2: Option<Vec<String>>,
    q3: Option<Vec<String>>,
    q4: Option<Vec<String>>,
}

impl BankHolidays {
    fn quarter_dates(&self, quarter: &str) -> Option<Vec<NaiveDate>> {
        match quarter {
            "q1" => self.q1.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            "q2" => self.q2.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            "q3" => self.q3.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            "q4" => self.q4.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            _ => None,
        }
    }
}

impl Leave {
    fn quarter_dates(&self, quarter: &str) -> Option<Vec<NaiveDate>> {
        match quarter {
            "q1" => self.q1.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            "q2" => self.q2.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            "q3" => self.q3.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            "q4" => self.q4.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            _ => None,
        }
    }
}

impl OfficeDays {
    fn quarter_dates(&self, quarter: &str) -> Option<Vec<NaiveDate>> {
        match quarter {
            "q1" => self.q1.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            "q2" => self.q2.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            "q3" => self.q3.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            "q4" => self.q4.as_ref()?.iter().map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()).collect(),
            _ => None,
        }
    }
}

fn print_help() {
    println!(
        "OfficeDays v0.2.3\nUsage: officedays [OPTIONS]

Options:
    -e                  Edit the configuration file for the current year
    -h                  Show this help message
    <no options>        Run the program with the configuration file for the current year

Configuration File:
    The program looks for the configuration file in:
        macOS: ~/Library/Application Support/officedays/<year>.toml
        Linux: ~/.config/officedays/<year>.toml"
    );
}

fn edit_config(config_path: &std::path::Path) {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    let status = Command::new(editor)
        .arg(config_path)
        .status()
        .expect("Failed to Open Editor");
    
    if !status.success() {
        eprintln!("Failed to Edit the Configuration File");
    }
}

fn config_search() -> PathBuf {
    let year = chrono::Local::now().year();
    let filename = format!("{}.toml", year);
    let config_dir = dirs::config_dir().expect("Configuration Directory Not Found");

    config_dir.join("officedays").join(filename)
}

fn current_quarter() -> String {
    let today = chrono::Local::now().date_naive();
    let month = today.month();

    match month {
        1..=3 => "q1".to_string(),
        4..=6 => "q2".to_string(),
        7..=9 => "q3".to_string(),
        10..=12 => "q4".to_string(),
        _ => unreachable!("Invalid Month"),
    }
}

fn days_required(config: &Config, quarter: &str) -> Result<u8, String> {
    let required_days = config.required_quarterly_attendance;

    let bank_holidays = match config.bank_holidays.quarter_dates(quarter) {
        Some(dates) => dates,
        None => vec![],
    };

    let leave_days = match config.leave.quarter_dates(quarter) {
        Some(dates) => dates,
        None => vec![],
    };

    let days_off = bank_holidays.len() as u8 + leave_days.len() as u8;

    Ok(required_days.saturating_sub(days_off))
}

fn days_worked(config: &Config, quarter: &str, today: Option<NaiveDate>) -> Result<u8, String> {
    let days_worked = match config.office_days.quarter_dates(quarter) {
        Some(dates) => {
            match today {
                Some(t) => dates.iter().filter(|&date| *date <= t).count() as u8,
                None => dates.len() as u8,
            }
        },
        None => 0
    };
    Ok(days_worked)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let valid_args = vec!["-e", "-h"];
    let config_path = config_search();

    if args.len() > 1 {
        let arg = &args[1];

        if arg == "-h" {
            print_help();
            return Ok(());
        } else if arg == "-e" {
            let config_path = config_search();

            if !config_path.exists() {
                eprintln!("Configuration File Not Found: {}", config_path.display());
                std::process::exit(1);
            }
            edit_config(&config_path);
            return Ok(());
        } else if !valid_args.contains(&arg.as_str()) {
            eprintln!("Error: Invalid argument '{}'", arg);
            eprintln!("Use '-h' to see available options.");
            std::process::exit(1);
        }
    }
    
    if !config_path.exists() {
        eprintln!("Configuration File Not Found: {}", config_path.display());
        std::process::exit(1);
    }

    let config_file = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_file)?;

    let today = chrono::Local::now().date_naive();
    let quarter = current_quarter();

    let days = days_required(&config, &quarter)?;
    let days_currently_worked = days_worked(&config, &quarter, Some(today))?;
    let days_scheduled = days_worked(&config, &quarter, None)?;
    let days_remaining = days as i8 - days_currently_worked as i8;
    let days_projected = days as i8 - days_scheduled as i8;

    println!("\x1b[1m{} Days Required in the Office\x1b[0m", quarter.to_uppercase());

    println!("\n{:<20} {:>4}", "Days Required", config.required_quarterly_attendance);
    println!("{:<20} {:>4}", "Leave Adjustment", config.required_quarterly_attendance - days);
    println!("{:<20} {:>4}", "Total Office Days", days);

    println!("\n{:<20} {:>4}", "Days Worked", days_currently_worked);
    if days_remaining > 0 {
        println!("\x1b[31m{:<20} {:>4}\x1b[0m", "Days Remaining", days_remaining);
    } else {
        println!("\x1b[32m{:<20} {:>4}\x1b[0m", "Days Remaining", days_remaining);
    }
    if days_projected > 0 {
        println!("\x1b[31m{:<20} {:>4}\x1b[0m", "Days Projected", days_projected);
        return Ok(());
    } else {
        println!("\x1b[32m{:<20} {:>4}\x1b[0m", "Days Projected", days_projected);
        return Ok(());
    }
}
