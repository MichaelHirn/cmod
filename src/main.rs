use clap::Parser;

mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the codebase
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();

    let path = std::env::current_dir().unwrap().join(&args.path);
    println!("Parsing dir: {}", path.display());

    let mut parser = parser::Parser::new();
    let _files = parser.get_target_files(&path);
}
