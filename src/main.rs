mod args;
mod demultiplex;
mod demultiplexstruct;
mod dereport;
mod dereportcall;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::demultiplex::demultiplexreads;
use crate::dereport::dereportcallreads;
use crate::dereportcall::dereportcallfunction;
use clap::Parser;
use figlet_rs::FIGfont;

/*
Gaurav Sablok,
gsablok@proton.me
*/

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("REPGENERATE");
    println!("{}", repgenerate.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::MULTIPLEX {
            illuminadir,
            samplesheet,
        } => {
            let command = demultiplexreads(illuminadir, samplesheet).unwrap();
            println!("The command has been finished:{}", command);
        }
        Commands::DEREPORT {
            illuminadir,
            samplesheet,
        } => {
            let command = dereportcallreads(illuminadir, samplesheet).unwrap();
            println!("The command has been finished:{}", command);
        }
        Commands::REPORT { report } => {
            let command = dereportcallfunction(report).unwrap();
            println!("The report have been generated:{}", command);
        }
    }
}
