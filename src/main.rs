extern crate structopt;
extern crate shiplift;

use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {

    // The number of occurences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
}

fn main() {
    let docker = shiplift::Docker::new();
    let images = docker.images();

    for i in images.list(&Default::default()).unwrap() {
        println!("-> {:?}", i);
    }
}
