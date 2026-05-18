use clap::Parser;
use std::process::{exit, Command};

#[derive(Parser, Debug)]
#[command(author, version, about = "Obsidian Thino CLI - Bolt speed!")]
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
    let memo = build_memo(&memo_text, args.tag.as_deref());

    let thino_args = build_thino_args(&args.vault, &memo);

    // Thino timestamps and formats the entry itself, so `obsidian` is invoked
    // directly instead of opening an obsidian:// URL. `status()` lets the CLI's
    // own messages reach the terminal and surfaces failures via the exit code.
    match Command::new("obsidian").args(&thino_args).status() {
        Ok(status) if status.success() => {}
        Ok(_) => exit(1),
        Err(e) => {
            eprintln!("Failed to run obsidian CLI: {}", e);
            exit(1);
        }
    }
}

fn build_memo(content: &str, tag: Option<&str>) -> String {
    match tag {
        Some(tag) => format!("{} #{}", content, tag),
        None => content.to_string(),
    }
}

fn build_thino_args(vault: &str, memo: &str) -> Vec<String> {
    vec![
        "thino".to_string(),
        "add".to_string(),
        format!("content={}", memo),
        format!("vault={}", vault),
        "type=task".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_memo_without_tag() {
        let memo = build_memo("Test memo", None);
        assert_eq!(memo, "Test memo");
    }

    #[test]
    fn test_build_memo_with_tag() {
        let memo = build_memo("Test memo", Some("tag1"));
        assert_eq!(memo, "Test memo #tag1");
    }

    #[test]
    fn test_build_thino_args() {
        let args = build_thino_args("MyVault", "Test memo #tag1");
        let expected: Vec<String> = [
            "thino",
            "add",
            "content=Test memo #tag1",
            "vault=MyVault",
            "type=task",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(args, expected);
    }
}
