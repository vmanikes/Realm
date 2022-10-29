mod engine;
mod errors;
mod logo;

use clap::Parser;

// TODO Build in release mode
// TODO Build for different platforms

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct RealmArgs {
    #[arg(
        short,
        long,
        long_help = "path where realm should run",
        required = true
    )]
    directory: String, // TODO Default vault is current dir
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    logo::generate::print_logo();

    engine::parser::generate_files_from_templates(".");

    // let args = RealmArgs::parse();
}
