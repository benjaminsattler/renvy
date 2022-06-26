use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(value_parser)]
    settings: String,

    #[clap(value_parser)]
    defaults: String,

    #[clap(short, long)]
    cleanup: bool,
}

fn main() {
    let matches = Args::parse();

    let (settings, defaults) = match (
        renvy::read_file(&matches.settings),
        renvy::read_file(&matches.defaults),
    ) {
        (Ok(settings), Ok(defaults)) => (settings, defaults),
        (Ok(_), Err(_)) => panic!("Error reading defaults file"),
        (Err(_), Ok(_)) => panic!("Error reading settings file"),
        (Err(_), Err(_)) => panic!("Error reading input files"),
    };

    let (settings, defaults) = (renvy::deserialize(&settings), renvy::deserialize(&defaults));

    let merged = renvy::merge(settings, defaults, Some(matches.cleanup));

    let merged = renvy::serialize(&merged);

    renvy::write_file(&matches.settings, &merged).unwrap()
}
