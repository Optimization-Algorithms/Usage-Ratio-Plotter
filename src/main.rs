use std::path::PathBuf;

use structopt::StructOpt;

mod error;
mod log_file_loader;
mod plotter;

#[derive(StructOpt, Debug)]
struct Arguments {
    #[structopt(name = "input", help="Specify usage ratio CSV file (from feasth)")]
    input_file: PathBuf,
    #[structopt(name = "output", help="Specify output file. File format is understood using file extension, currently available: PNG, SVG")]
    output_file: PathBuf,
}

fn run_plot(args: Arguments) -> Result<(), error::ProgramError> {
    let data = log_file_loader::load_status_file(&args.input_file)?;
    if data.len() > 0 {
        plotter::scatter_status(&data, &args.output_file)?;
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
