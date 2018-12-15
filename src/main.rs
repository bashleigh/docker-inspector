extern crate structopt;
extern crate shiplift;
extern crate tui;
extern crate termion;

use structopt::StructOpt;
use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
}

fn main() {
    let docker = shiplift::Docker::new();
    let containers = docker.containers();

    //println!("{%d} containers ", containers.len());

    for container in containers.list(&Default::default()).unwrap() {
        println!("-> {:?}", container);
    }
}
