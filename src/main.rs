//! # should-i
//!
//! A CLI tool to help you make decisions by consulting the yesno.wtf API.
//!
//! ## Usage
//!
//! ```bash
//! should-i "go to the gym"
//! should-i "eat pizza tonight" --open
//! ```

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;

/// Command-line arguments for the should-i tool
#[derive(Parser, Debug)]
#[command(name = "should-i")]
#[command(about = "Ask the universe for guidance on your decisions", long_about = None)]
struct Args {
    /// The question to ask (e.g., "go to the gym")
    #[arg(required = true)]
    question: Vec<String>,

    /// Open the GIF image in your browser
    #[arg(short, long)]
    open: bool,
}

/// Response from the yesno.wtf API
#[derive(Debug, Deserialize)]
struct YesNoResponse {
    /// The answer: "yes", "no", or "maybe"
    answer: String,
    /// Whether the answer was forced
    #[allow(dead_code)]
    forced: bool,
    /// URL to a GIF image representing the answer
    image: String,
}

/// Fetches a yes/no answer from the yesno.wtf API
///
/// # Errors
///
/// Returns an error if the HTTP request fails or the response cannot be parsed
async fn fetch_answer() -> Result<YesNoResponse> {
    let url = "https://yesno.wtf/api";
    let response = reqwest::get(url).await?.json::<YesNoResponse>().await?;
    Ok(response)
}

/// Displays the answer with appropriate emoji and formatting
fn display_answer(response: &YesNoResponse) {
    println!("\n🎲 Asking the universe...\n");

    let (emoji, message) = match response.answer.to_lowercase().as_str() {
        "yes" => ("✅", "YES! Do it! 🎉"),
        "no" => ("❌", "NO! Don't do it! 🚫"),
        "maybe" => ("🤔", "MAYBE... It's up to you! 🤷"),
        _ => ("❓", "UNKNOWN"),
    };

    println!("{} {} \n", emoji, message);
    println!("🖼️  {}\n", response.image);
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let _question = args.question.join(" ");

    let response = fetch_answer().await?;

    display_answer(&response);

    if args.open {
        println!("🌐 Opening in browser...\n");
        if let Err(e) = webbrowser::open(&response.image) {
            eprintln!("⚠️  Failed to open browser: {}", e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_matching() {
        let response = YesNoResponse {
            answer: "yes".to_string(),
            forced: false,
            image: "https://example.com/yes.gif".to_string(),
        };
        assert_eq!(response.answer, "yes");
    }

    #[test]
    fn test_answer_normalization() {
        let answers = vec!["yes", "no", "maybe"];
        for answer in answers {
            assert!(answer == "yes" || answer == "no" || answer == "maybe");
        }
    }
}
