use std::path::PathBuf;
use std::fs::File;
use std::io::{Error, Read, stdin};

use structopt::StructOpt;

mod error;
mod log_file_loader;
mod plotter;

#[derive(StructOpt, Debug)]
struct Arguments {
    #[structopt(name = "input", help = "Specify usage ratio CSV file (from feasth). By default read from STDIN", short="-i", long="--input")]
    input_file: Option<PathBuf>,
    #[structopt(
        name = "output",
        help = "Specify output file. File format is understood using file extension, currently available: PNG, SVG",
        short="-o", long="--output"
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


fn load_file_data(file: &mut dyn Read) -> Result<String, Error> {
    let mut output = String::new();
    file.read_to_string(&mut output)?;
    Ok(output)
}

fn load_csv_data(file_name: &Option<PathBuf>) -> Result<String, Error> {
    if let Some(file_name) = file_name {
        let mut file = File::open(file_name)?;
        load_file_data(&mut file)
    } else {
        let mut stdin = stdin();
        load_file_data(&mut stdin)
    }
}

fn print_warning(file_name: &Option<PathBuf>) {
    let name = if let Some(file_name) = file_name {
        if let Some(name) = file_name.to_str() {
            name
        } else {
            "**UNKONW FILE NAME**"
        }
    } else {
        "<STDIN>"
    };
    println!(
            "WARNING: Given data log is empty: {}",
            name
        );
}


fn run_plot(args: Arguments) -> Result<(), error::ProgramError> {
    let csv_string = load_csv_data(&args.input_file)?;
    let data = log_file_loader::parse_log_file(&csv_string)?;
    if data.len() > 0 {
        let config = build_config(&args);
        plotter::scatter_status(&data, &args.output_file, config)?;
    } else {
        print_warning(&args.input_file);
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
