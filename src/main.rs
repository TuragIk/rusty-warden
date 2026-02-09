use clap::Parser;
use std::path::Path;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(long)]
    json: bool,
}
fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    match rusty_warden::scan_directory(path) {
        Ok(findings) => {
            if args.json {
                let json_output = serde_json::to_string_pretty(&findings).unwrap();
                println!("{}", json_output);
            } else {
                for finding in findings {
                    println!("{}:{}: Found potential secret", finding.file, finding.line);
                    println!("Content: {}", finding.content);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
