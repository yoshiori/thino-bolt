use chrono::{DateTime, Local};
use clap::Parser;
use std::process::Command;
use urlencoding::encode;

#[derive(Parser, Debug)]
#[command(author, version, about = "Obsidan Thino CLI - Bolt speed!")]
struct Args {
    vault: String,

    #[arg(required = true)]
    content: Vec<String>,

    #[arg(short, long)]
    tag: Option<String>,
}

fn main() {
    let args = Args::parse();
    let memo_text = args.content.join(" ");
    let full_memo = build_memo(&memo_text, args.tag.as_deref(), Local::now());

    let obsidian_url = build_obsidian_url(&args.vault, &full_memo);

    if let Err(e) = Command::new("xdg-open").arg(obsidian_url).spawn() {
        eprintln!("Failed to open Obsidian: {}", e);
    }
}

fn build_memo(content: &str, tag: Option<&str>, time: DateTime<Local>) -> String {
    let time_str = time.format("%H:%M").to_string();
    let tag_part = match tag {
        Some(tag) => format!(" #{}", tag),
        None => String::new(),
    };
    format!("- [ ] {} {}{}", time_str, content, tag_part)
}

fn build_obsidian_url(vault: &str, memo: &str) -> String {
    let encoded_data = encode(memo);
    format!(
        "obsidian://advanced-uri?vault={}&daily=true&data={}&mode=append",
        vault, encoded_data
    )
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_build_memo_without_tag() {
        let time = Local.with_ymd_and_hms(2024, 6, 1, 12, 0, 0).unwrap();
        let memo = build_memo("Test memo", None, time);
        assert_eq!(memo, "- [ ] 12:00 Test memo");
    }

    #[test]
    fn test_build_memo_with_tag() {
        let time = Local.with_ymd_and_hms(2024, 6, 1, 12, 0, 0).unwrap();
        let memo = build_memo("Test memo", Some("tag1"), time);
        assert_eq!(memo, "- [ ] 12:00 Test memo #tag1");
    }

    #[test]
    fn test_build_obsidian_url() {
        let memo = "- [ ] 12:00 Test memo #tag1";
        let result = build_obsidian_url("MyVault", memo);
        assert_eq!(
            result,
            "obsidian://advanced-uri?vault=MyVault&daily=true&data=-%20%5B%20%5D%2012%3A00%20Test%20memo%20%23tag1&mode=append"
        );
    }
}
