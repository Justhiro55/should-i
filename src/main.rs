use anyhow::Result;
use clap::Parser;
use serde::Deserialize;

/// Ask the universe a yes/no question
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

#[derive(Debug, Deserialize)]
struct YesNoResponse {
    answer: String,
    #[allow(dead_code)]
    forced: bool,
    image: String,
}

async fn fetch_answer() -> Result<YesNoResponse> {
    let url = "https://yesno.wtf/api";
    let response = reqwest::get(url).await?.json::<YesNoResponse>().await?;
    Ok(response)
}

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
