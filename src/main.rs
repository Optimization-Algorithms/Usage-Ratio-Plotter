use std::path::PathBuf;

use structopt::StructOpt;

mod error;
mod log_file_loader;
mod plotter;

#[derive(StructOpt, Debug)]
struct Arguments {
    #[structopt(name = "input", help = "Specify usage ratio CSV file (from feasth)")]
    input_file: PathBuf,
    #[structopt(
        name = "output",
        help = "Specify output file. File format is understood using file extension, currently available: PNG, SVG"
    )]
    output_file: PathBuf,
    #[structopt(
        short = "-w",
        long = "--width",
        help = "Specify output image width",
        default_value = "640"
    )]
    width: u32,
    #[structopt(
        short = "-h",
        long = "--height",
        help = "Specify output image height",
        default_value = "480"
    )]
    height: u32,
    #[structopt(
        short = "-m",
        long = "--margin",
        help = "Specify plot margin",
        default_value = "15"
    )]
    margin: u32,
    #[structopt(short="-r", long = "--radius", help="Specify scatter radius", default_value="2")]
    radius: u32
}

fn build_config(args: &Arguments) -> plotter::Config {
    plotter::Config::new()
        .set_size(args.width, args.height)
        .set_margin(args.margin)
        .set_radius(args.radius)
}

fn run_plot(args: Arguments) -> Result<(), error::ProgramError> {
    let data = log_file_loader::load_status_file(&args.input_file)?;
    if data.len() > 0 {
        let config = build_config(&args);
        plotter::scatter_status(&data, &args.output_file, config)?;
    } else {
        println!(
            "WARNING: Given data log is empty: {}",
            args.input_file.to_str().unwrap()
        );
    }
    Ok(())
}

fn main() {
    let args = Arguments::from_args();

    match run_plot(args) {
        Ok(()) => {}
        Err(err) => println!("Error: {}", err),
    }
}
