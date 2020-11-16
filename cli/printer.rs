use std::io;

use console::Color;

pub fn print_info(msg: &str) {
    let badge = console::style(" INFO ")
        .fg(Color::Color256(255))
        .bg(Color::Color256(24));

    println!("\n{} {}", badge, msg);
}

pub fn print_warning(msg: &str) {
    let badge = console::style(" WARNING ")
        .fg(Color::Color256(94))
        .bg(Color::Yellow);

    eprintln!("\n{} {}", badge, msg);
}

pub fn print_error(error: io::Error) {
    let badge = console::style(" ERROR ")
        .fg(Color::Color256(255))
        .bg(Color::Red);
    let msg = console::style(error).red().bold();

    eprintln!("\n{} {}", badge, msg);
}

pub fn print_non_zero_exit_code(code: i32) {
    let badge = console::style(" ERROR ")
        .fg(Color::Color256(255))
        .bg(Color::Red);
    let msg = console::style(format!("Exit code: {}", code)).red().bold();

    eprintln!("\n{} {}", badge, msg);
}

pub fn print_done() {
    println!("\n✨ Done.");
}
