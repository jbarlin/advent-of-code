#![feature(drain_filter)]
use color_eyre::eyre::Result;
use structopt::StructOpt;

mod day;
mod run;
use run::Run;

#[derive(StructOpt)]
#[structopt(name = "Advent Of Code")]
enum Args {
    Run(Run),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::from_args();
    match args {
        Args::Run(run) => {
            let output = run.run()?;
            println!("{}", output)
        }
    }
    Ok(())
}
