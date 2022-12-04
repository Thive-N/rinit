use clap::{Parser, ValueEnum};
use std::{
    env,
    fs::{self},
    path::PathBuf,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[arg(value_enum)]
    template: Template,

    #[arg(short, long)]
    directory: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[allow(non_camel_case_types)]
enum Template {
    python,
    latex_barebones,
    latex_fancy,
    c,
    cpp,
}

fn main() {
    let args = Args::parse();
    let mut path: PathBuf;
    match env::current_dir() {
        Ok(p) => {
            path = p;
        }
        Err(e) => {
            println!("unexpected error {}", e);
            return;
        }
    };

    if let Some(a) = args.directory {
        path = path.join(a);
    }

    println!("{:?}", path.to_str());

    if args.template == Template::latex_fancy {
        let filepath = path.join("main.tex");
        let filename = filepath.to_str().unwrap();

        fs::File::create(filename).expect("Unable to create file");
        let data = include_str!("../templates/latex/main_fancy.tex");
        fs::write(filename, data).expect("Unable to write file");
    }
}
