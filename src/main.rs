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

/// API レスポンスの構造体
#[derive(Debug, Deserialize)]
struct YesNoResponse {
    answer: String,
    forced: bool,
    image: String,
}

/// yesno.wtf API にリクエストを送信
async fn fetch_answer() -> Result<YesNoResponse> {
    let url = "https://yesno.wtf/api";
    let response = reqwest::get(url).await?.json::<YesNoResponse>().await?;
    Ok(response)
}

/// 回答を整形して表示
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

    // 質問を結合して表示（使わないけど、ユーザーの質問を受け取ったことを示す）
    let _question = args.question.join(" ");

    // API にリクエスト
    let response = fetch_answer().await?;

    // 結果を表示
    display_answer(&response);

    // --open オプションが指定されていたらブラウザで開く
    if args.open {
        println!("🌐 Opening in browser...\n");
        if let Err(e) = webbrowser::open(&response.image) {
            eprintln!("⚠️  Failed to open browser: {}", e);
        }
    }

    Ok(())
}
