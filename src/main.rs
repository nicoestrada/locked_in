use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;
use std::fs::{File, remove_file};
use std::io::{BufRead, BufReader};

const ITEMS_FILE: &str = "focus_items.txt";

fn display_banner() {
    println!(r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⣤⣤⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿⠟⠉⠀⠀⠀⠈⠙⠿⣿⣿⣷⡄⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢰⣿⣿⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠙⣿⣿⣿⡀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣸⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⡇⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⡇⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⡇⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢠⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⡀⠀⠀⠀⠀
⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠉⠉⠛⣿⣿⣿⣿⣿⣿⣿⣿⣷⠀⠀⠀⠀
⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀
⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⡶⠀⠀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀
⠀⠀⠀⠀⢻⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⣿⠏⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠙⢿⣿⣿⣿⣿⣿⡏⠀⠀⠀⠀⢻⣿⣿⣿⣿⣿⡿⠃⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠈⠛⢿⣿⣿⣶⣶⣶⣶⣶⣾⣿⣿⠿⠛⠁⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠙⠛⠛⠉⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    "#);
}

fn load_focus_items() -> Vec<String> {
    match File::open(ITEMS_FILE) {
        Ok(file) => {
            let reader = BufReader::new(file);
            reader.lines().filter_map(Result::ok).take(3).collect()
        },
        Err(_) => Vec::new(),
    }
}

fn save_focus_items(items: &[String]) {
    let mut file = File::create(ITEMS_FILE).unwrap();
    for item in items {
        writeln!(file, "{}", item).unwrap();
    }
}

fn get_focus_items() -> Vec<String> {
    let mut items = load_focus_items();
    if items.len() < 3 {
        println!("Would you like to add focus items? (y/n)");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "y" {
            items.clear();
            for i in 1..=3 {
                println!("Enter focus item {}: ", i);
                let mut item = String::new();
                stdin().read_line(&mut item).unwrap();
                items.push(item.trim().to_string());
            }
            save_focus_items(&items);
        }
    }
    items
}

fn display_focus_items(items: &[String]) {
    println!("\nYour focus items:");
    for (i, item) in items.iter().enumerate() {
        println!("{}. {}", i + 1, item);
    }
    println!();
}

fn run_timer(duration: Duration) {
    let start_time = Local::now();
    let end_time = start_time + chrono::Duration::from_std(duration).unwrap();
    while Local::now() < end_time {
        let remaining = end_time - Local::now();
        let hours = remaining.num_hours();
        let minutes = remaining.num_minutes() % 60;
        let seconds = remaining.num_seconds() % 60;
        print!("\rYou're locked in for {:02}:{:02}:{:02}", hours, minutes, seconds);
        stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
    println!("\nTime's up! You're free!");
}

fn clear_focus_items() {
    if std::path::Path::new(ITEMS_FILE).exists() {
        remove_file(ITEMS_FILE).unwrap();
        println!("Focus items have been cleared.");
    } else {
        println!("No focus items to clear.");
    }
}

fn get_user_command() -> String {
    let mut command = String::new();
    stdin().read_line(&mut command).unwrap();
    command.trim().to_lowercase()
}

fn main() {
    display_banner();
    
    println!("Enter 'clear' to clear focus items, or press Enter to continue:");
    let command = get_user_command();
    
    if command == "clear" {
        clear_focus_items();
        return;
    }
    
    let focus_items = get_focus_items();
    display_focus_items(&focus_items);
    
    println!("Enter 'start' to begin the timer, or 'clear' to clear focus items:");
    loop {
        let command = get_user_command();
        match command.as_str() {
            "start" => break,
            "clear" => {
                clear_focus_items();
                return;
            }
            _ => println!("Invalid command. Enter 'start' or 'clear':"),
        }
    }
    
    let duration = Duration::from_secs(45 * 60); // 25 minutes
    run_timer(duration);
}
