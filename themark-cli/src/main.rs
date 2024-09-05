mod path_resolver;

use path_resolver::DocumentPath;
use themark_fs::{load_markdown, read_curr_dir, read_dir};

use clap::Parser;
use themark_ui::Entrypoint;

#[derive(Parser, Debug)]
#[command(version, about = "Render markdown files on your terminal", long_about = None)]
struct Args {
    path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let entrypoint = match args.path {
        Some(path) => match DocumentPath::try_from(path)? {
            DocumentPath::File(path) => Entrypoint::Viewer(load_markdown(path)?),
            DocumentPath::Dir(path) => Entrypoint::Dashboard(read_dir(path)?),
            //DocumentPath::Uri(uri) => load_markdown(fetch_markdown(uri)?)?,
        },
        None => Entrypoint::Dashboard(read_curr_dir()?),
    };

    themark_ui::setup(entrypoint)?;

    Ok(())
}
