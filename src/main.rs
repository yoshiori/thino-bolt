use chrono::Local;
use clap::Parser;
use std::process::Command;
use urlencoding::encode;

#[derive(Parser, Debug)]
#[command(author, version, about = "Obsidan Thino CLI - Bolt speed!")]
struct Args {
    valut: String,

    #[arg(required = true)]
    content: Vec<String>,

    #[arg(short, long)]
    tag: Option<String>,
}

fn main() {
    let args = Args::parse();
    let memo_text = args.content.join(" ");
    let now = Local::now().format("%H:%M").to_string();

    let tag_part = match &args.tag {
        Some(tag) => format!(" #{}", tag),
        None => String::new(),
    };

    let full_memo = format!("- [ ] {} {}{}", now, memo_text, tag_part);

    let encoded_data = encode(&full_memo);
    let obsidian_url = format!(
        "obsidian://advanced-uri?vault={}&daily=true&data={}&mode=append",
        args.valut, encoded_data
    );

    if let Err(e) = Command::new("xdg-open").arg(obsidian_url).spawn() {
        eprintln!("Failed to open Obsidian: {}", e);
    }
}
