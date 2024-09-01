use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    let Some(path) = args.path else {
        panic!("required path");
    };

    let markdown = std::fs::read_to_string(&path).unwrap();

    let markdown = themark_parser::parse(&markdown);

    themark_ui::setup(markdown);
}
